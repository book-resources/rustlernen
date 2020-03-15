use std::fmt::Display;

trait Print {
	fn print(&self);
}

impl<T: Display> Print for T {
	fn print(&self) {
		println!("{}", &self);
	}
}

fn main() {
	let a = 5;
	a.print();
}