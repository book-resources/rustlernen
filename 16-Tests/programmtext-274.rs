fn square(x: i32) -> i32 {
	x * x
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn it_squares() {
		assert_eq!(144, square(12));
	}
}