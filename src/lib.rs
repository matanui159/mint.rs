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

#[macro_use]
extern crate memoffset;

pub mod core;
pub mod input;
pub mod graphics;

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