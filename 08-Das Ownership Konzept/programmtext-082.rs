fn main() {
	let mut s = String::from("Hallo");
	append_str(&mut s);
	println!("s = {}", s);
}

fn append_str(some_str: &mut String) {
	some_str.push_str(", Welt!");
}