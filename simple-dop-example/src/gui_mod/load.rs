use crate::gui_mod::internal_prelude::*;



pub type FieldWasApplied = bool;

pub trait LoadingFns<CustomData, LoadingData> {
	fn apply_custom_key(element: &mut GuiElement<CustomData>, key: &str, value: &str, line: usize, path: &Path, loading_data: &mut LoadingData) -> Result<FieldWasApplied, Error>;
}



pub fn load_gui<CustomData, LoadingFnsImpl: LoadingFns<CustomData, LoadingData>, LoadingData>(
	folder_path: impl AsRef<Path>,
	custom_data_fn: &mut impl FnMut() -> CustomData,
	loading_data: &mut LoadingData,
	errors: &mut Vec<Error>
) -> Result<GuiElement<CustomData>> {
	let (children_by_layer, children_by_name) = load_gui_elements_in_folder::<CustomData, LoadingFnsImpl, LoadingData>(folder_path, custom_data_fn, loading_data, errors)?;
	let output = GuiElement::new("main", children_by_layer, children_by_name, custom_data_fn);
	Ok(output)
}



#[allow(clippy::type_complexity)]
pub fn load_gui_elements_in_folder<CustomData, LoadingFnsImpl: LoadingFns<CustomData, LoadingData>, LoadingData>(
	folder_path: impl AsRef<Path>,
	custom_data_fn: &mut impl FnMut() -> CustomData, 
	loading_data: &mut LoadingData,
	errors: &mut Vec<Error>
) -> Result<(Vec<GuiElement<CustomData>>, HashMap<String, usize>)> {
	let folder_path = folder_path.as_ref();
	let (mut by_layer_output, mut by_name_output) = (Vec::<GuiElement<CustomData>>::new(), HashMap::new());
	
	for curr_path in fs::read_dir(folder_path)? {
		let curr_path = curr_path.expect("Could not read file in gui directory").path();
		if !curr_path.is_file() {continue;}
		if get_file_extension(&curr_path)? != "txt" {continue;}
		let new_gui_element = load_gui_element::<CustomData, LoadingFnsImpl, LoadingData>(&curr_path, custom_data_fn, loading_data, errors)?;
		let mut insert_i = by_layer_output.len();
		for (iter_i, element) in by_layer_output.iter().enumerate() {
			if element.render_priority > new_gui_element.render_priority {
				insert_i = iter_i;
				break;
			}
		}
		let name_clone = new_gui_element.name.clone();
		by_layer_output.insert(insert_i, new_gui_element);
		by_name_output.insert(name_clone, insert_i);
	}
	
	Ok((by_layer_output, by_name_output))
}



pub fn load_gui_element<CustomData, LoadingFnsImpl: LoadingFns<CustomData, LoadingData>, LoadingData>(
	path: impl AsRef<Path>,
	custom_data_fn: &mut impl FnMut() -> CustomData,
	loading_data: &mut LoadingData,
	errors: &mut Vec<Error>
) -> Result<GuiElement<CustomData>> {
	let path = path.as_ref();
	let element_name = get_file_name(path)?.to_string();
	let mut output = GuiElement::new(&element_name, vec!(), HashMap::new(), custom_data_fn);
	
	let file_string = fs::read_to_string(path)?;
	for (i, mut line) in file_string.split('\n').enumerate() {
		
		if let Some(comment_index) = line.find("//") {
			line = &line[0..comment_index];
		}
		line = line.trim();
		if line.is_empty() {continue;}
		
		let Some(colon_index) = line.find(':') else {
			errors.push(GuiError::MissingColon {line: i, path: path.to_path_buf()}.into());
			continue;
		};
		
		let field_name = line[..colon_index].trim();
		let field_value = line[(colon_index + 1)..].trim();
		match apply_field_to_element::<CustomData, LoadingFnsImpl, LoadingData>(&mut output, field_name, field_value, i, path, loading_data) {
			Result::Ok (()) => {},
			Result::Err (new_error) => errors.push(new_error),
		}
		
	}
	
	let mut children_path = path.to_path_buf();
	children_path.pop();
	children_path.push(String::from("in ") + &element_name);
	if children_path.exists() {
		let (children_by_layer, children_by_name) = load_gui_elements_in_folder::<CustomData, LoadingFnsImpl, LoadingData>(children_path, custom_data_fn, loading_data, errors)?;
		output.children_by_layer = children_by_layer;
		output.children_by_name = children_by_name;
	}
	
	Ok(output)
}



