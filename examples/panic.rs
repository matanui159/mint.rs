extern crate mint2d;

use mint2d::core::{Window, Config, Fullscreen};
use mint2d::input::InputError;

extern crate backtrace;
use backtrace::Backtrace;

fn main() {
	let mut config = Config::default();
	config.fullscreen = Fullscreen::Monitor(String::from("not a monitor"));
	Window::new(config).unwrap();
}