use ::Size;

/// All the possible fullscreen configurations.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Fullscreen {
	/// Fullscreen is disabled.
	Disabled,

	/// Uses the primary monitor for fullscreen.
	Primary,

	/// Uses the monitor that matches the provided name for fullscreen.
	Monitor(String)
}

/// The configuration options when creating a window.
#[derive(Clone, Debug, PartialEq)]
pub struct Config {
	/// The title of the window.
	pub title: String,

	/// The initial size of the window.
	pub size: Size,

	/// The minimum size of the window.
	pub min_size: Option<Size>,

	/// The maximum size of the window.
	pub max_size: Option<Size>,

	/// Whether or not the window starts maximized.
	pub maximized: bool,

	/// Whether or not the window is resizable.
	pub resizable: bool,

	/// The fullscreen configuration.
	pub fullscreen: Fullscreen,

	/// Whether or not V-sync is enabled.
	pub vsync: bool,

	/// The MSAA amount. Must be a power of two.
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