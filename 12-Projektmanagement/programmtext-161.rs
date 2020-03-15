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

use crate::some_module::Event;
use crate::other_module::Event; // Fehler!

fn some_function() {
	let c = Event::Click {x: 100, y: 150};
}