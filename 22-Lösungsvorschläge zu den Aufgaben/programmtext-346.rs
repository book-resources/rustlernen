fn main() {
	println!("Ist 5 gerade? {}", is_even(5));
	println!("Ist 10 gerade? {}", is_even(10));
}

fn is_even(a: i32) -> bool {
	a % 2 == 0
}