fn some_function(val: i32) {
	if val <= 0 {
		panic!("val is too small! Must be greater than 0.");
	} else if val >= 1000 {
		panic!("val is too big! Must be smaller than 1000.");
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	#[should_panic(expected="val is too small!")]
	fn too_small() {
		some_function(-5);
	}

	#[test]
	#[should_panic(expected="val is too big!")]
	fn too_big() {
		some_function(5000);
	}
}