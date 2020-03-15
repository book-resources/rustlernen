mod some_module {
	fn call_some_function() {
		// Aufruf mittels relativem Pfad
		super::some_function();
	}
}

fn some_function() {}