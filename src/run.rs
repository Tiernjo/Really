use rsfml::graphics::{Color,Quads,RenderStates,Texture,Vertex,VertexArray};
use rsfml::system::Vector2f;
mod window;

fn test_vertex(win_size:Vector2f,texture_size:Vector2f, map_set:[int, ..32]) -> VertexArray {
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
				_	=>	{fail!(~"Setting texture position");}
			}
			// VertexArray Position
			let top_left_pos = &Vector2f::new(i as f32 * texture_size.x,j as f32 * texture_size.y);let top_right_pos = &Vector2f::new((i as f32 + 1.0) * texture_size.x,j as f32 * texture_size.y);
			let bot_left_pos = &Vector2f::new(i as f32 * 64.0,(j as f32 + 1.0) * texture_size.x);let bot_right_pos = &Vector2f::new((i as f32 + 1.0) * texture_size.x,(j as f32 + 1.0) * 64.0);
			// Instert Vertexes
			test.append(&Vertex::new_with_pos_coords(top_left_pos,&Vector2f::new(texture_pos.x,texture_pos.y)));
			test.append(&Vertex::new_with_pos_coords(top_right_pos,&Vector2f::new(texture_pos.x + texture_size.x,texture_pos.y)));
			test.append(&Vertex::new_with_pos_coords(bot_right_pos,&Vector2f::new(texture_pos.x + texture_size.x,texture_pos.y + texture_size.y)));
			test.append(&Vertex::new_with_pos_coords(bot_left_pos,&Vector2f::new(texture_pos.x, texture_pos.y + texture_size.y)));
			j += 1;	// Increment j
		}
		j = 0;	// Reset j ^_^
		i += 1;	// Increment i
	}
	test
}

pub fn test_game() {
	let main_win = &mut window::new(512,256);
	let texture_size = Vector2f::new(64.0,64.0);
	let win_size = Vector2f::new(8.0,4.0);
	let map = [
		0,1,2,0,1,2,0,1,
		2,0,1,2,0,1,2,0,
		1,2,0,1,2,0,1,2,
		0,1,2,0,1,2,0,1,
	];
	let test = test_vertex(win_size,texture_size, map);
	let m_texture = Texture::new_from_file("../img/tiles.png").expect("texture");
	let mut m_ren_state = RenderStates::default();
	m_ren_state.texture = Some(&m_texture);

	while main_win.is_open() {
		window::check_close(main_win);

		main_win.clear(&Color::blue());
		main_win.draw_with_renderstates(&test, &mut m_ren_state);
		main_win.display()
	}
}
