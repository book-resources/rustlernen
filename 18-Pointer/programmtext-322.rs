fn main() {
	let b = Box::new(String::from("Hallo, Welt!"));
	print_str(&b);
}

fn print_str(s: &str) {
	println!("s = {}", s);
}