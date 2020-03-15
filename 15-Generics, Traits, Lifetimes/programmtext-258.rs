fn main() {
	let list1 = vec![1,2,3,4,5];
	println!("max1 = {}", max(&list1));

	let list2 = vec![6,7,8];
	println!("max2 = {}", max(&list2));
}

fn max<T: PartialOrd>(list: &[T]) -> T {
	let mut max = list[0];
	for &num in list {
		if max < num {
			max = num;
		}
	}

	max
}