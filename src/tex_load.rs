use rsfml::graphics::{IntRect,Texture};
use rsfml::graphics::rc::Sprite;
use std::cell::RefCell;
use std::rc::Rc;

pub struct MultiSprite {
	top_left:Sprite,
	top_mid:Sprite,
	top_right:Sprite,
	mid_left:Sprite,
	mid_mid:Sprite,
	mid_right:Sprite,
}	

impl MultiSprite {
	pub fn new() -> MultiSprite {
		let texture: Rc<RefCell<Texture>> = match Texture::new_from_file("../img/tiles.png"){
			Some(texture)	=>	Rc::new(RefCell::new(texture)),
			None()			=>	fail!(~"tex of MultiSprite")
		};
		let tl_bounds = IntRect::new(128,0,64,64);let tm_bounds = IntRect::new(0,0,64,64);let tr_bounds = IntRect::new(64,0,64,64);
		let ml_bounds = IntRect::new(192,0,64,64);let mm_bounds = IntRect::new(256,0,64,64);let mr_bounds = IntRect::new(320,0,64,64);

		let mut tl = Sprite::new_with_texture(texture.clone()).expect("tl");let mut tm = Sprite::new_with_texture(texture.clone()).expect("tm");let mut tr = Sprite::new_with_texture(texture.clone()).expect("tr");
		let mut ml = Sprite::new_with_texture(texture.clone()).expect("ml");let mut mm = Sprite::new_with_texture(texture.clone()).expect("mm");let mut mr = Sprite::new_with_texture(texture.clone()).expect("mr");
		
		tl.set_texture_rect(&tl_bounds);tm.set_texture_rect(&tm_bounds);tr.set_texture_rect(&tr_bounds);
		ml.set_texture_rect(&ml_bounds);mm.set_texture_rect(&mm_bounds);mr.set_texture_rect(&mr_bounds);

		MultiSprite{
			top_left:tl,
			top_mid:tm,
			top_right:tr,
			mid_left:ml,
			mid_mid:mm,
			mid_right:mr,
		}
	}
}