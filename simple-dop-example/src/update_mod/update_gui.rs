use crate::prelude::*;



const GUI_ERROR_MESSAGE: &str = "could not update gui data";





pub fn update_gui_data(program_data: &mut ProgramData) -> Result<()> {
	match &mut program_data.mode {
		
		
		
		ProgramMode::MainMenu (main_menu_data) => {
			let main_menu = program_data.gui.child_mut_or_message("main_menu", GUI_ERROR_MESSAGE)?;
			
			let in_menu_duration = main_menu_data.enter_time.elapsed();
			let play_button = main_menu.child_mut_or_message("play_button", "could not update gui data")?;
			play_button.has_border = in_menu_duration > program_settings::MAIN_MENU_WAIT_DURATION;
			let play_button_progress = play_button.child_mut_or_message("play_button_progress", "could not update gui data")?;
			play_button_progress.width = in_menu_duration.as_secs_f32() / program_settings::MAIN_MENU_WAIT_DURATION.as_secs_f32();
			play_button_progress.width = play_button_progress.width.min(1.);
				
		},
		
		
		
		ProgramMode::Playing (playing_data) => update_playing_gui_data(&mut program_data.gui, playing_data)?,
		
		
		
	}
	Ok(())
}





pub fn update_playing_gui_data(gui: &mut GuiElement<CustomGuiData>, playing_data: &mut PlayingData) -> Result<()> {
	let playing = gui.child_mut_or_message("playing", GUI_ERROR_MESSAGE)?;
	
	// update pause menu
	let pause_data = &playing_data.pause_data;
	if pause_data.needs_gui_update {
		let curr_menu_alpha = pause_data.curr_menu_transparency;
		gui::utils::apply_to_all_children(playing.child_mut_or_message("main_pause_menu", GUI_ERROR_MESSAGE)?, |element| {
			if let Some(default_background_alpha) = element.custom_data.default_background_alpha {
				element.background_color.a = default_background_alpha * curr_menu_alpha;
			}
			if let Some(default_text_alpha) = element.custom_data.default_text_alpha {
				element.text_color.a = default_text_alpha * curr_menu_alpha;
			}
			Ok(())
		})?;
	}
	
	Ok(())
}
