use crate::{gui_integration_mod, prelude::*};



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
	let dt = app.system_timer.delta_f32();
	
	
	
	// inputs before main update
	process_inputs::process_input_start(app, program_data, dt)?;
	if program_data.exit {
		app.exit();
		return Ok(());
	}
	
	
	
	// gui
	
	let mouse_pos = app.mouse.position().to_i32();
	let last_screen_size = program_data.last_screen_size;
	if app.mouse.left_was_pressed() {
		let hovered_elements = gui_mod::gui_utils::get_hovered_elements(&program_data.gui, mouse_pos, last_screen_size.to_tuple());
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
	
	let initial_area = RealArea::new(program_data.last_screen_size.to_tuple());
	let keyboard_data = gui_integration_mod::get_gui_keyboard_data(&app.keyboard);
	update_gui_elements(&mut program_data.gui, initial_area, &keyboard_data);
	
	
	
	// main logic
	do_main_update(program_data, dt)?;
	
	
	
	// inputs after main update
	process_inputs::process_input_end(app, program_data, dt)?;
	if program_data.exit {
		app.exit();
		return Ok(());
	}
	
	
	
	Ok(())
}





pub fn do_main_update(program_data: &mut ProgramData, dt: f32) -> Result<()> {
	match &mut program_data.mode {
		
		
		
		ProgramMode::MainMenu (main_menu_data) => {
			
		}
		
		
		
		ProgramMode::Playing (playing_data) => {
			
			playing_data.player_pos += playing_data.player_vel * dt;
			playing_data.player_vel *= program_settings::PLAYER_DRAG_COEF.powf(dt);
			playing_data.player_pos = playing_data.player_pos.clamp(program_settings::PLAYER_SIZE * 0.5, 1.0 - program_settings::PLAYER_SIZE * 0.5);
			
			let mut bullet_datas = BulletDataRefs {
				bullets: &mut playing_data.player_bullets,
				player_pos: &playing_data.player_pos,
				player_health: &mut playing_data.player_health,
				enemies: &mut playing_data.enemies,
			};
			update_bullets(&mut bullet_datas, player_bullet_collision);
			
			bullet_datas.bullets = &mut playing_data.enemy_bullets;
			update_bullets(&mut bullet_datas, enemy_bullet_collision);
			
		}
		
		
		
	}
	Ok(())
}





// holds ALL the data needed to update the bullets
pub struct BulletDataRefs<'a> {
	bullets: &'a mut Vec<Bullet>,
	player_pos: &'a Vec2,
	player_health: &'a mut f32,
	enemies: &'a mut Vec<Enemy>,
}

type ShouldRemoveBullet = bool;

pub fn update_bullets(bullet_datas: &mut BulletDataRefs, collision_logic: fn(&mut BulletDataRefs, usize) -> ShouldRemoveBullet) {
	let mut i = 0;
	while i < bullet_datas.bullets.len() {
		let curr_bullet = &mut bullet_datas.bullets[i];
		
		// update pos
		curr_bullet.pos += curr_bullet.vel;
		if
			curr_bullet.pos.x > 1.1
			|| curr_bullet.pos.x < -0.1
			|| curr_bullet.pos.y > 1.1
			|| curr_bullet.pos.y < -0.1
		{
			bullet_datas.bullets.remove(i);
			continue; // note: don't inc i
		}
		
		// collision logic
		let should_remove_bullet = collision_logic(bullet_datas, i);
		if should_remove_bullet {
			bullet_datas.bullets.remove(i);
			continue; // note: don't inc i
		}
		
		i += 1;
	}
}



pub fn player_bullet_collision(bullet_datas: &mut BulletDataRefs, i: usize) -> ShouldRemoveBullet {
	let curr_bullet = &mut bullet_datas.bullets[i];
	
	let hit_enemy =
		bullet_datas.enemies
		.iter().enumerate()
		.find(|(_, enemy)| {
			let dist = (enemy.pos - curr_bullet.pos).length();
			dist <= program_settings::ENEMY_HIT_RADIUS
		});
	
	if let Some((hit_enemy_index, _)) = hit_enemy {
		bullet_datas.enemies.remove(hit_enemy_index);
		return true
	}
	
	false
}



pub fn enemy_bullet_collision(bullet_datas: &mut BulletDataRefs, i: usize) -> ShouldRemoveBullet {
	let curr_bullet = &mut bullet_datas.bullets[i];
	
	let dist = (*bullet_datas.player_pos - curr_bullet.pos).length();
	let is_hit = dist <= program_settings::PLAYER_HIT_RADIUS;
	
	if is_hit {
		*bullet_datas.player_health = 0.;
		return true;
	}
	
	false
}
