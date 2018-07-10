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

extern crate mint2d;

use mint2d::core::{Window, Config};
use mint2d::graphics::Color;

fn main() {
	let mut window = Window::new(Config::default()).unwrap();
	window.graphics().color(Color::new(1.0, 0.0, 0.0, 1.0));

	while window.update().unwrap() {
		window.graphics().clear();
	}
}