use rsfml::graphics::{Color,Texture,Vertex,VertexArray};
use rsfml::graphics::Quads;
use rsfml::system::{Vector2f,Vector2u};
mod window;

struct TestGrid{
	body:VertexArray,
	skin:Texture,
}
impl TestGrid {
	fn new(tileset:&str,tile_size:Vector2u,tiles:~[uint],width:uint,height:uint) -> TestGrid{
		let mut texture = match Texture::new_from_file(tileset) {
			Some(texture)	=>	texture,
			None()			=>	fail!(~"Error creating tile texture")
		};
		// New array vertex
		let mut v_array = match VertexArray::new(){
			Some(v_array)	=>	v_array,
			None()	=>	fail!(~"Error creating vertex array")
		};
		v_array.set_primitive_type(Quads);
		v_array.resize(width * height);

		let mut i:uint = 0;let mut j:uint = 0;

		while i < width {
			while j < height {
				
				// Current tile number
				let tile_number = tiles[i + j];

				// find position
				//let tu = tile_number  / ();
				//let tv = tile_number / ();
				// Pointer to current tile quad
				let mut quad:&Vertex = v_array.get_vertex(i + j * width);
				// Set positions
				v_array.get_vertex(0+i).position = Vector2f::new(i as f32, i as f32);
				v_array.get_vertex(1+i).position = Vector2f::new(i as f32 + tile_size.x as f32,i as f32);
				v_array.get_vertex(2+i).position = Vector2f::new(i as f32 + tile_size.x as f32,i as f32 + tile_size.y as f32);
				v_array.get_vertex(4+i).position = Vector2f::new(i as f32,i as f32 + tile_size.y as f32);
				// Set Tex loc
				v_array.get_vertex(0+i).tex_coords = Vector2f::new(i as f32 + tile_size.x as f32,i as f32 + tile_size.y as f32);
				v_array.get_vertex(1+i).tex_coords = Vector2f::new(i as f32 + tile_size.x as f32,i as f32 + tile_size.y as f32);
				v_array.get_vertex(2+i).tex_coords = Vector2f::new(i as f32 + tile_size.x as f32,i as f32 + tile_size.y as f32);
				v_array.get_vertex(4+i).tex_coords = Vector2f::new(i as f32 + tile_size.x as f32,i as f32 + tile_size.y as f32);

				v_array.get_vertex(0+i).color = Color::red();
				v_array.get_vertex(1+i).color = Color::blue();

				j += 1;
			}
			i += 1;
		}
		for v in v_array.iter() {
			println!("{:?}", v)
		}
		TestGrid{body:v_array,skin:texture}
	}
}
pub fn test_game() {
	let main_win = &mut window::new(512,256);
	let tile_size = Vector2u::new(64,64);
	let all_tiles = ~[0,0,0,0];
	let grid = TestGrid::new("../img/ground.png",tile_size,all_tiles,2,2);
	while main_win.is_open() {
		window::check_close(main_win);

		main_win.clear(&Color::black());
		main_win.draw(&grid.body);
		main_win.display()
	}
}
