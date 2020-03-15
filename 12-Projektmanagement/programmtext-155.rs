mod geometrie {
	pub struct Quadrat {
		pub breite: u32,
		hoehe: u32
	}

	impl Quadrat {
		pub fn create(laenge: u32) -> Quadrat {
			Quadrat {breite: laenge, hoehe: laenge}
		}
	}
}

pub fn some_function() {
	let r = geometrie::Quadrat::create(10);
	println!("{}", r.hoehe); // Fehler!
}