fn main() {
	let a = squared(10);
	println!("a = {}", a);
}

fn squared(a: i32) -> i32 {
	a * a
}