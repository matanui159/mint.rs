use ::Size;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Fullscreen {
	Disabled,
	Primary,
	Monitor(String)
}

#[derive(Clone, Debug, PartialEq)]
pub struct Config {
	pub title: String,

	pub size: Size,
	pub min_size: Option<Size>,
	pub max_size: Option<Size>,
	pub maximized: bool,
	pub resizable: bool,
	pub fullscreen: Fullscreen,

	pub vsync: bool,
	pub msaa: u16
}

impl Default for Config {
	fn default() -> Config {
		Config {
			title: String::new(),
			size: Size {width: 640.0, height: 480.0},
			min_size: None,
			max_size: None,
			maximized: false,
			resizable: true,
			fullscreen: Fullscreen::Disabled,
			vsync: true,
			msaa: 0
		}
	}
}