pub fn get_file_extension (path: &Path) -> Result<&str> {
	let ext = path.extension().ok_or_else(|| GuiError::InvalidFileName {path: path.to_path_buf()})?;
	let ext = ext.to_str().ok_or_else(|| GuiError::InvalidFileName {path: path.to_path_buf()})?;
	Ok(ext)
}

pub fn get_file_name (path: &Path) -> Result<&str> {
	let name = path.file_stem().ok_or_else(|| GuiError::InvalidFileName {path: path.to_path_buf()})?;
	let name = name.to_str().ok_or_else(|| GuiError::InvalidFileName {path: path.to_path_buf()})?;
	Ok(name)
}





pub fn apply_field_to_element<CustomData, LoadingDataImpls: LoadingFns<CustomData, LoadingData>, LoadingData>(
	element: &mut GuiElement<CustomData>,
	field_name: &str,
	field_value: &str,
	line: usize,
	path: &Path,
	loading_data: &mut LoadingData
) -> Result<()> {
	'apply_vanilla_field: {
		match field_name {
			
			"render priority" => element.render_priority = parse_value_to_f64(field_value, line, path)?,
			"visible" => element.visible = parse_value_to_bool(field_value, line, path)?,
			"enabled" => element.enabled = parse_value_to_bool(field_value, line, path)?,
			
			"x" => element.x = parse_value_to_f64(field_value, line, path)?,
			"y" => element.y = parse_value_to_f64(field_value, line, path)?,
			"width"  => element.width  = parse_value_to_f64(field_value, line, path)?,
			"height" => element.height = parse_value_to_f64(field_value, line, path)?,
			"natural x"     => element.natural_x     = parse_value_to_f64(field_value, line, path)?,
			"natural width" => element.natural_width = parse_value_to_f64(field_value, line, path)?,
			
			"has background"   => element.has_background = parse_value_to_bool(field_value, line, path)?,
			"background color" => element.background_color = parse_value_to_color(field_value, line, path)?,
			
			"has border"   => element.has_border   = parse_value_to_bool(field_value, line, path)?,
			"border color" => element.border_color = parse_value_to_color(field_value, line, path)?,
			"border width" => element.border_width = parse_value_to_f64(field_value, line, path)?,
			
			"has text"     => element.has_text     = parse_value_to_bool(field_value, line, path)?,
			"text color"   => element.text_color   = parse_value_to_color(field_value, line, path)?,
			"text"         => element.text         = parse_value_to_string_vec(field_value),
			"text x align" => element.text_x_align = parse_value_to_x_alignment(field_value, line, path)?,
			"text y align" => element.text_y_align = parse_value_to_y_alignment(field_value, line, path)?,
			"text size"    => element.text_size    = parse_value_to_f64(field_value, line, path)?,
			"is editing text"         => element.is_editing_text         = parse_value_to_bool(field_value, line, path)?,
			"can edit multiline"      => element.can_edit_multiline      = parse_value_to_bool(field_value, line, path)?,
			"return finishes editing" => element.return_finishes_editing = parse_value_to_bool(field_value, line, path)?,
			"esc finishes editing"    => element.esc_finishes_editing    = parse_value_to_bool(field_value, line, path)?,
			
			_ => break 'apply_vanilla_field,
		}
		return Ok(());
	}
	let field_was_applied =
		LoadingDataImpls::apply_custom_key(element, field_name, field_value, line, path, loading_data)
		.map_err(|err| GuiError::ApplyFieldError {cause: err})?;
	if !field_was_applied {
		Err(GuiError::InvalidFieldName {field_name: field_name.to_string(), line, path: path.to_path_buf()}.into())
	} else {
		Ok(())
	}
}



