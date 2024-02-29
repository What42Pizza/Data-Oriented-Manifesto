use crate::gui_mod::internal_prelude::*;
use std::error::Error;





pub trait RenderFn<CustomData, RenderingData> {
	fn render_element(element: &GuiElement<CustomData>, real_area: RealArea, rendering_data: &mut RenderingData) -> Result<(), Box<dyn Error>>;
}



pub fn render_gui_element<CustomData, RenderingData, RenderFnImpl: RenderFn<CustomData, RenderingData>>(
	element: &GuiElement<CustomData>,
	element_area: RealArea,
	rendering_data: &mut RenderingData,
	errors: &mut Vec<Box<dyn Error>>,
) {
	if !element.enabled {return;}
	if element.visible {
		let result = RenderFnImpl::render_element(element, element_area, rendering_data);
		if let Err(error) = result {
			errors.push(error);
		}
	}
	for child in &element.children_by_layer {
		let child_area: RealArea = element_area.get_sub_area_for_element(child);
		render_gui_element::<CustomData, RenderingData, RenderFnImpl>(child, child_area, rendering_data, errors);
	}
}
