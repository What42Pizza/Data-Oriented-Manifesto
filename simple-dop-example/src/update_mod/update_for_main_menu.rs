use crate::prelude::*;





// easily keep track of control flow
pub fn update(app: &mut App, program_data: &mut ProgramData, dt: f32) -> Result<()> {
	process_inputs_before_main(app, program_data);
	process_gui_clicks(app, program_data)?;
	let keyboard_data = gui_integration_mod::get_gui_keyboard_data(&app.keyboard);
	gui::update::update_gui_elements(&mut program_data.playing_data.gui, &keyboard_data);
	transfer_data_to_gui(&mut program_data.main_menu_data)?;
	process_inputs_after_main(app, program_data);
	Ok(())
}





pub fn process_inputs_before_main(app: &mut App, program_data: &mut ProgramData) {
	
	if app.keyboard.was_pressed(KeyCode::Escape) {
		program_data.exit = true;
		return;
	}
	
}





pub fn process_gui_clicks(app: &mut App, program_data: &mut ProgramData) -> Result<()> {
	
	let mouse_pos = app.mouse.position().to_i32();
	let last_screen_size = program_data.last_screen_size;
	if app.mouse.left_was_pressed() {
		let hovered_elements = gui::utils::get_hovered_elements(&program_data.main_menu_data.gui, mouse_pos, last_screen_size.to_tuple());
		for element in hovered_elements {
			if let Some(click_fn) = element.custom_data.click_fn {
				let result = click_fn(program_data);
				if let Err(err) = result {
					panic!("Error while processing button press: {err}");
				}
				break;
			}
		}
	}
	
	Ok(())
}



pub fn set_click_fns(gui: &mut GuiElement<CustomGuiData>) -> Result<()> {
	
	fn set_click_fn(element: &mut GuiElement<CustomGuiData>, click_fn: fn(&mut ProgramData) -> Result<()>) {
		element.custom_data.click_fn = Some(click_fn);
	}
	const GUI_ERROR_MESSAGE: &str = "Could not add click function";
	
	fn play_button(program_data: &mut ProgramData) -> Result<()> {
		let main_menu_data = &mut program_data.main_menu_data;
		if main_menu_data.enter_time.elapsed() < program_settings::MAIN_MENU_WAIT_DURATION {return Ok(());}
		program_data.mode = ProgramMode::Playing;
		program_data.playing_data.reset();
		Ok(())
	}
	set_click_fn(gui.child_mut_or_message("play_button", GUI_ERROR_MESSAGE)?, play_button);
	
	fn exit_button(program_data: &mut ProgramData) -> Result<()> {
		program_data.exit = true;
		Ok(())
	}
	set_click_fn(gui.child_mut_or_message("exit_button", GUI_ERROR_MESSAGE)?, exit_button);
	
	Ok(())
}





pub fn transfer_data_to_gui(main_menu_data: &mut MainMenuData) -> Result<()> {
	const GUI_ERROR_MESSAGE: &str = "Could not update gui";
	let gui = &mut main_menu_data.gui;
	
	let in_menu_duration = main_menu_data.enter_time.elapsed();
	let play_button = gui.child_mut_or_message("play_button", "could not update gui data")?;
	play_button.has_border = in_menu_duration > program_settings::MAIN_MENU_WAIT_DURATION;
	let play_button_progress = play_button.child_mut_or_message("play_button_progress", "could not update gui data")?;
	play_button_progress.width = in_menu_duration.as_secs_f32() / program_settings::MAIN_MENU_WAIT_DURATION.as_secs_f32();
	play_button_progress.width = play_button_progress.width.min(1.);
	
	Ok(())
}





pub fn process_inputs_after_main(app: &mut App, program_data: &mut ProgramData) {
	
	let wait_duration_ended = program_data.main_menu_data.enter_time.elapsed() > program_settings::MAIN_MENU_WAIT_DURATION;
	
	if app.keyboard.was_pressed(KeyCode::Space) && wait_duration_ended {
		program_data.mode = ProgramMode::Playing;
		program_data.playing_data.reset();
		return;
	}
	
}
