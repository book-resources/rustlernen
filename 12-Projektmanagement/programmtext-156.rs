mod some_module {
	pub enum SomeEnum {
		A,
		B
	}
}

pub fn some_function() {
	let a = some_module::SomeEnum::A;
}