mod some_module {
	mod some_inner_module {
		fn call_some_function() {
			// Aufruf mittels relativem Pfad
			super::super::some_function();
		}
	}
}

fn some_function() {}