enum Event {
	KeyPress(char),
	Click {x: i32, y: i32}
}

fn main() {
	use crate::Event::{KeyPress, Click};

	let e = KeyPress('c');
	match e {
		KeyPress(c) => println!("KeyPress"),
		Click {x, y} => println!("Click")
	}
}