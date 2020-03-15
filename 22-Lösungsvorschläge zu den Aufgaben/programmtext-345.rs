fn main() {
	let array = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
	println!("Ergebnis: {}", squared_array_sum(&array));
}

fn squared_array_sum(arr: &[i32]) -> i32 {
	let mut sum = 0;

	for e in arr.iter() {
		sum += e * e;
	}

	sum
}