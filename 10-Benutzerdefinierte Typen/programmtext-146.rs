fn main() {
	let a = Some(10);

	match a {
		Some(b) if b % 2 == 0 => println!("a ist gerade"),
		Some(b) => println!("a ist ungerade"),
		None => ()
	}
}