pub fn parse_value_to_f64(value: &str, line: usize, path: &Path) -> Result<f64> {
	value
		.parse::<f64>()
		.map_err(|cause|
			GuiError::CannotCastToFloat {value: value.to_string(), line, path: path.to_path_buf(), cause}.into()
		)
}

pub fn parse_value_to_bool(value: &str, line: usize, path: &Path) -> Result<bool> {
	value
		.parse::<bool>()
		.map_err(|cause|
			GuiError::CannotCastToBool {value: value.to_string(), line, path: path.to_path_buf(), cause}.into()
		)
}

pub fn parse_value_to_string_vec(value: &str) -> Vec<String> {
	let mut output = vec!(String::new());
	let mut is_special_char = false;
	for curr_char in value.chars() {
		if is_special_char {
			is_special_char = false;
			if curr_char == 'n' {
				output.push(String::new());
				continue;
			}
			output.last_mut().unwrap().push(curr_char);
			continue;
		}
		if curr_char == '\\' {
			is_special_char = true;
			continue;
		}
		output.last_mut().unwrap().push(curr_char);
	}
	output
}

pub fn parse_value_to_x_alignment(value: &str, line: usize, path: &Path) -> Result<XAlignment> {
	match &*value.to_lowercase() {
		"left" => Ok(XAlignment::Left),
		"center" => Ok(XAlignment::Center),
		"right" => Ok(XAlignment::Right),
		_ => Err(GuiError::CannotCastToXAlignment {value: value.to_string(), line, path: path.to_path_buf()}.into()),
	}
}

pub fn parse_value_to_y_alignment(value: &str, line: usize, path: &Path) -> Result<YAlignment> {
	match &*value.to_lowercase() {
		"bottom" => Ok(YAlignment::Bottom),
		"center" => Ok(YAlignment::Center),
		"top" => Ok(YAlignment::Top),
		_ => Err(GuiError::CannotCastToYAlignment {value: value.to_string(), line, path: path.to_path_buf()}.into()),
	}
}

pub fn parse_value_to_color(mut value: &str, line: usize, path: &Path) -> Result<Color> {
	let hex_to_u8 = |value| {
		match u8::from_str_radix(value, 16) {
			StdResult::Ok (v) => Ok(v),
			StdResult::Err (err) => Err(GuiError::CannotCastToColor {
				value: value.to_string(),
				line,
				path: path.to_path_buf(),
				cause: CannotCastToColorCause::ParseIntError (err),
			}.into())
		}
	};
	if value.starts_with("0x") {value = &value[2..];}
	let (r, g, b, a);
	match value.len() {
		1 => {
			r = hex_to_u8(&value[0..1])? as f32 / 15.;
			g = r;
			b = r;
			a = 1.;
		}
		3 => {
			r = hex_to_u8(&value[0..1])? as f32 / 15.;
			g = hex_to_u8(&value[1..2])? as f32 / 15.;
			b = hex_to_u8(&value[2..3])? as f32 / 15.;
			a = 1.;
		}
		4 => {
			r = hex_to_u8(&value[0..1])? as f32 / 15.;
			g = hex_to_u8(&value[1..2])? as f32 / 15.;
			b = hex_to_u8(&value[2..3])? as f32 / 15.;
			a = hex_to_u8(&value[3..4])? as f32 / 15.;
		}
		6 => {
			r = hex_to_u8(&value[0..2])? as f32 / 255.;
			g = hex_to_u8(&value[2..4])? as f32 / 255.;
			b = hex_to_u8(&value[4..6])? as f32 / 255.;
			a = 1.;
		}
		8 => {
			r = hex_to_u8(&value[0..2])? as f32 / 255.;
			g = hex_to_u8(&value[2..4])? as f32 / 255.;
			b = hex_to_u8(&value[4..6])? as f32 / 255.;
			a = hex_to_u8(&value[6..8])? as f32 / 255.;
		}
		_ => return Err(GuiError::CannotCastToColor {value: value.to_string(), line, path: path.to_path_buf(), cause: CannotCastToColorCause::InvalidLength{len: value.len()}}.into()),
	}
	Ok(Color::new(r, g, b, a))
}