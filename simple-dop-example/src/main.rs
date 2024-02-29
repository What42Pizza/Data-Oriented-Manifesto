// Started:      24/02/27  (though started as a copy of another notan program)
// Last updated: 24/02/27



#![allow(unused)]
#![warn(unused_must_use)]
#![allow(clippy::new_without_default)]



pub mod program_settings {
	use crate::prelude::*;
	
	pub const MAIN_MENU_WAIT_DURATION: Duration = Duration::from_secs(2);
	
	pub const PLAYER_SIZE: Vec2 = Vec2::new(0.05, 0.05);
	pub const PLAYER_ACCELERATION: f32 = 1.7;
	pub const PLAYER_DRAG_COEF: f32 = 0.01;
	pub const PLAYER_BULLET_SPEED: f32 = 0.01;
	
	pub const PLAYER_HIT_RADIUS: f32 = 0.05;
	pub const ENEMY_HIT_RADIUS: f32 = 0.05;
	
}



use prelude::*;
use notan::draw::{DrawConfig, CreateFont};

use crate::gui_mod::internal_prelude::RealArea;



pub mod update_mod;
pub mod render;
pub mod data_mod;
pub mod gui_mod;
pub mod gui_integration_mod;
pub mod utils;
pub mod custom_impls;



pub mod prelude {
	
	pub use crate::{
		data_mod::general_data::*,
		update_mod::*,
		utils::*,
		program_settings,
		gui_mod::{self, prelude::*},
		custom_impls::*,
	};
	pub use crate::gui_integration_mod::prelude::*;
	pub use std::{path::*, time::{Duration, Instant}};
	pub use std::{error::Error as StdError, result::Result as StdResult};
	
	pub use notan::{prelude::*, math::{Vec2, IVec2, UVec2}};
	pub use notan::draw::Font as RenderingFont;
	pub use ab_glyph::*;
	pub use ab_glyph::FontVec as PositioningFont;
	pub use anyhow::*;
	pub use derive_is_enum_variant::is_enum_variant;
	pub use array_init::array_init;
	
}



//#[notan_main]
fn main() -> Result<(), String> {
	let win_config = WindowConfig::new()
		.set_resizable(true)
		.set_size(1280, 720)
		.set_vsync(true);
	
	notan::init_with(init_wrapper)
		.add_config(win_config)
		.add_config(DrawConfig)
		.update(update_mod::update::update_wrapper)
		.draw(render::render_wrapper)
		.build()
}





pub fn init_wrapper(gfx: &mut Graphics) -> ProgramData {
	init(gfx).unwrap_or_else(|err| {
		println!("\n\n\n======== FATAL ERROR DURING INIT ========");
		for err in err.chain() {
			println!("{err}");
		}
		println!();
		panic!("[see above errors]");
	})
}

pub fn init(gfx: &mut Graphics) -> Result<ProgramData> {
	
	
	
	// General Data
	
	// load textures
	let textures = Textures {
		player       : load_texture(get_program_file_path("assets/textures/player.png"       ), gfx)?,
		enemy        : load_texture(get_program_file_path("assets/textures/enemy.png"        ), gfx)?,
		player_bullet: load_texture(get_program_file_path("assets/textures/player_bullet.png"), gfx)?,
		enemy_bullet : load_texture(get_program_file_path("assets/textures/enemy_bullet.png" ), gfx)?,
	};
	
	// load font
	const FONT_BYTES: &[u8] = include_bytes!("../assets/Ubuntu-B.ttf");
	let rendering_font = gfx.create_font(FONT_BYTES).unwrap();
	let positioning_font = PositioningFont::try_from_vec(FONT_BYTES.to_vec()).unwrap();
	
	// load gui
	let gui = init_gui(&textures)?;
	
	//fn print_gui(element: &GuiElement<CustomGuiData>) {
	//	println!("{}", element.name);
	//	for child in &element.children_by_layer {
	//		print_gui(child);
	//	}
	//}
	//print_gui(&gui);
	
	//let order_tree = gui_mod::render::get_element_ordering(&gui, (1280, 720));
	//for (i, node) in order_tree.iter().enumerate() {
	//	println!("{i}: {node:?}");
	//}
	//panic!();
	
	
	
	let output = ProgramData {
		
		exit: false,
		
		gui,
		textures,
		rendering_font,
		positioning_font,
		last_screen_size: gfx.size().to_uvec2(),
		
		mode: ProgramMode::MainMenu (MainMenuData::new()),
		
	};
	
	Ok(output)
}
