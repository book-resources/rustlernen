struct SomeNewType {
	text: String
}

impl Drop for SomeNewType {
	fn drop(&mut self) {
		println!("Drop fuer {}.", self.text);
	}
}

fn main() {
	let a = SomeNewType { text: String::from("Hallo") };
	let b = SomeNewType { text: String::from("Welt") };
	println!("Der Scope endet gleich..");
}