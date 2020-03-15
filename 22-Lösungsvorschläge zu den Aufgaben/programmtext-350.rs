fn main() {
	println!("Eingabe 'Anna': {}", is_palindrome("Anna"));
	println!("Eingabe 'hallo': {}", is_palindrome("hallo"));
}

fn reverse(s: &str) -> String {
	let mut result = String::new();

	for c in s.chars() {
		result = format!("{}{}", c, result);
	}

	result
}

fn is_palindrome(s: &str) -> bool {
	s.to_lowercase() == reverse(s).to_lowercase()
}