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

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn contains_test_1() {
		assert!(contains("Autoreifen", 'A'));
	}

	#[test]
	fn contains_test_2() {
		assert!(!contains("Autoreifen", 'E'));
	}

	#[test]
	fn contains_test_3() {
		assert!(contains("Autoreifen", 'f'));
	}
}