mod outer_module {
	pub mod inner_module {
		pub fn some_function() {}
	}
}

use crate::outer_module::inner_module::some_function;

fn call_some_function() {
	some_function();
}