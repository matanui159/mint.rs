extern crate glutin;
pub use self::glutin::VirtualKeyCode as Key;

use std::collections::HashMap;

use ::Point;

#[derive(Copy, Clone, PartialEq, Debug)]
pub(crate) struct Cursor {
	pub point: Point,
	pub hidden: bool
}

#[derive(Clone, Debug)]
pub struct Input {
	pub(crate) keys: HashMap<Key, bool>,
	pub(crate) cursor: Cursor
}

impl Input {
	pub fn get_key_state(&self, key: Key) -> bool {
		*self.keys.get(&key).unwrap_or(&false)
	}

	pub fn get_cursor_point(&self) -> Point {
		self.cursor.point
	}

	pub fn set_cursor_point(&mut self, point: Point) {
		self.cursor.point = point
	}

	pub fn set_cursor_hidden(&mut self, hidden: bool) {
		self.cursor.hidden = hidden
	}
}