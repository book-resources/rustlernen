#[derive(Debug)]
struct Rechteck {
	breite: u32,
	hoehe: u32
}

impl Rechteck {
	fn umschliesst(&self, other: &Rechteck) -> bool {
		self.breite > other.breite && self.hoehe > other.hoehe
	}
}

fn main() {
	let r1 = Rechteck {breite: 10, hoehe: 15};
	let r2 = Rechteck {breite: 5, hoehe: 10};

	println!("Passt r2 in r1? {}", r1.umschliesst(&r2));
	println!("Passt r1 in r2? {}", r2.umschliesst(&r1));
}