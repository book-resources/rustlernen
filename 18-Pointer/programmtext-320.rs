use std::ops::Deref;

struct NewBox<T>(T);

impl<T> Deref for NewBox<T> {
	type Target = T;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

fn main() {
	let a = 5;
	let b = NewBox(a);

	assert_eq!(a, *b);
}