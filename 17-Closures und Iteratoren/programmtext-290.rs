fn main() {
	let square_1 = |val: i32| -> i32 { val * val };
	let square_2 = |val| { val * val };
	let square_3 = |val| val * val;

	println!("square_1(5) = {}", square_1(5));
	println!("square_2(5) = {}", square_2(5));
	println!("square_3(5) = {}", square_3(5));
}