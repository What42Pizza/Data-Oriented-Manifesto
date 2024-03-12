use crate::{gui_integration_mod::init, prelude::*};

pub mod update_for_main_menu;
pub mod update_for_playing;



pub fn update_wrapper(app: &mut App, program_data: &mut ProgramData) {
	update(app, program_data).unwrap_or_else(|err| {
		println!("\n\n\n======== FATAL ERROR DURING UPDATE ========");
		for err in err.chain() {
			println!("{err}");
		}
		println!();
		panic!("[see above errors]");
	})
}

pub fn update(app: &mut App, program_data: &mut ProgramData) -> Result<()> {
	
	if app.keyboard.ctrl() && app.keyboard.was_pressed(KeyCode::R) {
		println!("Reloading gui...");
		init::init_all_guis(program_data)?;
		println!("Done");
	}
	
	let dt = app.system_timer.delta_f32();
	match &mut program_data.mode {
		ProgramMode::MainMenu => update_for_main_menu::update(app, program_data, dt),
		ProgramMode::Playing => update_for_playing::update(app, program_data, dt),
	}?;
	
	if program_data.exit {
		app.exit();
	}
	
	Ok(())
}
