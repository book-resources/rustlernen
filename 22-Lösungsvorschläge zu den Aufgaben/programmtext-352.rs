fn main() {
	println!("Eingabe ('Hallo', 3): {}", repeat("Hallo", 3));
	println!("Eingabe ('Hallo', 5): {}", repeat("Hallo", 5));
}

fn repeat(s: &str, x: u32) -> String {
	let mut result = String::new();

	for _ in 1..=x {
		result = format!("{}{}", result, s);
	}

	result
}