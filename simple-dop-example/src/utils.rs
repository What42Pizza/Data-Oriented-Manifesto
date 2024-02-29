use crate::prelude::*;
use std::fs;



pub fn get_program_dir() -> PathBuf {
	let mut output = std::env::current_exe()
		.expect("Could not retrieve the path for the current exe.");
	output.pop();
	output
}

pub fn get_program_file_path(input: impl AsRef<Path>) -> PathBuf {
	let input = input.as_ref();
	for path in get_program_dir().ancestors() {
		let mut output = path.to_path_buf();
		output.push(input);
		if output.exists() {
			return output;
		}
	}
	let mut output = get_program_dir();
	output.push(input);
	output
}



pub fn load_texture(path: impl AsRef<Path>, gfx: &mut Graphics) -> Result<Texture> {
	let texture_bytes = fs::read(path)?;
	gfx
		.create_texture()
		.from_image(&texture_bytes)
		.build()
		.map_err(Error::msg)
}



pub struct PackagedErrors (Vec<Error>);
