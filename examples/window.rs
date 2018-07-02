extern crate mint2d;

use mint2d::core::{Window, Config};

fn main() {
	let mut window = Window::new(Config::default()).unwrap();
	while window.update() {};
}