fn main() {
	println!("'a' in 'Hallo'? {}", contains("Hallo", 'a'));
	println!("'b' in 'Hallo'? {}", contains("Hallo", 'b'));
}

fn contains(s: &str, c: char) -> bool {
	for ch in s.chars() {
		if ch == c {
			return true;
		}
	}

	false
}