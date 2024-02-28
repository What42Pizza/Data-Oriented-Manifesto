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
	let gui = load_gui::<CustomGuiData, GuiLoadingFnsImpl, GuiLoadingData>(gui_dir, &mut CustomGuiData::default, &mut gui_loading_data, &mut errors);
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

impl<'a> LoadingFns<CustomGuiData, GuiLoadingData<'a>> for GuiLoadingFnsImpl {
	fn apply_custom_key(element: &mut GuiElement<CustomGuiData>, key: &str, value: &str, line: usize, path: &Path, loading_data: &mut GuiLoadingData) -> StdResult<FieldWasApplied, Box<dyn StdError>> {
		match key {
			
			"image" => apply_image(element, value, loading_data)?,
			
			"darken while pressed" => element.custom_data.darken_while_pressed = gui_mod::load::parse_value_to_bool(value, line, path)?,
			
			_ => return StdResult::Ok(false),
			
		};
		StdResult::Ok(true)
	}
}

pub fn apply_image(element: &mut GuiElement<CustomGuiData>, value: &str, loading_data: &GuiLoadingData) -> StdResult<(), Box<dyn StdError>> {
	let texture: &Texture = match value {
		
		//"base" => &loading_data.textures.base,
		
		_ => return GuiApplyFieldError::err_from_string(format!("Cannot apply image \"{value}\" to gui element. (element: \"{}\")", element.name)),
		
	};
	element.custom_data.image = Some(texture.clone());
	StdResult::Ok(())
}



#[derive(Debug)]
pub struct GuiApplyFieldError {
	pub message: String,
}

impl std::fmt::Display for GuiApplyFieldError {
	fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(fmt, "GuiApplyFieldError: \"{}\"", self.message)
	}
}

impl StdError for GuiApplyFieldError {}

impl GuiApplyFieldError {
	pub fn err_from_string(message: String) -> StdResult<(), Box<dyn StdError>> {
		StdResult::Err(Box::new(Self {
			message
		}))
	}
}
