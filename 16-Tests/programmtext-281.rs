fn square(x: i32) -> i32 {
	x * x
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	#[ignore]
	fn square_5() {
		assert_eq!(square(5), 25);
	}

	#[test]
	fn square_minus_5() {
		assert_eq!(square(-5), 25);
	}
}