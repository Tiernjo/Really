use rsfml::graphics::{Color,Quads,RenderStates,Texture,Vertex,VertexArray};
use rsfml::system::Vector2f;
mod window;

fn test_vertex(win_size:Vector2f,texture_size:Vector2f) -> VertexArray {
	let mut test = VertexArray::new().expect("blip");
	test.set_primitive_type(Quads);
	let mut i:uint = 0; let mut j:uint = 0;
	
	while i < win_size.x as uint {
		while j < win_size.y as uint {
			// VertexArray Position
			let top_left = &Vector2f::new(i as f32 * texture_size.x,j as f32 * texture_size.y);let top_right = &Vector2f::new((i as f32 + 1.0) * texture_size.x,j as f32 * texture_size.y);
			let bot_left = &Vector2f::new(i as f32 * 64.0,(j as f32 + 1.0) * texture_size.x);let bot_right = &Vector2f::new((i as f32 + 1.0) * texture_size.x,(j as f32 + 1.0) * 64.0);

			test.append(&Vertex::new_with_pos_coords(top_left,&Vector2f::new(0.0,0.0)));
			test.append(&Vertex::new_with_pos_coords(top_right,&Vector2f::new(64.0,0.0)));
			test.append(&Vertex::new_with_pos_coords(bot_right,&Vector2f::new(64.0,64.0)));
			test.append(&Vertex::new_with_pos_coords(bot_left,&Vector2f::new(0.0,64.0)));
			j += 1;
		}
		j = 0;
		i += 1;
	}
	test
}

pub fn test_game() {
	let main_win = &mut window::new(512,256);
	let texture_size = Vector2f::new(64.0,64.0);
	let win_size = Vector2f::new(8.0,4.0);
	let test = test_vertex(win_size,texture_size);
	let m_texture = Texture::new_from_file("../img/tiles.png").expect("texture");
	let mut m_ren_state = RenderStates::default();
	m_ren_state.texture = Some(&m_texture);

	while main_win.is_open() {
		window::check_close(main_win);

		main_win.clear(&Color::black());
		main_win.draw_with_renderstates(&test, &mut m_ren_state);
		main_win.display()
	}
}
