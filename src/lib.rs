pub mod core;
pub mod input;

/// A simple struct that represents the size of an object.
#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct Size {
	/// The width of the object.
	pub width: f64,

	/// The height of the object.
	pub height: f64
}

/// A simple struct that represents a point or position.
#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct Point {
	/// The x-coordinate of the point.
	pub x: f64,

	/// The y-coordinate of the point.
	pub y: f64
}