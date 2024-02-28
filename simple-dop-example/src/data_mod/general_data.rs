use crate::prelude::*;
use notan::random::rand::thread_rng;



#[derive(AppState)]
pub struct ProgramData {
	pub exit: bool,
	
	pub gui: GuiElement<CustomGuiData>,
	pub textures: Textures,
	pub rendering_font: RenderingFont,
	pub positioning_font: PositioningFont,
	pub last_screen_size: UVec2,
	
	pub mode: ProgramMode,
	
}



pub struct Textures {
	pub player: Texture,
	pub enemy: Texture,
	pub player_bullet: Texture,
	pub enemy_bullet: Texture,
}



pub enum ProgramMode {
	MainMenu (MainMenuData),
	Playing (PlayingData),
}



pub struct MainMenuData {
	pub enter_time: Instant,
}

impl MainMenuData {
	pub fn new() -> Self {
		Self {
			enter_time: Instant::now(),
		}
	}
}



pub struct PlayingData {
	
	pub start_time: Instant,
	pub paused_data: PausedData,
	
	pub player_pos: Vec2,
	pub player_vel: Vec2,
	pub player_health: f32,
	pub score: usize,
	
	pub enemies: Vec<Enemy>,
	pub player_bullets: Vec<Bullet>,
	pub enemy_bullets: Vec<Bullet>,
	
}

impl PlayingData {
	pub fn new() -> Self {
		Self {
			
			start_time: Instant::now(),
			paused_data: PausedData::Unpaused {fade_out_percent: 1.0},
			
			player_pos: Vec2::new(0.5, 0.5),
			player_vel: Vec2::new(0., 0.),
			player_health: 1.,
			score: 0,
			
			enemies: vec!(),
			player_bullets: vec!(),
			enemy_bullets: vec!(),
			
		}
	}
}



pub enum PausedData {
	Paused {fade_in_percent: f32},
	Unpaused {fade_out_percent: f32},
}



pub struct Enemy {
	pub pos: Vec2,
	pub vel: Vec2,
	pub next_shoot_instant: Instant,
}



pub struct Bullet {
	pub pos: Vec2,
	pub vel: Vec2,
}

impl Bullet {
	pub fn new(start: Vec2, dest: Vec2, speed: f32) -> Self {
		Self {
			pos: start,
			vel: (dest - start).normalize() * speed,
		}
	}
}
