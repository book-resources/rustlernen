mod outer_module {
	pub mod inner_module {
		pub fn some_function() {}
	}
}

use self::outer_module::inner_module;

fn call_some_function() {
	inner_module::some_function();
}