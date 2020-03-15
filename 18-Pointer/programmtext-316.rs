#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil
}

fn main() {
	use crate::List::{Cons, Nil};

	let list2 = Nil;
	let list3 = Cons(10, Box::new(Nil));
	let list4 = Cons(10, Box::new(Cons(15, Box::new(Nil))));

	println!("list2 = {:?}", list2);
	println!("list3 = {:?}", list3);
	println!("list4 = {:?}", list4);
}