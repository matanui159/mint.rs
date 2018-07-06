extern crate mint2d;

use mint2d::core::{Window, Config};
use mint2d::input::Key;

fn main() {
	let mut window = Window::new(Config::default()).unwrap();

	while window.update() {
		let hidden = window.input().get_key_state(Key::Space);
		window.input().set_cursor_hidden(hidden);
	}
}