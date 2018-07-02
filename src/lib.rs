pub mod core;

/// A simple struct that represents the size of an object.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Size {
	pub width: f64,
	pub height: f64
}

/// A simple struct that represents a point or position.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Point {
	pub x: f64,
	pub y: f64
}