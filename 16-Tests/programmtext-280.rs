fn square(x: i32) -> i32 {
	x * x
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn square_minus_10() {
		assert_eq!(square(-10), 100);
	}

	#[test]
	fn square_minus_5() {
		assert_eq!(square(-5), 25);
	}

	#[test]
	fn square_0() {
		assert_eq!(square(0), 0);
	}
}