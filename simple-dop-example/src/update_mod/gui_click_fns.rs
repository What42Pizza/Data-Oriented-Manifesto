use crate::prelude::*;



pub fn add_click_fns(gui: &mut GuiElement<CustomGuiData>) -> Result<()> {
	
	fn set_click_fn(element: &mut GuiElement<CustomGuiData>, click_fn: fn(&mut ProgramData) -> Result<()>) {
		element.custom_data.click_fn = Some(click_fn);
	}
	
	const MESSAGE: &str = "Could not add click function";
	
	
	
	let main_menu = gui.child_mut_or_message("main_menu", MESSAGE)?; {
		
		
		
		fn play_button(program_data: &mut ProgramData) -> Result<()> {
			let ProgramMode::MainMenu (main_menu_data) = &program_data.mode else {panic!("main_menu/play_button was clicked even though program_data.mode isn't ProgramMode::MainMenu");};
			if main_menu_data.enter_time.elapsed() < program_settings::MAIN_MENU_WAIT_DURATION {return Ok(());}
			update_utils::switch_from_main_menu_to_playing(program_data)
		}
		set_click_fn(main_menu.child_mut_or_message("play_button", MESSAGE)?, play_button);
		
		
		
		fn exit_button(program_data: &mut ProgramData) -> Result<()> {
			program_data.exit = true;
			Ok(())
		}
		set_click_fn(main_menu.child_mut_or_message("exit_button", MESSAGE)?, exit_button);
		
		
		
	}
	
	
	
	let playing = gui.child_mut_or_message("playing", MESSAGE)?; {
		
		
		
		let main_pause_menu = playing.child_mut_or_message("main_pause_menu", MESSAGE)?; {
			
			
			
			fn resume_button(program_data: &mut ProgramData) -> Result<()> {
				let ProgramMode::Playing (playing_data) = &mut program_data.mode else {panic!("playing/pause_menu/resume_button was clicked even though program_data.mode isn't ProgramMode::Playing");};
				playing_data.pause_data.is_paused = false;
				Ok(())
			}
			set_click_fn(main_pause_menu.child_mut_or_message("resume_button", MESSAGE)?, resume_button);
			
			
			
		}
		
		
		
	}
	
	
	
	Ok(())
}
