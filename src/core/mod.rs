//! Core window, context and state management.

extern crate glutin;
use self::glutin::{EventsLoop, Event, WindowEvent, ElementState};
use self::glutin::{GlWindow, GlContext, GlRequest, Api};
use self::glutin::{WindowBuilder, ContextBuilder, dpi::LogicalSize};

use std::error::Error;
use std::fmt::{Display, Formatter, Error as FmtError};
use std::collections::HashSet;
use std::rc::Rc;

mod monitor;
pub use self::monitor::*;

mod config;
pub use self::config::*;

use ::{Point, Size, input::Input};

/// Possible errors that can occur from creating a window.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum WindowError {
	/// [`Fullscreen::Monitor`](enum.Fullscreen.html#variant.Monitor)
	/// didn't match any monitor name.
	UnknownMonitor,

	/// An unknown internal error occurred.
	InternalError(String)
}

impl Display for WindowError {
	fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
		match self {
			&WindowError::UnknownMonitor => write!(f, "Unknown monitor"),
			&WindowError::InternalError(ref error) => write!(f, "{}", error)
		}
	}
}
impl Error for WindowError {}

/// A window that handles the context and state of the game.
pub struct Window {
	events: EventsLoop,
	window: Rc<GlWindow>,
	input: Input
}

impl Window {
	/// # Errors
	/// If [`Config.fullscreen`](struct.Config.html#structfield.fullscreen)
	/// is [`Fullscreen::Monitor`](enum.Fullscreen.html#variant.Monitor)
	/// and it doesn't match any monitor name, this will return with
	/// [`WindowError::UnknownMonitor`](enum.WindowError.html#variant.UnknownMonitor).
	pub fn new(config: Config) -> Result<Window, WindowError> {
		let events = EventsLoop::new();
		let mut window = WindowBuilder::new()
			.with_title(config.title)
			.with_maximized(config.maximized)
			.with_resizable(config.resizable)
			.with_dimensions(LogicalSize {
				width: config.size.width,
				height: config.size.height
			});

		if let Some(size) = config.min_size {
			window = window.with_min_dimensions(LogicalSize {
				width: size.width,
				height: size.height
			});
		}

		if !config.resizable {
			window = window.with_max_dimensions(LogicalSize {
				width: config.size.width,
				height: config.size.height
			})
		} else if let Some(size) = config.max_size {
			window = window.with_max_dimensions(LogicalSize {
				width: size.width,
				height: size.height
			});
		}

		window = window.with_fullscreen(match config.fullscreen {
			Fullscreen::Disabled => None,
			Fullscreen::Primary => Some(events.get_primary_monitor()),
			Fullscreen::Monitor(name) => {
				let mut result = Err(WindowError::UnknownMonitor);
				for monitor in events.get_available_monitors() {
					if let Some(n) = monitor.get_name() {
						if name == n {
							result = Ok(monitor);
						}
					}
				}
				Some(result?)
			}
		});

		let context = ContextBuilder::new()
			.with_gl(GlRequest::Specific(Api::OpenGl, (3, 2)))
			.with_vsync(config.vsync)
			.with_multisampling(config.msaa);

		let window = GlWindow::new(window, context, &events)
			.map_err(|error| WindowError::InternalError(ToString::to_string(&error)))?;

		unsafe {
			window.make_current()
				.map_err(|error| WindowError::InternalError(ToString::to_string(&error)))?;
		}

		let rc = Rc::new(window);
		Ok(Window {
			events,
			window: Rc::clone(&rc),
			input: Input {
				window: Rc::clone(&rc),
				keys: HashSet::new(),
				buttons: HashSet::new(),
				cursor: Point::default()
			}
		})
	}

	/// Updates the window and processes all events.
	/// Will return false if the window has been closed,
	/// true otherwise.
	pub fn update(&mut self) -> Result<bool, String> {
		let input = &mut self.input;

		let mut result = true;
		self.events.poll_events(|event| {
			if let Event::WindowEvent {event, ..} = event {
				match event {
					WindowEvent::CloseRequested => result = false,
					WindowEvent::KeyboardInput {input: event, ..} => {
						if let Some(key) = event.virtual_keycode {
							match event.state {
								ElementState::Pressed => input.keys.insert(key),
								ElementState::Released => input.keys.remove(&key)
							};
						}
					},
					WindowEvent::MouseInput {button, state, ..} => {
						match state {
							ElementState::Pressed => input.buttons.insert(button),
							ElementState::Released => input.buttons.remove(&button)
						};
					},
					WindowEvent::CursorMoved {position, ..} => {
						input.cursor = Point {
							x: position.x,
							y: position.y
						}
					}
					_ => ()
				}
			}
		});
		Ok(result)
	}

	/// Gets the primary monitor.
	pub fn get_primary_monitor(&self) -> Monitor {
		Monitor {
			monitor: self.events.get_primary_monitor()
		}
	}

	/// Gets an iterator of all the monitors.
	pub fn get_all_monitors(&self) -> MonitorIter {
		MonitorIter {
			iter: self.events.get_available_monitors()
		}
	}

	/// Gets the current size of the window.
	pub fn get_size(&self) -> Size {
		self.window.get_inner_size()
			.map_or(Size {
				width: 1.0,
				height: 1.0
			}, |size| Size {
				width: size.width,
				height: size.height
			})
	}

	pub fn input(&mut self) -> &mut Input {
		&mut self.input
	}
}