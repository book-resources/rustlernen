struct Point(i32, i32);

fn main() {
	let p = Point(0, 1);
	println!("({}, {})", p.0, p.1);
}