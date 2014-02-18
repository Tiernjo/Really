use rsfml::graphics::{RenderWindow};
use rsfml::window::{Close,ContextSettings,event,VideoMode};

pub fn new(height:uint,width:uint) -> RenderWindow{
	let win_set = ContextSettings::default();
	let win_mode = VideoMode::new_init(height,width,32);

	let window = match RenderWindow::new(win_mode,"Really",Close,&win_set){
		Some(window)	=>	window,
		None()			=>	fail!(~"Error creating window")
	};
	window
}

pub fn check_close(window:&mut RenderWindow) {
	loop {
		match window.poll_event() {
			event::Closed	=>	window.close(),
			event::NoEvent	=>	break,
			_				=>	{}
		}
	}
}