enum List {
    Cons(i32, List),
    Nil
}

fn main() {
	use crate::List::{Cons, Nil};
	
	let list1 = Cons(10, Cons(15, Cons(20, Cons(25, Nil))));
	let list2 = Nil;
	let list3 = Cons(10, Nil);
	let list4 = Cons(10, Cons(15, Nil));
}