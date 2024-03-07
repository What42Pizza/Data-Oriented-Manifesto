use crate::prelude::*;



pub fn switch_from_main_menu_to_playing(program_data: &mut ProgramData) -> Result<()> {
	program_data.mode = ProgramMode::Playing (PlayingData::new());
	program_data.gui.child_mut_or_message("main_menu", "Could not disable 'main_menu'")?.enabled = false;
	program_data.gui.child_mut_or_message("playing", "Could not enable 'playing'")?.enabled = true;
	Ok(())
}
