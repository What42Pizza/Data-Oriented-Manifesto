use crate::prelude::*;



// inputs that should be processed before the main update
pub fn process_input_start(app: &mut App, program_data: &mut ProgramData, dt: f32) -> Result<()> {
	let screen_size = program_data.last_screen_size;
	let shift = app.keyboard.shift();
	let ctrl = app.keyboard.ctrl();
	let alt = app.keyboard.alt();
	
	if ctrl && app.keyboard.was_pressed(KeyCode::R) {
		println!("Reloading gui...");
		program_data.gui = init_gui(&program_data.textures)?;
		println!("Done");
	}
	
	match &mut program_data.mode {
		
		
		
		ProgramMode::MainMenu (main_menu_data) => {
			
			if app.keyboard.was_pressed(KeyCode::Escape) {
				program_data.exit = true;
				return Ok(());
			}
			
		}
		
		
		
		ProgramMode::Playing (playing_data) => {
			
			if app.keyboard.is_down(KeyCode::W) || app.keyboard.is_down(KeyCode::Up) {
				playing_data.player_vel.y -= program_settings::PLAYER_ACCELERATION * dt;
			}
			
			if app.keyboard.is_down(KeyCode::S) || app.keyboard.is_down(KeyCode::Down) {
				playing_data.player_vel.y += program_settings::PLAYER_ACCELERATION * dt;
			}
			
			if app.keyboard.is_down(KeyCode::A) || app.keyboard.is_down(KeyCode::Left) {
				playing_data.player_vel.x -= program_settings::PLAYER_ACCELERATION * dt;
			}
			
			if app.keyboard.is_down(KeyCode::D) || app.keyboard.is_down(KeyCode::Right) {
				playing_data.player_vel.x += program_settings::PLAYER_ACCELERATION * dt;
			}
			
		}
		
		
		
	}
	Ok(())
}





// inputs that should be processed after the main update
pub fn process_input_end(app: &mut App, program_data: &mut ProgramData, dt: f32) -> Result<()> {
	let screen_size = program_data.last_screen_size;
	let shift = app.keyboard.shift();
	let ctrl = app.keyboard.ctrl();
	let alt = app.keyboard.alt();
	match &mut program_data.mode {
		
		
		
		ProgramMode::MainMenu (main_menu_data) => {
			
			let wait_duration_ended = main_menu_data.enter_time.elapsed() > program_settings::MAIN_MENU_WAIT_DURATION;
			
			if app.keyboard.was_pressed(KeyCode::Space) && wait_duration_ended {
				update_utils::switch_from_main_menu_to_playing(program_data)?;
				return Ok(());
			}
			
		}
		
		
		
		ProgramMode::Playing (playing_data) => {
			
			if app.keyboard.was_pressed(KeyCode::Escape) {
				match playing_data.paused_data {
					PausedData::Paused {enter_time} => {
						playing_data.paused_data = PausedData::Unpaused {enter_time: PausedData::flip_fade_percent(enter_time)};
					}
					PausedData::Unpaused {enter_time} => {
						playing_data.paused_data = PausedData::Paused {enter_time: PausedData::flip_fade_percent(enter_time)};
					}
				}
			}
			
			if app.mouse.left_was_pressed() {
				let mouse_pos = app.mouse.position().to_vec2() / screen_size.as_vec2();
				let new_bullet = Bullet::new(playing_data.player_pos, mouse_pos, program_settings::PLAYER_BULLET_SPEED);
				playing_data.player_bullets.push(new_bullet);
			}
			
		}
		
		
		
	}
	Ok(())
}
