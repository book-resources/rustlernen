use std::fmt::Display;

struct SomeStruct<T> {
	a: T,
	b: T
}

impl<T> SomeStruct<T> {
	fn new(a: T, b: T) -> Self {
		Self { a, b }
	}
}

impl<T: PartialOrd + Display> SomeStruct<T> {
	fn max(&self) {
		if &self.a > &self.b {
			println!("max = {}", &self.a);
		} else {
			println!("max = {}", &self.b);
		}
	}
}

fn main() {
	let s = SomeStruct::new(5, 10);
	s.max();
}