fn main() {
	println!("15 > 10? {}", is_greater_than(15, 10));
	println!("10 > 10? {}", is_greater_than(10, 10));
}

fn is_greater_than(a: i32, b: i32) -> bool {
	a > b
}