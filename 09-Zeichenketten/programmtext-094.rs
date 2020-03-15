fn main() {
	let s1 = String::from("Hallo");
	let s2 = "Welt";
	
	use_str(&s1[..]);
	use_str(s2);
}

fn use_str(some_str: &str) {
	println!("some_str = {}", some_str);
}