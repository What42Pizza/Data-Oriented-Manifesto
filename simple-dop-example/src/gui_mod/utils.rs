use crate::gui_mod::internal_prelude::*;
use ab_glyph::*;





pub fn get_hovered_elements<CustomData>(element: &GuiElement<CustomData>, mouse_pos: (i32, i32), screen_size: (u32, u32)) -> Vec<&GuiElement<CustomData>> {
	let mut output = vec!();
	add_hovered_elements(element, mouse_pos, &RealArea::new(screen_size), &mut output);
	output
}

pub fn add_hovered_elements<'a, CustomData>(element: &'a GuiElement<CustomData>, mouse_pos: (i32, i32), parent_area: &RealArea, output: &mut Vec<&'a GuiElement<CustomData>>) {
	if !element.enabled {return;}
	let area = parent_area.get_sub_area(element.x, element.y, element.width, element.height, element.natural_x, element.natural_width);
	if !area.contains_point(mouse_pos) {return;}
	if element.visible {output.push(element);}
	for child in &element.children_by_layer {
		add_hovered_elements(child, mouse_pos, &area, output);
	}
}





pub fn apply_to_all_children<CustomData>(element: &mut GuiElement<CustomData>, update_fn: impl Fn(&mut GuiElement<CustomData>) -> Result<()>) -> Result<()> {
	let mut elements_to_update = vec!(element);
	loop {
		let Some(curr_element) = elements_to_update.pop() else {break};
		update_fn(curr_element)?;
		for child in &mut curr_element.children_by_layer {
			elements_to_update.push(child);
		}
	}
	Ok(())
}





pub fn get_char_spacings<CustomData>(element: &GuiElement<CustomData>, font: &FontVec, text_size: f32) -> Vec<Vec<f32>> {
	let font = font.as_scaled(PxScale::from(text_size));
	let mut output = Vec::with_capacity(element.text.len());
	for line in &element.text {
		let mut pos = 0.;
		let mut line_positions = Vec::with_capacity(line.len());
		for char in line.chars() {
			let glyph = font.glyph_id(char);
			pos += font.h_advance(glyph);
			line_positions.push(pos);
		}
		output.push(line_positions);
	}
	output
}



pub fn get_line_start_pos<CustomData>(element: &GuiElement<CustomData>, element_pos: (f32, f32), element_size: (f32, f32), text_size: f32, line_num: usize, line_width: f32) -> (f32, f32) {
	let x_align_mult = match element.text_x_align {
		XAlignment::Left => 0.,
		XAlignment::Center => 0.5,
		XAlignment::Right => 1.,
	};
	let y_align_mult = match element.text_y_align {
		YAlignment::Bottom => 1.,
		YAlignment::Center => 0.5,
		YAlignment::Top => 0.,
	};
	let x_margin = element_size.0 - (line_width - text_size * 0.05).max(0.);
	let y_margin = element_size.1 - element.text.len() as f32 * text_size * 1.1;
	let x = element_pos.0 + x_margin * x_align_mult;
	let y = element_pos.1 + y_margin * y_align_mult + (line_num as f32 + 0.05) * text_size * 1.1;
	(x, y)
}
