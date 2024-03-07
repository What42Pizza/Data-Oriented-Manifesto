use crate::prelude::*;



pub trait Vec2Fns {
	fn from_tuple(from: (f32, f32)) -> Self;
}

impl Vec2Fns for Vec2 {
	fn from_tuple(from: (f32, f32)) -> Self {
		Self::new(from.0, from.1)
	}
}



pub trait UVec2Fns {
	fn to_tuple(self) -> (u32, u32);
}

impl UVec2Fns for UVec2 {
	fn to_tuple(self) -> (u32, u32) {
		(self.x, self.y)
	}
}



pub trait TupleF32F32Fns {
	fn to_vec2(self) -> Vec2;
}

impl TupleF32F32Fns for (f32, f32) {
	fn to_vec2(self) -> Vec2 {
		Vec2::new(self.0, self.1)
	}
}



pub trait TupleU32U32Fns {
	fn to_vec2(self) -> Vec2;
	fn to_ivec2(self) -> IVec2;
	fn to_uvec2(self) -> UVec2;
}

impl TupleU32U32Fns for (u32, u32) {
	fn to_vec2(self) -> Vec2 {
		Vec2::new(self.0 as f32, self.1 as f32)
	}
	fn to_ivec2(self) -> IVec2 {
		IVec2::new(self.0 as i32, self.1 as i32)
	}
	fn to_uvec2(self) -> UVec2 {
		UVec2::new(self.0, self.1)
	}
}



pub trait GuiColor {
	fn as_notan_color(&self) -> notan::prelude::Color;
}

impl GuiColor for crate::gui_mod::data::Color {
	fn as_notan_color(&self) -> notan::prelude::Color {
		notan::prelude::Color::new(self.r, self.g, self.b, self.a)
	}
}



pub trait TupleF32F32 {
	fn to_i32(self) -> (i32, i32);
}

impl TupleF32F32 for (f32, f32) {
	fn to_i32(self) -> (i32, i32) {
		(self.0 as i32, self.1 as i32)
	}
}



pub trait TupleI32I32 {
	fn to_f32(self) -> (f32, f32);
}

impl TupleI32I32 for (i32, i32) {
	fn to_f32(self) -> (f32, f32) {
		(self.0 as f32, self.1 as f32)
	}
}



pub trait TupleU32U32 {
	fn to_f32(self) -> (f32, f32);
}

impl TupleU32U32 for (u32, u32) {
	fn to_f32(self) -> (f32, f32) {
		(self.0 as f32, self.1 as f32)
	}
}
