fn main() {
	let b = Box::new("Hallo, Welt!");
	print_str(&b);
}

fn print_str(s: &str) {
	println!("s = {}", s);
}