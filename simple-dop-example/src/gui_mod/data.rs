use crate::gui_mod::internal_prelude::*;



pub struct GuiElement<CustomData> {
	
	pub name: String,
	pub render_priority: f64,
	
	pub visible: bool,
	pub enabled: bool,
	pub children_by_layer: Vec<GuiElement<CustomData>>,
	pub children_by_name: HashMap<String, usize>,
	
	pub x: f64,
	pub y: f64,
	pub width: f64,
	pub height: f64,
	pub natural_x: f64,
	pub natural_width: f64,
	
	pub has_background: bool,
	pub background_color: Color,
	
	pub has_border: bool,
	pub border_color: Color,
	pub border_width: f64,
	
	pub has_text: bool,
	pub text_color: Color,
	pub text: Vec<String>,
	pub text_x_align: XAlignment,
	pub text_y_align: YAlignment,
	pub text_size: f64,
	pub is_editing_text: bool,
	pub can_edit_multiline: bool,
	pub return_finishes_editing: bool,
	pub esc_finishes_editing: bool,
	pub cursor_x: usize,
	pub cursor_y: usize,
	pub cursor_target_x: usize,
	
	pub custom_data: CustomData,
	
}

impl<T> GuiElement<T> {
	
	pub fn new(name: impl Into<String>, children_by_layer: Vec<GuiElement<T>>, children_by_name: HashMap<String, usize>, custom_data_fn: &mut impl FnMut() -> T) -> Self {
		Self {
			
			name: name.into(),
			render_priority: 0.,
			visible: true,
			enabled: true,
			children_by_layer,
			children_by_name,
			
			x: 0.,
			y: 0.,
			width: 1.,
			height: 1.,
			natural_x: 0.,
			natural_width: 0.,
			
			has_background: false,
			background_color: Color::new(0.9, 0.9, 0.93, 1.),
			
			has_border: false,
			border_color: Color::new(1., 1., 1., 1.),
			border_width: 0.1,
			
			has_text: false,
			text_color: Color::new(0., 0., 0., 1.),
			text: vec!(String::new()),
			text_x_align: XAlignment::Center,
			text_y_align: YAlignment::Center,
			text_size: 1.,
			is_editing_text: false,
			can_edit_multiline: true,
			return_finishes_editing: true,
			esc_finishes_editing: true,
			cursor_x: 0,
			cursor_y: 0,
			cursor_target_x: 0,
			
			custom_data: custom_data_fn(),
			
		}
	}
	
	
	
	pub fn single_child_option(&self, name: &str) -> Option<&GuiElement<T>> {
		let index = *self.children_by_name.get(name)?;
		Some(&self.children_by_layer[index])
	}
	
	pub fn single_child_mut_option(&mut self, name: &str) -> Option<&mut GuiElement<T>> {
		let index = *self.children_by_name.get(name)?;
		Some(&mut self.children_by_layer[index])
	}
	
	pub fn child_option(&self, name: &str) -> Option<&GuiElement<T>> {
		if let Some(slash_index) = name.find('/') {
			self.single_child_option(&name[..slash_index])?.child_option(&name[slash_index+1..])
		} else {
			self.single_child_option(name)
		}
	}
	
	pub fn child_mut_option(&mut self, name: &str) -> Option<&mut GuiElement<T>> {
		if let Some(slash_index) = name.find('/') {
			self.single_child_mut_option(&name[..slash_index])?.child_mut_option(&name[slash_index+1..])
		} else {
			self.single_child_mut_option(name)
		}
	}
	
	
	
	pub fn single_child_result(&self, name: &str, on_err: &impl Fn() -> Error) -> Result<&GuiElement<T>> {
		self
			.single_child_option(name)
			.ok_or_else(|| Error::msg(format!("Could not find child '{name}'")).context(on_err()))
	}
	
	pub fn single_child_mut_result(&mut self, name: &str, on_err: &impl Fn() -> Error) -> Result<&mut GuiElement<T>> {
		self
			.single_child_mut_option(name)
			.ok_or_else(|| Error::msg(format!("Could not find child '{name}'")).context(on_err()))
	}
	
	pub fn child_result(&self, name: &str, on_err: &impl Fn() -> Error) -> Result<&GuiElement<T>> {
		if let Some(slash_index) = name.find('/') {
			self.single_child_result(&name[..slash_index], on_err)?.child_result(&name[slash_index+1..], on_err)
		} else {
			self.single_child_result(name, on_err)
		}
	}
	
	pub fn child_mut_result(&mut self, name: &str, on_err: &impl Fn() -> Error) -> Result<&mut GuiElement<T>> {
		if let Some(slash_index) = name.find('/') {
			self.single_child_mut_result(&name[..slash_index], on_err)?.child_mut_result(&name[slash_index+1..], on_err)
		} else {
			self.single_child_mut_result(name, on_err)
		}
	}
	
	
	
	pub fn single_child_or_message(&self, name: &str, message: &'static str) -> Result<&GuiElement<T>> {
		self
			.single_child_option(name)
			.ok_or_else(|| Error::msg(format!("Could not find child '{name}'")).context(Error::msg(message)))
	}
	
