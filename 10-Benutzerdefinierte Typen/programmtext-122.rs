#[derive(Debug)]
struct Point {
	x: i32,
	y: i32
}

fn main() {
	let p = Point {x: 0, y: 1};
	println!("p = {:?}", p);
}