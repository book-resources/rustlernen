fn square(x: i32) -> i32 {
	println!("x = {}", x);
	x * x
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn it_works() {
		assert_eq!(square(5), 25);
	}

	#[test]
	fn it_fails() {
		assert_eq!(square(4), 25);
	}
}