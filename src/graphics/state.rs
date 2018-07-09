use ::{Size, Point};

use std::f64::consts::PI;

const RAD_PER_DEG: f64 = PI / 180.0;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Color {
	pub red: f64,
	pub green: f64,
	pub blue: f64,
	pub alpha: f64
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Angle(f64);

impl Angle {
	pub fn from_radians(rad: f64) -> Angle {
		Angle(rad)
	}

	pub fn from_degrees(deg: f64) -> Angle {
		Angle(deg * RAD_PER_DEG)
	}

	pub fn as_radians(&self) -> f64 {
		self.0
	}

	pub fn as_degrees(&self) -> f64 {
		self.0 / RAD_PER_DEG
	}
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Transform((f64, f64, f64), (f64, f64, f64));

impl Transform {
	pub fn identity(&mut self) {
		*self = Transform::default()
	}

	pub fn translate(&mut self, offset: Point) {

	}

	pub fn rotate(&mut self, angle: Angle) {

	}

	pub fn scale(&mut self, size: Size) {

	}

	pub fn transform(&mut self, point: Point) -> Point {
		let x = self.0;
		let y = self.0.0; // TODO figure this out
		Point {
			x: self.0 .0 * point.x
		}
	}
}

impl Default for Transform {
	fn default() -> Transform {
		Transform(
			(1.0, 0.0, 0.0),
			(0.0, 1.0, 0.0)
		)
	}
}

pub struct State {
	pub color: Color,
	pub transform: Transform
}