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

	fn ist_volljaehrig(&self) -> bool {
		self.alter >= 18
	}
}

fn main() {
	let p = Mensch::create("Peter", 29, 'm');
	println!("Ist Peter volljaehrig? {}", p.ist_volljaehrig());
}