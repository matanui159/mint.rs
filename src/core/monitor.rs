// Copyright 2018 Joshua Minter
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

extern crate glutin;
use self::glutin::{MonitorId, AvailableMonitorsIter};

use ::Size;

/// A monitor or display.
#[derive(Clone, Debug)]
pub struct Monitor {
	monitor: MonitorId
}

impl Monitor {
	pub(crate) fn new(monitor: MonitorId) -> Monitor {
		Monitor {monitor}
	}

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
	iter: AvailableMonitorsIter
}

impl MonitorIter {
	pub(crate) fn new(iter: AvailableMonitorsIter) -> MonitorIter {
		MonitorIter {iter}
	}
}

impl Iterator for MonitorIter {
	type Item = Monitor;
	fn next(&mut self) -> Option<Monitor> {
		self.iter.next()
			.map(|monitor| Monitor {monitor})
	}
}