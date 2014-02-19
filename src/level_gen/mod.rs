use rsfml::graphics::{Quads,Texture,Vertex,VertexArray};
use rsfml::graphics::rc::{RenderStates};
use rsfml::system::Vector2f;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Level{
	map:VertexArray,
	state:RenderStates,
}

impl Level {
	pub fn new(win_size:Vector2f,texture_size:Vector2f, map_set:[int, ..500], tile_set:&str) -> Level{
		let v_array = create_map(win_size,texture_size, map_set);
		let ren_state = apply_texture(tile_set);
		Level{map:v_array,state:ren_state}
	}
}

fn apply_texture(tile_set:&str) -> RenderStates{
	let m_texture = match Texture::new_from_file(tile_set){
		Some(m_texture)	=>	Rc::new(RefCell::new(m_texture)),
		None()			=>	fail!(~"yah")
	};
	let mut m_ren_state = RenderStates::default();
	m_ren_state.texture = Some(m_texture);
	m_ren_state
}

fn create_map(win_size:Vector2f,texture_size:Vector2f, map_set:[int, ..500]) -> VertexArray {
	let mut test = VertexArray::new().expect("blip");
	test.set_primitive_type(Quads);
	let mut i:uint = 0; let mut j:uint = 0;
	let mut texture_pos = Vector2f::new(0.0,0.0);
	
	while i < win_size.x as uint {
		while j < win_size.y as uint {
			// Find Which tile to get from tileset
			let tile_num = map_set[i + j * win_size.x as uint];
			match tile_num {
				0	=>	{texture_pos = Vector2f::new(128.0,0.0)}
				1	=>	{texture_pos = Vector2f::new(0.0,0.0)}
				2   =>	{texture_pos = Vector2f::new(64.0,0.0)}
				_	=>	{texture_pos = Vector2f::new(384.0,0.0)}
			}
			// VertexArray Position
			let top_left_pos = &Vector2f::new(i as f32 * texture_size.x,j as f32 * texture_size.y + 320.0);let top_right_pos = &Vector2f::new((i as f32 + 1.0) * texture_size.x,j as f32 * texture_size.y + 320.0);
			let bot_left_pos = &Vector2f::new(i as f32 * texture_size.x,(j as f32 + 1.0) * texture_size.y +320.0);let bot_right_pos = &Vector2f::new((i as f32 + 1.0) * texture_size.x,(j as f32 + 1.0) * texture_size.y + 320.0);
			// VertexArray Texture Cooridnates
			let top_left_tex = &Vector2f::new(texture_pos.x,texture_pos.y);let top_right_tex = &Vector2f::new(texture_pos.x + texture_size.x,texture_pos.y);
			let bot_left_tex = &Vector2f::new(texture_pos.x, texture_pos.y + texture_size.y);let bot_right_tex = &Vector2f::new(texture_pos.x + texture_size.x,texture_pos.y + texture_size.y);
			// Instert Vertexes
			test.append(&Vertex::new_with_pos_coords(top_left_pos,top_left_tex));
			test.append(&Vertex::new_with_pos_coords(top_right_pos,top_right_tex));
			test.append(&Vertex::new_with_pos_coords(bot_right_pos,bot_right_tex));
			test.append(&Vertex::new_with_pos_coords(bot_left_pos,bot_left_tex));
			j += 1;	// Increment j
		}
		j = 0;	// Reset j ^_^
		i += 1;	// Increment i
	}
	test
}

