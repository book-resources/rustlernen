fn main() {
	let list = vec![1,2,3,4,5];

	let mut max = list[0];
	for &num in &list {
		if max < num {
			max = num;
		}
	}

	println!("max = {}", max);
}