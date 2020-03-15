#[derive(Debug)]
struct Mensch {
	name: String,
	alter: u8,
	geschlecht: char
}

impl Mensch {
	fn create(name: &str, alter: u8, geschlecht: char) -> Mensch {
		Mensch { name: name.to_string(), alter, geschlecht }
	}
}

fn main() {
	let p = Mensch::create("Peter", 29, 'm');
	println!("p = {:?}", p);
}