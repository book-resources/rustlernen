#[derive(Debug)]
struct Rechteck {
	breite: u32,
	hoehe: u32
}

impl Rechteck {
	fn create(breite: u32, hoehe: u32) -> Rechteck {
		Rechteck {breite, hoehe}
	}
}

fn main() {
	let r1 = Rechteck::create(10, 15);

	println!("r1 = {:?}", r1);
}