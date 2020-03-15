mod outer_module {
	pub mod inner_module1 {
		pub fn some_function1() {}

		fn some_function2() {}
	}

	mod inner_module2 {
		fn some_function3() {}
	}
}

fn call_some_function() {
	// Aufruf mittels absolutem Pfad
	crate::outer_module::inner_module1::some_function1();
}