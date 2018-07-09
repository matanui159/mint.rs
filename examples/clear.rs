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