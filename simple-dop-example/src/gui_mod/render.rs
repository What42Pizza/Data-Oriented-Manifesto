use crate::gui_mod::internal_prelude::*;



pub trait RenderFn<CustomData, RenderingData> {
	fn render_element(element: &GuiElement<CustomData>, real_area: RealArea, rendering_data: &mut RenderingData) -> Result<()>;
}



pub fn run_render_fns<CustomData, RenderingData, RenderFnImpl: RenderFn<CustomData, RenderingData>>(
	base_element: &GuiElement<CustomData>,
	screen_size: (u32, u32),
	rendering_data: &mut RenderingData,
) -> std::result::Result<(), Vec<Error>> {
	let mut errors = vec!();
	
	let order_tree = prepare_element_rendering(base_element, screen_size);
	let mut branches_to_render = vec!(0);
	loop {
		
		let Some(curr_branch_index) = branches_to_render.pop() else {break;};
		let curr_branch = &order_tree[curr_branch_index];
		match curr_branch {
			OrderingNode::Branch {left_index, right_index, ..} => {
				branches_to_render.push(*right_index);
				branches_to_render.push(*left_index);
			}
			OrderingNode::Element {element, render_data} => {
				let result = RenderFnImpl::render_element(element, render_data.real_area, rendering_data);
				if let Err(err) = result {
					errors.push(err);
				}
			}
		}
		
	}
	
	if errors.is_empty() {
		std::result::Result::Ok(())
	} else {
		std::result::Result::Err(errors)
	}
}





pub enum OrderingNode<'a, CustomData> {
	Element {
		element: &'a GuiElement<CustomData>,
		render_data: ElementRenderData
	},
	Branch {
		left_index: usize, left_max: f32,
		right_index: usize, right_min: f32,
		prev_equal_was_right: bool
	},
}

impl<'a, CustomData> std::fmt::Debug for OrderingNode<'a, CustomData> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Element {element, ..} => write!(f, "element \"{}\"", element.name),
			Self::Branch {left_index, left_max, right_index, right_min, ..} => write!(f, "branch {{{left_index}, {left_max}, {right_index}, {right_min}}}"),
		}
	}
}

#[derive(Copy, Clone)]
pub struct ElementRenderData {
	pub real_area: RealArea,
}

impl ElementRenderData {
	pub fn new(screen_size: (u32, u32)) -> Self {
		Self {
			real_area: RealArea::new(screen_size),
		}
	}
}



pub fn prepare_element_rendering<CustomData>(base_element: &GuiElement<CustomData>, screen_size: (u32, u32)) -> Vec<OrderingNode<CustomData>> {
	
	let base_render_data = ElementRenderData {
		real_area: RealArea::new(screen_size),
	};
	let mut elements_to_insert = vec!();
	for child in &base_element.children_by_layer {
		elements_to_insert.push((child, ElementRenderData {
			real_area: base_render_data.real_area.get_sub_area_for_element(child),
		}));
	}
	
	let base_element_node = OrderingNode::Element {
		element: base_element,
		render_data: ElementRenderData::new(screen_size),
	};
	let mut output = vec!(base_element_node);
	
	loop {
		
		let Some((curr_element, render_data)) = elements_to_insert.pop() else {break;};
		if !curr_element.enabled {continue;}
		if curr_element.visible {
			insert_into_order_tree(curr_element, render_data, &mut output);
		}
		for child in &curr_element.children_by_layer {
			elements_to_insert.push((child, ElementRenderData {
				real_area: render_data.real_area.get_sub_area_for_element(child),
			}));
		}
		
	}
	output
}



pub fn insert_into_order_tree<'a, CustomData>(element: &'a GuiElement<CustomData>, render_data: ElementRenderData, order_tree: &mut Vec<OrderingNode<'a, CustomData>>) {
	let element_priority = element.render_priority;
	let mut i = 0;
	// TODO: traversed_nodes probably isn't needed now that OrderingNode::Branch has prev_equal_was_right
	let mut traversed_nodes = vec!();
	loop {
		match &mut order_tree[i] {
			
			OrderingNode::Branch {left_index, left_max, right_index, right_min, prev_equal_was_right} => {
				if element_priority <= *left_max {
					traversed_nodes.push((i, false));
					i = *left_index;
					continue;
				}
				if element_priority >= *right_min {
					traversed_nodes.push((i, true));
					i = *right_index;
					continue;
				}
				if *prev_equal_was_right {
					*prev_equal_was_right = false;
					traversed_nodes.push((i, false));
					i = *left_index;
				} else {
					*prev_equal_was_right = true;
					traversed_nodes.push((i, true));
					i = *right_index;
				}
			}
			
			OrderingNode::Element {element: prev_element, render_data: prev_render_data} => {
				let (left_element, left_render_data, right_element, right_render_data) = if element_priority < prev_element.render_priority {
					(element, render_data, *prev_element, *prev_render_data)
				} else {
					(*prev_element, *prev_render_data, element, render_data)
				};
				let left_index = order_tree.len();
				order_tree.push(OrderingNode::Element {element: left_element, render_data: left_render_data});
				let right_index = order_tree.len();
				order_tree.push(OrderingNode::Element {element: right_element, render_data: right_render_data});
				order_tree[i] = OrderingNode::Branch {
					left_index, left_max: left_element.render_priority,
					right_index, right_min: right_element.render_priority,
					prev_equal_was_right: false
				};
				break;
			}
			
		}
	}
	for nodes in traversed_nodes.windows(2) {
		let OrderingNode::Branch {left_max: child_left_max, right_min: child_right_min, ..} = &order_tree[nodes[1].0] else {unreachable!()};
		let (min, max) = (child_right_min.min(*child_left_max), child_right_min.max(*child_left_max));
		let OrderingNode::Branch {left_max: curr_left_max, right_min: curr_right_min, ..} = &mut order_tree[nodes[0].0] else {unreachable!()};
		let is_right = nodes[0].1;
		if is_right {
			*curr_right_min = min;
		} else {
			*curr_left_max = max;
		}
	}
}
