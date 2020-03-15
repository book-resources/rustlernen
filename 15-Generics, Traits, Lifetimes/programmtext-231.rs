fn main() {
	let list1 = vec![1,2,3,4,5];
	
	let mut max1 = list1[0];
	for &num in &list1 {
		if max1 < num {
			max1 = num;
		}
	}

	println!("max1 = {}", max1);

	let list2 = vec![6,7,8];
	
	let mut max2 = list2[0];
	for &num in &list2 {
		if max2 < num {
			max2 = num;
		}
	}

	println!("max2 = {}", max2);
}