//! Manages all the drawing and graphics.

extern crate glutin;
use self::glutin::{GlWindow, GlContext};

extern crate backtrace;
use self::backtrace::Backtrace;

mod gl;
use self::gl::{Gl, CheckError};

mod state;
pub use self::state::{Color, Angle};
use self::state::State;

use ::{Size, Point};
use ::core::WindowError;

use std::rc::Rc;
use std::cell::RefCell;

/// Possible errors that can occur from push/pop operations.
pub enum StackError {
	/// There was nothing on the stack when
	/// [`Graphics.pop`](struct.Graphics.html#method.pop)
	/// was called.
	StackUnderflow(Backtrace)
}

struct GraphicsImpl {
	window: Rc<GlWindow>,
	gl: Gl,
	state: Vec<State>
}

type RcGraphics = Rc<RefCell<GraphicsImpl>>;

/// Contains all graphics related methods and data.
pub struct Graphics {
	rc: RcGraphics
}

impl Graphics {
	pub(crate) fn new(window: Rc<GlWindow>) -> Graphics {
		let gl = Gl::load_with(|name| window.get_proc_address(name) as *const _);
		Graphics {
			rc: Rc::new(RefCell::new(GraphicsImpl {
				window,
				gl,
				state: vec![State::default()]
			}))
		}
	}

//	fn clone_rc(&self) -> RcGraphics {
//		Rc::clone(&self.rc)
//	}

	pub(crate) fn update(&self) -> Result<(), WindowError> {
		let graphics = self.rc.borrow();
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