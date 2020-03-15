fn main() {
	let v = vec![1, 2, 3, 4];
	println!("Eingabe [1, 2, 3, 4]: {}", squared_even_sum(v));
}

fn squared_even_sum(vector: Vec<i32>) -> i32 {
	vector.iter().filter(|e| *e % 2 == 0).map(|e| e * e).sum()
}