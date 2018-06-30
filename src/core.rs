//! Application and window management.

use super::glutin::*;
use super::glutin::Window as GlutinWindow;
use std::ops::Deref;

/// A context in which you can get input, draw and play audio.
pub struct Window {
	window: GlutinWindow
}

impl Window {
	/// Gets an immutable reference to the internal window.
	/// Due to the design of the underlying API (Glutin),
	/// this internal window can only be immutable and
	/// only needs to be immutable.
	pub fn window(&self) -> &GlutinWindow {
		&self.window
	}
}

impl Deref for Window {
	type Target = GlutinWindow;
	fn deref(&self) -> &GlutinWindow {
		&self.window
	}
}