use crate::prelude::*;
use notan::draw::*;



pub struct GuiRenderingData<'a> {
	pub draw: &'a mut Draw,
	pub textures: &'a Textures,
	pub rendering_font: RenderingFont,
	pub positioning_font: &'a PositioningFont,
}



pub struct GuiRenderFn;

impl<'a> gui::render::RenderFn<CustomGuiData, GuiRenderingData<'a>> for GuiRenderFn {
	fn render_element(element: &GuiElement<CustomGuiData>, element_area: gui::data::RealArea, rendering_data: &mut GuiRenderingData) -> Result<()> {
		let (pos, size) = element_area.get_absolute();
		let (pos, size) = (pos.to_f32(), size.to_f32());
		
		// background
		if element.has_background {
			rendering_data.draw.rect(pos, size).color(element.background_color.as_notan_color());
		}
		
		// image
		if let Some(image) = &element.custom_data.image {
			rendering_data.draw.image(image)
				.position(pos.0, pos.1)
				.size(size.0, size.1);
		}
		
		// border
		if element.has_border {
			let width = element.border_width * size.1;
			let half_width = width * 0.5;
			let border_top_left = (pos.0 - half_width, pos.1 - half_width);
			let border_bottom_right = (pos.0 + size.0 + half_width, pos.1 + size.1 + half_width);
			rendering_data.draw.rect(border_top_left, (size.0 + width, width)).color(element.border_color.as_notan_color());
			rendering_data.draw.rect(border_top_left, (width, size.1 + width)).color(element.border_color.as_notan_color());
			rendering_data.draw.rect(border_bottom_right, (-(size.0 + width), -width)).color(element.border_color.as_notan_color());
			rendering_data.draw.rect(border_bottom_right, (-width, -(size.1 + width))).color(element.border_color.as_notan_color());
		}
		
		// text
		if element.has_text {
			let text_size = element.text_size * element_area.height * element_area.screen_size.1 as f32 * 0.7;
			let spacings = gui::utils::get_char_spacings(element, rendering_data.positioning_font, text_size);
			let mut line_start_poss = Vec::with_capacity(element.text.len());
			for (i, line) in element.text.iter().enumerate() {
				let line_width = *spacings[i].last().unwrap_or(&0.);
				let line_start_pos = gui::utils::get_line_start_pos(element, pos, size, text_size, i, line_width);
				line_start_poss.push(line_start_pos);
				if line.is_empty() {continue;}
				rendering_data.draw.text(&rendering_data.rendering_font, line)
					.position(line_start_pos.0, line_start_pos.1)
					.color(element.text_color.as_notan_color())
					.size(text_size);
			}
			
			// cursor
			if element.is_editing_text {
				let mut cursor_pos = line_start_poss[element.cursor_y];
				if element.cursor_x > 0 {
					cursor_pos.0 += spacings[element.cursor_y][element.cursor_x - 1];
					cursor_pos.0 -= text_size * 0.05;
				}
				rendering_data.draw
					.rect(cursor_pos, (text_size * 0.08, text_size))
					.color(Color::new(0.0, 0.0, 0.0, 1.0));
			}
			
		}
		
		StdResult::Ok(())
	}
}
