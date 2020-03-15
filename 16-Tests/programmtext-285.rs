pub fn add(x: i32, y: i32) -> i32 {
	x + y
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn add_one() {
		assert_eq!(add(5, 1), 6);
	}
}