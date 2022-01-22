extern crate cbindgen;

use std::env;

fn main() {
	let out_dir = env::var("OUT_DIR").unwrap();
	let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
	let package_name = env::var("CARGO_PKG_NAME").unwrap().replace("-", "_");

	let output_file = format!("{}/{}.h", out_dir, package_name);

	cbindgen::generate(crate_dir)
		.unwrap()
		.write_to_file(output_file);
}