	pub fn single_child_mut_or_message(&mut self, name: &str, message: &'static str) -> Result<&mut GuiElement<T>> {
		self
			.single_child_mut_option(name)
			.ok_or_else(|| Error::msg(format!("Could not find child '{name}'")).context(Error::msg(message)))
	}
	
	pub fn child_or_message(&self, name: &str, message: &'static str) -> Result<&GuiElement<T>> {
		if let Some(slash_index) = name.find('/') {
			self.single_child_or_message(&name[..slash_index], message)?.child_or_message(&name[slash_index+1..], message)
		} else {
			self.single_child_or_message(name, message)
		}
	}
	
	pub fn child_mut_or_message(&mut self, name: &str, message: &'static str) -> Result<&mut GuiElement<T>> {
		if let Some(slash_index) = name.find('/') {
			self.single_child_mut_or_message(&name[..slash_index], message)?.child_mut_or_message(&name[slash_index+1..], message)
		} else {
			self.single_child_mut_or_message(name, message)
		}
	}
	
}



#[derive(Copy, Clone)]
pub enum XAlignment {
	Left,
	Center,
	Right,
}

#[derive(Copy, Clone)]
pub enum YAlignment {
	Bottom,
	Center,
	Top,
}

pub struct Color {
	pub r: f32,
	pub g: f32,
	pub b: f32,
	pub a: f32,
}

impl Color {
	pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
		Self {
			r,
			g,
			b,
			a,
		}
	}
}





#[derive(Copy, Clone, Default)]
pub struct RealArea {
	pub screen_size: (u32, u32),
	pub x: f64,
	pub y: f64,
	pub width: f64,
	pub height: f64,
}

impl RealArea {
	
	pub fn new(screen_size: (u32, u32)) -> Self {
		Self {
			screen_size,
			x: 0.,
			y: 0.,
			width: 1.,
			height: 1.,
		}
	}
	
	pub fn get_basic_sub_area(&self, x: f64, y: f64, width: f64, height: f64) -> Self {
		Self {
			screen_size: self.screen_size,
			x: self.x + x * self.width,
			y: self.y + y * self.height,
			width:  width  * self.width,
			height: height * self.height,
		}
	}
	
	pub fn get_sub_area_for_element<T>(&self, element: &GuiElement<T>) -> Self {
		self.get_sub_area(element.x, element.y, element.width, element.height, element.natural_x, element.natural_width)
	}
	
	pub fn get_sub_area(&self, x: f64, y: f64, width: f64, height: f64, natural_x: f64, natural_width: f64) -> Self {
		let aspect_ratio = (self.screen_size.0 as f64 / self.screen_size.1 as f64) * (self.width / self.height);
		Self {
			screen_size: self.screen_size,
			x: self.x + x * self.width + natural_x * self.width / aspect_ratio,
			y: self.y + y * self.height,
			width:  width  * self.width + natural_width * self.width / aspect_ratio,
			height: height * self.height,
		}
	}
	
	pub fn get_point(&self, x: f64, y: f64, natural_x: f64) -> (i32, i32) {
		let aspect_ratio = (self.screen_size.0 as f64 / self.screen_size.1 as f64) * (self.width / self.height);
		let mut point_x = self.x + x * self.width + natural_x * self.width / aspect_ratio;
		let mut point_y = self.y + y * self.height;
		point_x *= self.screen_size.0 as f64;
		point_y *= self.screen_size.1 as f64;
		(point_x.round() as i32, point_y.round() as i32)
	}
	
	pub fn contains_point(&self, point: (i32, i32)) -> bool {
		let (pos, size) = self.get_absolute();
		point.0 >= pos.0 &&
			point.0 <= pos.0 + size.0 as i32 &&
			point.1 >= pos.1 &&
			point.1 <= pos.1 + size.1 as i32
	}
	
	pub fn get_absolute(&self) -> ((i32, i32), (u32, u32)) {
		let x = self.x * self.screen_size.0 as f64;
		let y = self.y * self.screen_size.1 as f64;
		let width  = self.width  * self.screen_size.0 as f64;
		let height = self.height * self.screen_size.1 as f64;
		let end_x = x + width;
		let end_y = y + height;
		let final_x = x.round() as i32;
		let final_y = y.round() as i32;
		let final_width = (end_x.round() as i32) - final_x;
		let final_height = (end_y.round() as i32) - final_y;
		((final_x, final_y), (final_width as u32, final_height as u32))
	}
	
}





#[derive(Debug)]
pub struct GuiKeyboardData {
	pub shift_pressed: bool,
	pub control_pressed: bool,
	pub alt_pressed: bool,
	pub typed_text: String,
	pub control_keys_pressed: Vec<ControlKey>,
}

#[derive(Debug)]
pub enum ControlKey {
	
	Up,
	Down,
	Left,
	Right,
	
	Return,
	Delete,
	Backspace,
	End,
	Esc,
	
}
