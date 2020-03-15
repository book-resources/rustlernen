struct Struct {
	a: i32,
	b: i32,
	c: i32,
	d: i32
}

fn main() {
	let s = Struct {a: 1, b: 2, c: 3, d: 4};

	match s {
		Struct {a: 1, ..} => println!("a = 1"),
		Struct {..} => println!("a != 1")
	}
}