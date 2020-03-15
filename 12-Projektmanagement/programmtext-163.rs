mod some_module {
	pub enum Event {
		Click {x: i32, y: i32}
	}
}

mod other_module {
	pub enum Event {
		KeyPress(char)
	}
}

use crate::some_module::Event as ClickEvent;
use crate::other_module::Event as KeyEvent;

fn some_function() {
	let c = ClickEvent::Click {x: 100, y: 150};
	let k = KeyEvent::KeyPress('c');
}