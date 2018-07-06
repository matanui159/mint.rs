extern crate glutin;
use self::glutin::{GlWindow, GlContext};

extern crate backtrace;
use self::backtrace::Backtrace;

mod gl;
//use self::gl::types::*;
use self::gl::{Gl, CheckError};

use ::core::WindowError;

use std::rc::Rc;
use std::cell::RefCell;

struct GraphicsImpl {
	window: Rc<GlWindow>,
	gl: Gl
}

type RcGraphics = Rc<RefCell<GraphicsImpl>>;

pub struct Graphics {
	rc: RcGraphics
}

impl Graphics {
	pub(crate) fn new(window: Rc<GlWindow>) -> Graphics {
		let gl = Gl::load_with(|name| window.get_proc_address(name) as *const _);
		Graphics {
			rc: Rc::new(RefCell::new(GraphicsImpl {
				window,
				gl
			}))
		}
	}

//	fn clone_rc(&self) -> RcGraphics {
//		Rc::clone(&self.rc)
//	}

	pub(crate) fn update(&self) -> Result<(), WindowError> {
		unsafe {
			let graphics = self.rc.borrow();
			graphics.window.swap_buffers()
				.map_err(|error| WindowError::InternalError(ToString::to_string(&error), Backtrace::new()))?;

			graphics.gl.ClearColor(1.0, 0.0, 0.0, 1.0);
			graphics.gl.Clear(gl::COLOR_BUFFER_BIT);
			graphics.gl.check_error();
			Ok(())
		}
	}
}