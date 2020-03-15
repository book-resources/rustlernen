fn first<'a, T>(x: &'a Vec<T>) -> &'a T {
	&x[0]
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	#[should_panic]
	fn it_panics() {
		let v: Vec<i32> = Vec::new();
		first(&v);
	}
}