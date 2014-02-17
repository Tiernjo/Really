use rsfml::graphics::{Color,IntRect,Texture,Vertex,VertexArray};
use rsfml::graphics::rc::Sprite;
use rsfml::system::{Vector2f,Vector2u};
use std::cell::RefCell;
use std::rc::Rc;
mod window;

struct MultiSprite {
	top_left:Sprite,
	top_mid:Sprite,
	top_right:Sprite,
}	

impl MultiSprite {
	fn new() -> MultiSprite {
		let texture: Rc<RefCell<Texture>> = match Texture::new_from_file("../img/tiles.png"){
			Some(texture)	=>	Rc::new(RefCell::new(texture)),
			None()			=>	fail!(~"tex of MultiSprite")
		};
		let tl_bounds = IntRect::new(128,0,64,64);
		let tm_bounds = IntRect::new(0,0,64,64);
		let tr_bounds = IntRect::new(64,0,64,64);
		let mut tl = Sprite::new_with_texture(texture.clone()).expect("tl");
		let mut tm = Sprite::new_with_texture(texture.clone()).expect("mid");
		let mut tr = Sprite::new_with_texture(texture.clone()).expect("tr");
		tl.set_texture_rect(&tl_bounds);
		tm.set_texture_rect(&tm_bounds);
		tr.set_texture_rect(&tr_bounds);
		MultiSprite{
			top_left:tl,
			top_mid:tm,
			top_right:tr,
		}
	}
}

pub fn test_game() {
	let main_win = &mut window::new(512,256);
	let mut test = MultiSprite::new();
	test.top_mid.set_position(&Vector2f::new(64.0,0.0));
	test.top_right.set_position(&Vector2f::new(128.0,0.0));
	while main_win.is_open() {
		window::check_close(main_win);

		main_win.clear(&Color::black());
		main_win.draw(&test.top_left);main_win.draw(&test.top_mid);main_win.draw(&test.top_right);
		main_win.display()
	}
}
