struct Rechteck {
	breite: u32,
	hoehe: u32
}

fn main() {
	let r = Rechteck {breite: 10, hoehe: 15};

	match r {
		Rechteck {breite: _, hoehe: 10} => println!("breite egal, hoehe = 10"),
		Rechteck {breite: 10, hoehe: _} => println!("breite = 10, hoehe egal"),
		Rechteck {breite: _, hoehe: _} => println!("breite und hoehe egal")
	}
}