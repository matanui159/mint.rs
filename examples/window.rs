extern crate mint2d;

use mint2d::core::{Window, Config};

fn main() {
	let mut config = Config::default();
	config.title = String::from("Hello, World!");

	let mut window = Window::new(config).unwrap();

	while window.update().unwrap() {}
}