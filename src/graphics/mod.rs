// Copyright 2018 Joshua Minter
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Manages all the drawing and graphics.

extern crate glutin;
use self::glutin::{GlWindow, GlContext};

extern crate backtrace;
use self::backtrace::Backtrace;

mod gl;
use self::gl::{Gl, CheckError};
use self::gl::types::*;

mod state;
pub use self::state::{Color, Angle};
use self::state::State;

use ::{Size, Point};
use ::core::WindowError;

use std::rc::Rc;
use std::cell::RefCell;
use std::mem;
use std::ptr;

const BUFFER_SIZE: usize = 4;

/// Possible errors that can occur from push/pop operations.
#[derive(Clone, Debug)]
pub enum StackError {
	/// There was nothing on the stack when
	/// [`Graphics.pop`](struct.Graphics.html#method.pop)
	/// was called.
	StackUnderflow(Backtrace)
}

#[derive(Copy, Clone, PartialEq, Debug)]
struct Point32 {
	x: f32,
	y: f32
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct Color8 {
	red:   u8,
	green: u8,
	blue:  u8,
	alpha: u8
}

#[derive(Copy, Clone, PartialEq, Debug)]
struct Vertex {
	point: Point32,
	texcoord: Point32,
	color: Color8
}

struct GraphicsImpl {
	window: Rc<GlWindow>,
	gl: Gl,
	state: Vec<State>,
	vertex_array: GLuint,
	elements: GLuint,
	buffer: GLuint,
	buffer_data: Vec<Vertex>
}

impl GraphicsImpl {
	fn new(window: Rc<GlWindow>) -> GraphicsImpl {
		unsafe {
			let gl = Gl::load_with(|name| window.get_proc_address(name) as *const _);

			let mut vertex_array = 0;
			gl.GenVertexArrays(1, &mut vertex_array);
			gl.BindVertexArray(vertex_array);

			let mut elements = 0;
			gl.GenBuffers(1, &mut elements);
			gl.BindBuffer(gl::ELEMENT_ARRAY_BUFFER, elements);

			let elements_data = [0u8, 1u8, 2u8, 2u8, 1u8, 3u8];
			gl.BufferData(
				gl::ELEMENT_ARRAY_BUFFER,
				elements_data.len() as GLsizeiptr,
				elements_data.as_ptr() as *const _,
				gl::STATIC_DRAW
			);

			let mut buffer = 0;
			gl.GenBuffers(1, &mut buffer);
			gl.BindBuffer(gl::ARRAY_BUFFER, buffer);

			gl.EnableVertexAttribArray(0);
			gl.EnableVertexAttribArray(1);
			gl.EnableVertexAttribArray(2);

			let stride = mem::size_of::<Vertex>() as GLsizei;
			gl.VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, stride, offset_of!(Vertex, point) as *mut _);
			gl.VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, stride, offset_of!(Vertex, texcoord) as *mut _);
			gl.VertexAttribPointer(2, 4, gl::UNSIGNED_BYTE, gl::TRUE, stride, offset_of!(Vertex, color) as *mut _);

			gl.check_error();
			GraphicsImpl {
				window,
				gl,
				state: vec![State::default()],
				vertex_array,
				elements,
				buffer,
				buffer_data: Vec::with_capacity(BUFFER_SIZE)
			}
		}
	}

	fn vertex(&mut self, point: Point, texcoord: Point) {
		let point = self.state.last().unwrap().transform(point);
		let color = self.state.last().unwrap().get_color();
		self.buffer_data.push(Vertex {
			point: Point32 {
				x: point.x as f32,
				y: point.y as f32
			},
			texcoord: Point32 {
				x: texcoord.x as f32,
				y: texcoord.y as f32
			},
			color: Color8 {
				red:   (color.red   * 255.0) as u8,
				green: (color.green * 255.0) as u8,
				blue:  (color.blue  * 255.0) as u8,
				alpha: (color.alpha * 255.0) as u8
			}
		});

		if self.buffer_data.len() == BUFFER_SIZE {
			self.flush();
		}
	}

	fn flush(&mut self) {
		unsafe {
			if self.buffer_data.len() > 0 {
				self.gl.BufferData(
					gl::ARRAY_BUFFER,
					(self.buffer_data.len() * mem::size_of::<Vertex>()) as GLsizeiptr,
					self.buffer_data.as_ptr() as *const _,
					gl::STREAM_DRAW
				);
				// TODO: fix element buffer size
				self.gl.DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_BYTE, ptr::null());
				self.gl.check_error();
				self.buffer_data.clear();
			}
		}
	}
}

