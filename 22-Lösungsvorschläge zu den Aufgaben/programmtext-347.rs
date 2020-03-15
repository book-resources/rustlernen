fn main() {
	println!("Hallo <-> {}", reverse("Hallo"));
}

fn reverse(s: &str) -> String {
	let mut result = String::new();

	for c in s.chars() {
		result = format!("{}{}", c, result);
	}

	result
}