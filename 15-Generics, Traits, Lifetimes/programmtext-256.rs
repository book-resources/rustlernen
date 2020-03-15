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
	let r = some_function(10, 15);
	r.print();
}

fn some_function(a: u32, b: u32) -> impl Print {
	Rechteck { breite: a, hoehe: b }
}