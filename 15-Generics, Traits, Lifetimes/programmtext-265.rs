fn main() {
	let s1 = "Hallo, Welt!";
	let return_value;

	{
		let s2 = String::from("Hallo!");
		return_value = some_function(s1, s2.as_str());
	}

	println!("{}", return_value);
}

fn some_function<'a>(s1: &'a str, s2: &'a str) -> &'a str {
	if s1.len() > s2.len() {
		s1
	} else {
		s2
	}
}