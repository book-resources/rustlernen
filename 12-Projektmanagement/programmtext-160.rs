mod some_module {
	pub enum Event {
		KeyPress(char),
		Click {x: i32, y: i32}
	}
}

use crate::some_module::Event;

fn some_function() {
	let c = Event::Click {x: 100, y: 150};
}