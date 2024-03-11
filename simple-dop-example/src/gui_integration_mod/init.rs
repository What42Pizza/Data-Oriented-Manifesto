use crate::prelude::*;



pub struct GuiLoadingData<'a> {
	pub textures: &'a Textures,
}



pub fn init_gui(textures: &Textures) -> Result<GuiElement<CustomGuiData>> {
	
	// load gui
	let mut gui_loading_data = GuiLoadingData {
		textures,
	};
	let gui_dir = get_program_file_path("assets/gui");
	let mut errors = vec!();
	let gui = gui::load::load_gui::<CustomGuiData, GuiLoadingFnsImpl, GuiLoadingData>(gui_dir, &mut CustomGuiData::default, &mut gui_loading_data, &mut errors);
	let mut gui = match gui {
		Result::Ok (v) => v,
		Result::Err (err) => panic!("\n\nFatal error while loading gui:\n{err:?}\n"),
	};
	if !errors.is_empty() {
		println!("Errors ocurred while loading gui:");
		for error in errors {
			println!("{error}");
		}
	}
	
	// add click functions
	gui_click_fns::add_click_fns(&mut gui)?;
	
	Ok(gui)
}



pub struct GuiLoadingFnsImpl;

impl<'a> gui::load::LoadingFns<CustomGuiData, GuiLoadingData<'a>> for GuiLoadingFnsImpl {
	fn apply_custom_key(element: &mut GuiElement<CustomGuiData>, key: &str, value: &str, line: usize, path: &Path, loading_data: &mut GuiLoadingData) -> Result<gui::load::FieldWasApplied> {
		match key {
			
			"image" => apply_image(element, value, loading_data)?,
			
			"darken while pressed" => element.custom_data.darken_while_pressed = gui::load::parse_value_to_bool(value, line, path)?,
			"default background alpha" => element.custom_data.default_background_alpha = Some(match value.len() {
				1 => u8::from_str_radix(value, 16)? as f32 / 15.,
				2 => u8::from_str_radix(value, 16)? as f32 / 255.,
				_ => return Err(Error::msg(format!("Invalid value at {path:?} line {line} (value: \"{value}\"): Invalid length (must be 1 or 2 chars)"))),
			}),
			"default text alpha" => element.custom_data.default_text_alpha = Some(match value.len() {
				1 => u8::from_str_radix(value, 16)? as f32 / 15.,
				2 => u8::from_str_radix(value, 16)? as f32 / 255.,
				_ => return Err(Error::msg(format!("Invalid value at {path:?} line {line} (value: \"{value}\"): Invalid length (must be 1 or 2 chars)"))),
			}),
			
			_ => return Ok(false),
			
		};
		Ok(true)
	}
}

pub fn apply_image(element: &mut GuiElement<CustomGuiData>, value: &str, loading_data: &GuiLoadingData) -> Result<()> {
	let texture: &Texture = match value {
		
		//"base" => &loading_data.textures.base,
		
		_ => return Err(Error::msg(format!("Cannot apply image \"{value}\" to gui element. (element: \"{}\")", element.name))),
		
	};
	element.custom_data.image = Some(texture.clone());
	Ok(())
}
