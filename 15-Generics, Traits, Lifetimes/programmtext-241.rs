struct SomeStruct<T, U> {
	a: T,
	b: U
}

impl<T, U> SomeStruct<T, U> {
	fn add<V, W>(&self, other: SomeStruct<V, W>) -> (&T, &U, V, W) {
		(&self.a, &self.b, other.a, other.b)
	}
}

fn main() {
	let s1 = SomeStruct { a: 5, b: 10.5 };
	let s2 = SomeStruct { a: true, b: 'c' };

	let (a, b, c, d) = s1.add(s2);
	println!("a = {}, b = {}, c = {}, d = {}", a, b, c, d);
}