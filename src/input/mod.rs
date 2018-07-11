//! Manages keyboard and mouse input
//! and the cursor.

extern crate glutin;
pub use self::glutin::{VirtualKeyCode as Key, MouseButton as Button};
use self::glutin::dpi::LogicalPosition;

extern crate backtrace;
use self::backtrace::Backtrace;

use ::Point;
use ::core::RcWindow;

use std::fmt::{Display, Formatter, Error};
use std::collections::HashSet;

/// Possible errors that can occur from input related actions.
#[derive(Clone, Debug)]
pub enum InputError {
	/// An unknown internal error occurred.
	InternalError(String, Backtrace)
}

impl Display for InputError {
	fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
		match self {
			&InputError::InternalError(ref error, ..) => write!(f, "{}", error)
		}
	}
}

/// Contains all input related methods and data.
pub struct Input {
	window: RcWindow,
	pub(crate) keys: HashSet<Key>,
	pub(crate) buttons: HashSet<Button>,
	pub(crate) cursor: Point
}

impl Input {
	pub(crate) fn new(window: RcWindow) -> Input {
		Input {
			window,
			keys: HashSet::new(),
			buttons: HashSet::new(),
			cursor: Point::default()
		}
	}

	/// Gets the current state of the keyboard key.
	/// Returns true if the key is pressed,
	/// false otherwise.
	pub fn get_key_state(&self, key: Key) -> bool {
		self.keys.contains(&key)
	}

	/// Gets the current state of the mouse button.
	/// Returns true if the key is pressed,
	/// false otherwise.
	pub fn get_button_state(&self, button: Button) -> bool {
		self.buttons.contains(&button)
	}

	/// Gets the current position of the cursor.
	pub fn get_cursor_point(&self) -> Point {
		self.cursor
	}

	/// Sets the current position of the cursor.
	pub fn set_cursor_point(&mut self, point: Point) -> Result<(), InputError> {
		self.window.borrow().window.set_cursor_position(LogicalPosition {
			x: point.x,
			y: point.y
		}).map_err(|error| InputError::InternalError(
			error,
			Backtrace::new()
		))?;
		self.cursor = point;
		Ok(())
	}

	/// Sets the cursor as hidden
	/// or visible.
	pub fn set_cursor_hidden(&mut self, hidden: bool) {
		self.window.borrow().window.hide_cursor(hidden);
	}
}