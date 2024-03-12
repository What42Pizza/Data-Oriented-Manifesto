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
	pub pause_data: PauseData,
	
	pub player_pos: Vec2,
	pub player_vel: Vec2,
	pub player_health: f32,
	pub score: usize,
	
	pub enemies: Vec<Enemy>,
	pub enemy_spawn_timer: LoopingTimer,
	pub player_bullets: Vec<Bullet>,
	pub enemy_bullets: Vec<Bullet>,
	
}

impl PlayingData {
	pub fn empty() -> Self {
		Self {
			
			gui: GuiElement::new("", vec!(), HashMap::new(), &mut CustomGuiData::default),
			start_time: Instant::now(),
			pause_data: PauseData::new(),
			
			player_pos: Vec2::default(),
			player_vel: Vec2::default(),
			player_health: 0.,
			score: 0,
			
			enemies: vec!(),
			enemy_spawn_timer: LoopingTimer::new(Duration::from_secs(0)),
			
			player_bullets: vec!(),
			enemy_bullets: vec!(),
			
		}
	}
	pub fn reset(&mut self) {
		
		self.start_time = Instant::now();
		self.pause_data = PauseData::new();
		
		self.player_pos = Vec2::new(0.5, 0.5);
		self.player_vel = Vec2::new(0., 0.);
		self.player_health = 1.;
		self.score = 0;
		
		self.enemies = vec!();
		self.enemy_spawn_timer = LoopingTimer::new(Duration::from_secs_f32(program_settings::ENEMY_SPAWN_WAIT_SECS));
		self.player_bullets = vec!();
		self.enemy_bullets = vec!();
		
	}
}



pub struct PauseData {
	pub is_paused: bool,
	pub curr_menu_transparency: f32,
	pub needs_gui_update: bool,
}

impl PauseData {
	pub fn new() -> Self {
		Self {
			is_paused: false,
			curr_menu_transparency: 0.,
			needs_gui_update: false,
		}
	}
}



#[derive(Debug)]
pub struct Enemy {
	pub pos: Vec2,
	pub vel: Vec2,
	pub shoot_timer: LoopingTimer,
}

impl Enemy {
	pub fn new() -> Self {
		let (pos_x, pos_y) = thread_rng().gen();
		let (vel_x, vel_y) = thread_rng().gen();
		let timer = LoopingTimer::new(Duration::from_secs_f32(program_settings::ENEMY_SHOOT_WAIT_SECS));
		Self {
			pos: Vec2::new(pos_x, pos_y),
			vel: Vec2::new(vel_x, vel_y).normalize() * program_settings::ENEMY_SPEED,
			shoot_timer: timer,
		}
	}
}



#[derive(Debug)]
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





#[derive(Debug)]
pub struct LoopingTimer {
	pub time_left: Duration,
	pub starting_duration: Duration,
}

pub type TimerDidLoop = bool;

impl LoopingTimer {
	pub fn new(start: Duration) -> Self {
		Self {
			time_left: start,
			starting_duration: start,
		}
	}
	pub fn tick(&mut self, dt: f32) -> TimerDidLoop {
		let dt = Duration::from_secs_f32(dt);
		let looping = self.time_left < dt;
		if looping {
			self.time_left += self.starting_duration;
		}
		self.time_left -= dt; // do after restart to avoid negatives
		looping
	}
}
