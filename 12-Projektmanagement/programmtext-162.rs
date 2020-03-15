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

fn some_function() {
	let c = some_module::Event::Click {x: 100, y: 150};
	let k = other_module::Event::KeyPress('c');
}