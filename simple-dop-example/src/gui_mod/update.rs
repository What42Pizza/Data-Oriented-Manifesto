use crate::gui_mod::internal_prelude::*;





pub fn update_gui_elements<CustomData>(element: &mut GuiElement<CustomData>, keyboard_data: &GuiKeyboardData) {
	
	// update real area
	//let current_area: RealArea = parent_area.get_sub_area_for_element(element);
	//element.latest_real_area = current_area;
	
	// update text
	if element.has_text && element.is_editing_text {
		process_typing(element, keyboard_data);
	}
	
	// update children
	for child in &mut element.children_by_layer {
		update_gui_elements(child, keyboard_data);
	}
	
}





pub fn process_typing<CustomData>(element: &mut GuiElement<CustomData>, keyboard_data: &GuiKeyboardData) {
	
	
	
	for control in &keyboard_data.control_keys_just_pressed {
		match control {
			
			
			
			ControlKey::Up => {
				if !element.can_edit_multiline {continue;}
				if element.cursor_y == 0 {
					element.cursor_x = 0;
					continue;
				}
				element.cursor_y -= 1;
				element.cursor_y = element.cursor_y.min(element.text.len() - 1);
				element.cursor_x = element.cursor_target_x.min(element.text[element.cursor_y].len());
			}
			
			ControlKey::Down => {
				if !element.can_edit_multiline {continue;}
				element.cursor_y = element.cursor_y.min(element.text.len() - 1);
				if element.cursor_y == element.text.len() - 1 {
					element.cursor_x = element.text[element.cursor_y].len();
					continue;
				}
				element.cursor_y += 1;
				element.cursor_y = element.cursor_y.min(element.text.len() - 1);
				element.cursor_x = element.cursor_target_x.min(element.text[element.cursor_y].len());
			}
			
			ControlKey::Left => {
				if element.cursor_x == 0 {
					if element.cursor_y == 0 {continue;}
					element.cursor_y = (element.cursor_y - 1).min(element.text.len() - 1);
					element.cursor_x = element.text[element.cursor_y].len();
					element.cursor_target_x = element.cursor_x;
					continue;
				}
				element.cursor_x -= 1;
				element.cursor_target_x = element.cursor_x;
			}
			
			ControlKey::Right => {
				element.cursor_y = element.cursor_y.min(element.text.len() - 1);
				if element.cursor_x == element.text[element.cursor_y].len() {
					if element.cursor_y == element.text.len() - 1 {continue;}
					element.cursor_y += 1;
					element.cursor_x = 0;
					element.cursor_target_x = 0;
					continue;
				}
				element.cursor_x += 1;
				element.cursor_target_x = element.cursor_x;
			}
			
			
			
			ControlKey::Return => {
				if element.return_finishes_editing && !keyboard_data.shift_is_pressed {
					element.is_editing_text = false;
					continue;
				}
				if !element.can_edit_multiline {continue;}
				element.cursor_y = element.cursor_y.min(element.text.len() - 1);
				element.cursor_x = element.cursor_x.min(element.text[element.cursor_y].len());
				let current_line = &*element.text[element.cursor_y];
				let new_line = current_line[element.cursor_x..].to_string();
				element.text[element.cursor_y].truncate(element.cursor_x);
				element.text.insert(element.cursor_y + 1, new_line);
				element.cursor_y += 1;
				element.cursor_x = 0;
				element.cursor_target_x = 0;
			}
			
			ControlKey::Delete => {
				element.cursor_y = element.cursor_y.min(element.text.len() - 1);
				element.cursor_x = element.cursor_x.min(element.text[element.cursor_y].len());
				if element.cursor_x == element.text[element.cursor_y].len() {
					if !element.can_edit_multiline {continue;}
					if element.cursor_y == element.text.len() - 1 {continue;}
					let next_line = element.text.remove(element.cursor_y + 1);
					element.text[element.cursor_y].push_str(&next_line);
					continue;
				}
				let current_line = &mut element.text[element.cursor_y];
				current_line.remove(element.cursor_x);
			}
			
			ControlKey::Backspace => {
				element.cursor_y = element.cursor_y.min(element.text.len() - 1);
				element.cursor_x = element.cursor_x.min(element.text[element.cursor_y].len());
				if element.cursor_x == 0 {
					if !element.can_edit_multiline {continue;}
					if element.cursor_y == 0 {continue;}
					let current_line = element.text.remove(element.cursor_y);
					let prev_line = &mut element.text[element.cursor_y - 1];
					let new_cursor_x = prev_line.len();
					prev_line.push_str(&current_line);
					element.cursor_y -= 1;
					element.cursor_x = new_cursor_x;
					element.cursor_target_x = element.cursor_x;
					continue;
				}
				let current_line = &mut element.text[element.cursor_y];
				current_line.remove(element.cursor_x - 1);
				element.cursor_x -= 1;
				element.cursor_target_x = element.cursor_x;
			}
			
			ControlKey::End => {
				element.cursor_y = element.cursor_y.min(element.text.len() - 1);
				element.cursor_x = element.text[element.cursor_y].len();
				element.cursor_target_x = element.cursor_x;
			}
			
			ControlKey::Esc => {
				if element.esc_finishes_editing {
					element.is_editing_text = false;
				}
			}
			
			
			
		}
	}
	
	
	
	if keyboard_data.text_just_typed.is_empty() {return;}
	
	let current_line = &mut element.text[element.cursor_y];
	current_line.insert_str(element.cursor_x, &keyboard_data.text_just_typed);
	element.cursor_x += keyboard_data.text_just_typed.len();
	element.cursor_target_x = element.cursor_x;
	
	
	
}
