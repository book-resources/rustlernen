fn main() {
	println!("Eingabe 'anna': {}", is_palindrome("anna"));
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
	s.to_string() == reverse(s)
}