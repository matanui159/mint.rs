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
use mint2d::input::Key;

fn main() {
	let mut window = Window::new(Config::default()).unwrap();

	while window.update().unwrap() {
		let hidden = window.input().get_key_state(Key::Space);
		window.input().set_cursor_hidden(hidden);
	}
}