impl Drop for GraphicsImpl {
	fn drop(&mut self) {
		unsafe {
			self.gl.DeleteBuffers(1, &self.buffer);
			self.gl.DeleteBuffers(1, &self.elements);
			self.gl.DeleteVertexArrays(1, &self.vertex_array);
		}
	}
}

type RcGraphics = Rc<RefCell<GraphicsImpl>>;

/// Contains all graphics related methods and data.
pub struct Graphics {
	rc: RcGraphics
}

impl Graphics {
	pub(crate) fn new(window: Rc<GlWindow>) -> Graphics {
		Graphics {
			rc: Rc::new(RefCell::new(GraphicsImpl::new(window)))
		}
	}

//	fn clone_rc(&self) -> RcGraphics {
//		Rc::clone(&self.rc)
//	}

	pub(crate) fn update(&self) -> Result<(), WindowError> {
		let mut graphics = self.rc.borrow_mut();
		graphics.flush();
		graphics.window.swap_buffers()
			.map_err(|error| WindowError::InternalError(
				ToString::to_string(&error),
				Backtrace::new()
			))?;
		Ok(())
	}

	/// Pushes the current rendering state.
	/// Call [`pop`](#method.pop) to undo any changes since the last push.
	pub fn push(&mut self) {
		let mut graphics = self.rc.borrow_mut();
		let state = *graphics.state.last().unwrap();
		graphics.state.push(state);
	}

	/// Pops the current rendering state.
	/// # Errors
	/// Returns [`StackError::StackUnderflow`](enum.StackError.html#variant.StackUnderflow)
	/// if the there is nothing to pop.
	pub fn pop(&mut self) -> Result<(), StackError> {
		let mut graphics = self.rc.borrow_mut();
		if graphics.state.len() <= 1 {
			Err(StackError::StackUnderflow(Backtrace::new()))
		} else {
			graphics.state.pop();
			Ok(())
		}
	}

	/// Sets the current color.
	pub fn color(&mut self, color: Color) {
		self.rc.borrow_mut().state.last_mut().unwrap().color(color)
	}

	/// Tints the current color.
	/// This is done by multiplying the current color with the tint.
	pub fn tint(&mut self, color: Color) {
		self.rc.borrow_mut().state.last_mut().unwrap().tint(color);
	}

	/// Resets the transform to identity.
	pub fn identity(&mut self) {
		self.rc.borrow_mut().state.last_mut().unwrap().identity();
	}

	/// Translates the current transform.
	pub fn translate(&mut self, offset: Point) {
		self.rc.borrow_mut().state.last_mut().unwrap().translate(offset);
	}

	/// Scales the current transform.
	pub fn scale(&mut self, size: Size) {
		self.rc.borrow_mut().state.last_mut().unwrap().scale(size);
	}

	/// Rotates the current transform.
	pub fn rotate(&mut self, angle: Angle) {
		self.rc.borrow_mut().state.last_mut().unwrap().rotate(angle);
	}

	/// Transforms the given point with the current transform.
	pub fn transform(&mut self, point: Point) -> Point {
		self.rc.borrow_mut().state.last_mut().unwrap().transform(point)
	}

	/// Clears the screen with the current color.
	pub fn clear(&self) {
		unsafe {
			let graphics = self.rc.borrow();
			let color = graphics.state.last().unwrap().get_color();
			graphics.gl.ClearColor(
				color.red   as f32,
				color.green as f32,
				color.blue  as f32,
				color.alpha as f32
			);
			graphics.gl.Clear(gl::COLOR_BUFFER_BIT);
			graphics.gl.check_error();
		}
	}
}