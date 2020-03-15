struct SomeStruct<T> {
	a: T,
	b: T
}

impl<T> SomeStruct<T> {
	fn some_method(&self) -> (&T, &T) {
		(&self.a, &self.b)
	}
}

fn main() {
	let s = SomeStruct { a: 5, b: 10 };
	let (a, b) = s.some_method();
	println!("s.a = {}, s.b = {}", a, b);
}