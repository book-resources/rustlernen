fn main() {
	let vector = vec![1, 2, 3, 4];
	println!("Durchschnitt von [1, 2, 3, 4]: {}", avg(vector));
}

fn avg(vector: Vec<i32>) -> f32 {
	(vector.iter().sum::<i32>() as f32) / (vector.len() as f32)
}