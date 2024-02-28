use crate::gui_mod::internal_prelude::*;
use std::error::Error;





pub trait RenderFn<CustomData, RenderingData> {
	fn render_element(element: &GuiElement<CustomData>, pos: (i32, i32), size: (u32, u32), rendering_data: &mut RenderingData) -> Result<(), Box<dyn Error>>;
}



pub fn render_gui_element<CustomData, RenderingData, RenderFnImpl: RenderFn<CustomData, RenderingData>>(
	element: &GuiElement<CustomData>,
	rendering_data: &mut RenderingData,
	errors: &mut Vec<Box<dyn Error>>,
) {
	if !element.enabled {return;}
	if element.visible {
		let (pos, size) = element.latest_real_area.get_absolute();
		let result = RenderFnImpl::render_element(element, pos, size, rendering_data);
		if let Err(error) = result {
			errors.push(error);
		}
	}
	for child in &element.children_by_layer {
		render_gui_element::<CustomData, RenderingData, RenderFnImpl>(child, rendering_data, errors);
	}
}
