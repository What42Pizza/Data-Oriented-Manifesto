use crate::prelude::*;
use notan::draw::{CreateDraw, Draw, DrawImages, DrawShapes};



pub fn render_wrapper(gfx: &mut Graphics, program_data: &mut ProgramData) {
	render(gfx, program_data).unwrap_or_else(|err| {
		println!("\n\n\n======== FATAL ERROR DURING RENDER ========");
		for err in err.chain() {
			println!("{err}");
		}
		println!();
		panic!("[see above errors]");
	})
}

pub fn render(gfx: &mut Graphics, program_data: &mut ProgramData) -> Result<()> {
	
	
	
	// ======== MISC ========
	
	let screen_size = gfx.size().to_uvec2();
	program_data.last_screen_size = screen_size;
	let program_data = &*program_data;
	
	let textures = &program_data.textures;
	
	let mut draw = gfx.create_draw();
	
	
	
	match &program_data.mode {
		
		
		
		ProgramMode::MainMenu => {
			
			draw.clear(Color::from_rgba(0.5, 0.42, 0.42, 1.0));
			
			render_gui(&program_data.main_menu_data.gui, program_data, &mut draw, screen_size)?;
			
		}
		
		
		
		ProgramMode::Playing => {
			let playing_data = &program_data.playing_data;
			
			// background
			draw.clear(Color::BLACK);
			let (on_screen_pos, on_screen_size) = arena_placement_to_screen_placement(
				Vec2::new(0., 0.),
				Vec2::new(1., 1.),
				screen_size
			);
			draw
				.rect(on_screen_pos, on_screen_size)
				.color(Color::from_rgb(0.2, 0.2, 0.2));
			
			// player
			let (on_screen_pos, on_screen_size) = arena_placement_to_screen_placement(
				playing_data.player_pos - program_settings::PLAYER_SIZE * 0.5,
				Vec2::new(program_settings::PLAYER_SIZE, program_settings::PLAYER_SIZE),
				screen_size
			);
			draw
				.image(&program_data.textures.player)
				.position(on_screen_pos.0, on_screen_pos.1)
				.size(on_screen_size.0, on_screen_size.1);
			
			// enemies
			for enemy in &playing_data.enemies {
				let (on_screen_pos, on_screen_size) = arena_placement_to_screen_placement(
					enemy.pos - program_settings::ENEMY_SIZE * 0.5,
					Vec2::new(program_settings::ENEMY_SIZE, program_settings::ENEMY_SIZE),
					screen_size
				);
				draw
					.image(&program_data.textures.enemy)
					.position(on_screen_pos.0, on_screen_pos.1)
					.size(on_screen_size.0, on_screen_size.1);
			}
			
			render_gui(&program_data.playing_data.gui, program_data, &mut draw, screen_size)?;
			
		}
		
		
		
	}
	
	
	
	gfx.render(&draw);
	
	Ok(())
}



pub fn render_gui(gui: &GuiElement<CustomGuiData>, program_data: &ProgramData, draw: &mut Draw, screen_size: UVec2) -> Result<()> {
	
	// render
	let mut render_data = GuiRenderingData {
		draw,
		textures: &program_data.textures,
		rendering_font: program_data.rendering_font,
		positioning_font: &program_data.positioning_font,
	};
	let render_gui_result = gui::render::run_render_fns::<CustomGuiData, GuiRenderingData, GuiRenderFn>(gui, screen_size.to_tuple(), &mut render_data);
	if let StdResult::Err(render_gui_errors) = render_gui_result {
		println!("Errors ocurred while rendering:");
		for error in render_gui_errors {
			println!("{error}");
		}
	}
	
	Ok(())
}



pub fn arena_placement_to_screen_placement(arena_pos: Vec2, arena_size: Vec2, screen_size: UVec2) -> ((f32, f32), (f32, f32)) {
	let top_left = arena_pos_to_screen_pos(arena_pos, screen_size);
	let bottom_right = arena_pos_to_screen_pos(arena_pos + arena_size, screen_size);
	let size = (bottom_right.0 - top_left.0, bottom_right.1 - top_left.1);
	(top_left, size)
}



pub fn arena_pos_to_screen_pos(arena_pos: Vec2, screen_size: UVec2) -> (f32, f32) {
	if screen_size.y < screen_size.x {
		
		let mut screen_pos = (arena_pos * screen_size.y as f32).as_ivec2();
		screen_pos.x += (screen_size.x - screen_size.y) as i32 / 2;
		(screen_pos.x as f32, screen_pos.y as f32)
		
	} else {
		
		let mut screen_pos = (arena_pos * screen_size.x as f32).as_ivec2();
		screen_pos.y += (screen_size.y - screen_size.x) as i32 / 2;
		(screen_pos.x as f32, screen_pos.y as f32)
		
	}
}
