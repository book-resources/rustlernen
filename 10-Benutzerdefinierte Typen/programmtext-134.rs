enum Event {
	KeyPress(char),
	Click {x: i32, y: i32}
}

impl Event {
	fn some_method(&self) {
		match self {
			Event::KeyPress(c) => println!("Taste {} gedrueckt", c),
			Event::Click {x, y} => println!("Klick bei ({}, {})", x, y)
		}
	}
}

fn main() {
	let a = Event::Click {x: 120, y: 100};
	a.some_method();

	let b = Event::KeyPress('b');
	b.some_method();
}