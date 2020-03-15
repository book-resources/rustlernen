pub trait Print {
	fn print(&self) {
		println!("Hallo, Welt!");
	}
}

pub struct Rechteck {
	breite: u32,
	hoehe: u32
}

impl Print for Rechteck {

}

fn main() {
	let r = Rechteck { breite: 10, hoehe: 15 };

	r.print();
}