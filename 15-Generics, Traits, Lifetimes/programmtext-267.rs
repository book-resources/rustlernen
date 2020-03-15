struct SomeStruct<'a> {
	a: &'a str
}

impl<'a> SomeStruct<'a> {
	fn some_method(&self) -> &str {
		&self.a
	}
}

fn main() {
	let s = SomeStruct { a: "Hallo" };
	let a = s.some_method();
	println!("a = {}", a);
}