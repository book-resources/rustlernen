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

pub struct Punkt {
	x: i32,
	y: i32
}

impl Print for Punkt {
	fn print(&self) {
		println!("x = {}, y = {}", &self.x, &self.y);
	}
}

fn main() {
	let r = Rechteck { breite: 10, hoehe: 15 };
	let p = Punkt { x: 5, y: 10 };

	r.print();
	p.print();
}