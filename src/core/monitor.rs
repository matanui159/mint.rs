extern crate glutin;
use self::glutin::{MonitorId, AvailableMonitorsIter};

use ::Size;

/// A monitor or display.
#[derive(Clone, Debug)]
pub struct Monitor {
	pub(crate) monitor: MonitorId
}

impl Monitor {
	/// Gets the name of the monitor.
	/// This can be used to create a fullscreen window.
	pub fn get_name(&self) -> String {
		self.monitor.get_name().unwrap_or(String::new())
	}

	/// Gets the size of the monitor.
	pub fn get_size(&self) -> Size {
		let size = self.monitor.get_dimensions()
			.to_logical(self.monitor.get_hidpi_factor());
		Size {
			width: size.width,
			height: size.height
		}
	}
}

/// An iterator over monitors.
#[derive(Debug)]
pub struct MonitorIter {
	pub(crate) iter: AvailableMonitorsIter
}

impl Iterator for MonitorIter {
	type Item = Monitor;
	fn next(&mut self) -> Option<Monitor> {
		self.iter.next()
			.map(|monitor| Monitor {monitor})
	}
}