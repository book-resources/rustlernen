fn square(x: i32) -> i32 {
	x * x
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn it_squares() -> Result<(), String> {
		if square(12) == 144 {
			Ok(())
		} else {
			Err(String::from("12 * 12 sollte 144 ergeben"))
		}
	}
}