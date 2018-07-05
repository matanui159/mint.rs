extern crate mint2d;

use mint2d::core::{Window, Config};
use mint2d::input::Button;

fn main() {
	let mut window = Window::new(Config::default()).unwrap();
	while window.update().unwrap() {
		let hidden = window.input().get_button_state(Button::Left);
		window.input().set_cursor_hidden(hidden);
	}
}