#[derive(Debug)]
struct Rechteck {
	breite: u32,
	hoehe: u32
}

impl Rechteck {
	fn flaecheninhalt(&self) -> u32 {
		self.breite * self.hoehe
	}
}

fn main() {
	let r = Rechteck {breite: 10, hoehe: 15};
	println!("Flaecheninhalt von r: {}", r.flaecheninhalt());
}