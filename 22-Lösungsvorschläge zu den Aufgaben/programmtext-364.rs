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

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn reverse_test_1() {
		assert_eq!(reverse("Hallo, Welt!"), "!tleW ,ollaH");
	}

	#[test]
	fn reverse_test_2() {
		assert_eq!(reverse("Dies ist ein Test"), "tseT nie tsi seiD");
	}

	#[test]
	fn reverse_test_3() {
		assert_eq!(reverse("Rust is fun"), "nuf si tsuR");
	}
}