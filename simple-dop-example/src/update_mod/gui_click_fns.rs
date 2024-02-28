use crate::prelude::*;



pub fn add_click_fns(gui: &mut GuiElement<CustomGuiData>) -> Result<()> {
	
	fn set_click_fn(element: &mut GuiElement<CustomGuiData>, click_fn: fn(&mut ProgramData) -> Result<()>) {
		element.custom_data.click_fn = Some(click_fn);
	}
	
	const MESSAGE: &str = "Could not add click function";
	
	
	
	let main_menu = gui.child_mut_or_message("main_menu", MESSAGE)?; {
		
		
		
		fn play_button(program_data: &mut ProgramData) -> Result<()> {
			
			program_data.mode = ProgramMode::Playing (PlayingData::new());
			program_data.gui.child_mut_or_message("main_menu", "Could not disable 'main_menu'")?.enabled = false;
			program_data.gui.child_mut_or_message("playing", "Could not enable 'playing'")?.enabled = true;
			
			Ok(())
		}
		set_click_fn(main_menu.child_mut_or_message("play_button", MESSAGE)?, play_button);
		
		
		
	}
	
	
	
	Ok(())
}
