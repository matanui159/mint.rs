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

use ::{Size, Point};

use std::f64::consts::PI;

const RAD_PER_DEG: f64 = PI / 180.0;

/// An RGBA color of something.
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Color {
	/// The red channel.
	/// Should be between 0 and 1.
	pub red: f64,

	/// The green channel.
	/// Should be between 0 and 1.
	pub green: f64,

	/// The blue channel.
	/// Should be between 0 and 1.
	pub blue: f64,

	/// The alpha channel.
	/// Should be between 0 and 1.
	pub alpha: f64
}

impl Color {
	/// A utility method that makes it neater to create a color.
	pub fn new(red: f64, green: f64, blue: f64, alpha: f64) -> Color {
		Color {red, green, blue, alpha}
	}
}

/// An angle of something, either in degrees or radians.
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Angle(f64);

impl Angle {
	/// Creates an angle from radians.
	pub fn from_radians(rad: f64) -> Angle {
		Angle(rad)
	}

	/// Creates an angle from degrees.
	pub fn from_degrees(deg: f64) -> Angle {
		Angle(deg * RAD_PER_DEG)
	}

	/// Converts an angle to radians.
	pub fn as_radians(&self) -> f64 {
		self.0
	}

	/// Converts an angle to degrees.
	pub fn as_degrees(&self) -> f64 {
		self.0 / RAD_PER_DEG
	}
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct State {
	color: Color,
	t0: (f64, f64, f64),
	t1: (f64, f64, f64)
}

impl State {
	pub fn color(&mut self, color: Color) {
		self.color = color
	}

	pub fn tint(&mut self, color: Color) {
		self.color.red   *= color.red;
		self.color.green *= color.green;
		self.color.blue  *= color.blue;
		self.color.alpha *= color.alpha;
	}

	pub fn get_color(&self) -> Color {
		self.color
	}

	pub fn identity(&mut self) {
		self.t0 = (1.0, 0.0, 0.0);
		self.t1 = (0.0, 1.0, 0.0);
	}

	pub fn translate(&mut self, offset: Point) {
		let offset = self.transform(offset);
		self.t0.2 = offset.x;
		self.t1.2 = offset.y;
	}

	pub fn scale(&mut self, size: Size) {
		self.t0.0 *= size.width;
		self.t0.1 *= size.height;
		self.t1.0 *= size.width;
		self.t1.1 *= size.height;
	}

	pub fn rotate(&mut self, angle: Angle) {
		let c = angle.as_radians().cos();
		let s = angle.as_radians().sin();
		self.t0.0 = self.t0.0 * c + self.t0.1 * s;
		self.t0.1 = self.t0.1 * c - self.t0.0 * s;
		self.t1.0 = self.t1.0 * c + self.t1.1 * s;
		self.t1.1 = self.t1.1 * c - self.t1.0 * s;
	}

	pub fn transform(&self, point: Point) -> Point {
		Point {
			x: self.t0.0 * point.x + self.t0.1 * point.y + self.t0.2,
			y: self.t1.0 * point.x + self.t1.1 * point.y + self.t1.2
		}
	}
}

impl Default for State {
	fn default() -> State {
		State {
			color: Color {
				red:   1.0,
				green: 1.0,
				blue:  1.0,
				alpha: 1.0
			},
			t0: (1.0, 0.0, 0.0),
			t1: (0.0, 1.0, 0.0)
		}
	}
}