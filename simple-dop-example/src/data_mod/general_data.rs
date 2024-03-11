use std::collections::HashMap;

use crate::prelude::*;
use notan::random::rand::thread_rng;



#[derive(AppState)]
pub struct ProgramData {
	pub exit: bool,
	
	pub textures: Textures,
	pub rendering_font: RenderingFont,
	pub positioning_font: PositioningFont,
	pub last_screen_size: UVec2,
	
	pub mode: ProgramMode,
	pub main_menu_data: MainMenuData,
	pub playing_data: PlayingData,
	
}



pub struct Textures {
	pub player: Texture,
	pub enemy: Texture,
	pub player_bullet: Texture,
	pub enemy_bullet: Texture,
}



pub enum ProgramMode {
	MainMenu,
	Playing,
}



pub struct MainMenuData {
	pub gui: GuiElement<CustomGuiData>,
	pub enter_time: Instant,
}

impl MainMenuData {
	pub fn empty() -> Self {
		Self {
			gui: GuiElement::new("", vec!(), HashMap::new(), &mut CustomGuiData::default),
			enter_time: Instant::now(),
		}
	}
	pub fn reset(&mut self) {
		self.enter_time = Instant::now();
	}
}



pub struct PlayingData {
	
	pub gui: GuiElement<CustomGuiData>,
	pub start_time: Instant,
	pub pause_data: PausedData,
	
	pub player_pos: Vec2,
	pub player_vel: Vec2,
	pub player_health: f32,
	pub score: usize,
	
	pub enemies: Vec<Enemy>,
	pub player_bullets: Vec<Bullet>,
	pub enemy_bullets: Vec<Bullet>,
	
}

impl PlayingData {
	pub fn empty() -> Self {
		Self {
			
			gui: GuiElement::new("", vec!(), HashMap::new(), &mut CustomGuiData::default),
			start_time: Instant::now(),
			pause_data: PausedData::new(),
			
			player_pos: Vec2::default(),
			player_vel: Vec2::default(),
			player_health: 0.,
			score: 0,
			
			enemies: vec!(),
			player_bullets: vec!(),
			enemy_bullets: vec!(),
			
		}
	}
	pub fn reset(&mut self) {
		
		self.start_time = Instant::now();
		self.pause_data = PausedData::new();
		
		self.player_pos = Vec2::new(0.5, 0.5);
		self.player_vel = Vec2::new(0., 0.);
		self.player_health = 1.;
		self.score = 0;
		
		self.enemies = vec!();
		self.player_bullets = vec!();
		self.enemy_bullets = vec!();
		
	}
}



pub struct PausedData {
	pub is_paused: bool,
	pub curr_menu_transparency: f32,
	pub needs_gui_update: bool,
}

impl PausedData {
	pub fn new() -> Self {
		Self {
			is_paused: false,
			curr_menu_transparency: 0.,
			needs_gui_update: false,
		}
	}
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
