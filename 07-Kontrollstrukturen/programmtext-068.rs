fn main() {
	let array = [1, 2, 3, 4, 5];
	some_function(&array);
}

fn some_function(arr: &[i32]) {
	println!("arr = {:?}", arr);
}