pub trait Print {
	fn print(&self);
}

pub struct Rechteck {
	breite: u32,
	hoehe: u32
}

impl Print for Rechteck {
	fn print(&self) {
		println!("breite={}, hoehe={}", &self.breite, &self.hoehe);
	}
}

fn main() {
	let r = Rechteck { breite: 10, hoehe: 15 };

	some_function(r);
}

fn some_function(a: impl Print) {
	a.print();
}