extern crate gl_generator;

use gl_generator::*;
use std::env;
use std::path::Path;
use std::fs::File;

fn main() {
	let dir = env::var("OUT_DIR").unwrap();
	let path = Path::new(&dir).join("gl.rs");
	let mut file = File::create(&path).unwrap();

	Registry::new(Api::Gl, (3, 2), Profile::Core, Fallbacks::All, [])
		.write_bindings(StructGenerator, &mut file)
		.unwrap();
}