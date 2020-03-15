mod geometrie {
	pub struct Rechteck {
		pub breite: u32,
		pub hoehe: u32
	}
}

pub fn some_function() {
	let r = geometrie::Rechteck {breite: 10, hoehe: 15};
}