fn square(x: i32) -> i32 {
	x * x
}

pub fn add(x: i32, y: i32) -> i32 {
	x + y
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn square_5() {
		assert_eq!(square(5), 25);
	}
}