include!(concat!(env!("OUT_DIR"), "/gl.rs"));

pub trait CheckError {
	fn check_error(&self);
}

impl CheckError for Gl {
	fn check_error(&self) {
		unsafe {
			let error = self.GetError();
			if error != NO_ERROR {
				panic!("glGetError = 0x{:X}", error)
			}
		}
	}
}