use crate::prelude::*;





pub fn update(app: &mut App, program_data: &mut ProgramData, dt: f32) -> Result<()> {
	
	
	
	// process inputs before main update
	let playing_data = &mut program_data.playing_data;
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
	
	
	
	// gui
	let mouse_pos = app.mouse.position().to_i32();
	let last_screen_size = program_data.last_screen_size;
	if app.mouse.left_was_pressed() {
		let hovered_elements = gui::utils::get_hovered_elements(&program_data.playing_data.gui, mouse_pos, last_screen_size.to_tuple());
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
	let playing_data = &mut program_data.playing_data;
	
	let keyboard_data = gui_integration_mod::get_gui_keyboard_data(&app.keyboard);
	gui::update::update_gui_elements(&mut playing_data.gui, &keyboard_data);
	
	
	
	// main logic
	
	
	// update player
	playing_data.player_pos += playing_data.player_vel * dt;
	playing_data.player_vel *= program_settings::PLAYER_DRAG_COEF.powf(dt);
	playing_data.player_pos = playing_data.player_pos.clamp(program_settings::PLAYER_SIZE * 0.5, 1.0 - program_settings::PLAYER_SIZE * 0.5);
	
	
	// update bullets
	let mut bullet_datas = BulletDataRefs {
		bullets: &mut playing_data.player_bullets,
		player_pos: &playing_data.player_pos,
		player_health: &mut playing_data.player_health,
		enemies: &mut playing_data.enemies,
	};
	update_bullets(&mut bullet_datas, player_bullet_collision);
	
	bullet_datas.bullets = &mut playing_data.enemy_bullets;
	update_bullets(&mut bullet_datas, enemy_bullet_collision);
	
	
	// update pause menu
	let pause_data = &mut playing_data.pause_data;
	let target_transparency = pause_data.is_paused as u8 as f32;
	pause_data.needs_gui_update = false;
	if target_transparency != pause_data.curr_menu_transparency {
		pause_data.needs_gui_update = true;
		let direction = (target_transparency - pause_data.curr_menu_transparency);
		let step = dt / program_settings::PLAYING_PAUSE_MENU_FADE_DURATION.as_secs_f32();
		pause_data.curr_menu_transparency += direction.clamp(-step, step);
		pause_data.curr_menu_transparency = pause_data.curr_menu_transparency.clamp(0., 1.);
	}
	
	
	
	// update gui
	update_gui_data(playing_data)?;
	
	
	
	// process inputs after main update
	if app.keyboard.was_pressed(KeyCode::Escape) {
		let pause_data = &mut playing_data.pause_data;
		pause_data.is_paused = !pause_data.is_paused;
	}
	if app.mouse.left_was_pressed() {
		let mouse_pos = app.mouse.position().to_vec2() / program_data.last_screen_size.as_vec2();
		let new_bullet = Bullet::new(playing_data.player_pos, mouse_pos, program_settings::PLAYER_BULLET_SPEED);
		playing_data.player_bullets.push(new_bullet);
	}
	
	
	
	Ok(())
}





pub fn update_gui_data(playing_data: &mut PlayingData) -> Result<()> {
	const GUI_ERROR_MESSAGE: &str = "Could not update gui";
	let gui = &mut playing_data.gui;
	
	// update pause menu
	let pause_data = &playing_data.pause_data;
	if pause_data.needs_gui_update {
		let curr_menu_alpha = pause_data.curr_menu_transparency;
		gui::utils::apply_to_all_children(gui.child_mut_or_message("main_pause_menu", GUI_ERROR_MESSAGE)?, |element| {
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





pub fn set_click_fns(gui: &mut GuiElement<CustomGuiData>) -> Result<()> {
	
	fn set_click_fn(element: &mut GuiElement<CustomGuiData>, click_fn: fn(&mut ProgramData) -> Result<()>) {
		element.custom_data.click_fn = Some(click_fn);
	}
	const GUI_ERROR_MESSAGE: &str = "Could not add click function";
	
	let main_pause_menu = gui.child_mut_or_message("main_pause_menu", GUI_ERROR_MESSAGE)?; {
		
		fn resume_button(program_data: &mut ProgramData) -> Result<()> {
			program_data.playing_data.pause_data.is_paused = false;
			Ok(())
		}
		set_click_fn(main_pause_menu.child_mut_or_message("resume_button", GUI_ERROR_MESSAGE)?, resume_button);
		
		fn exit_button(program_data: &mut ProgramData) -> Result<()> {
			program_data.mode = ProgramMode::MainMenu;
			program_data.main_menu_data.reset();
			Ok(())
		}
		set_click_fn(main_pause_menu.child_mut_or_message("exit_button", GUI_ERROR_MESSAGE)?, exit_button);
		
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
