mod other_file;

pub use crate::other_file::other_module;

pub fn some_function() {
	other_module::other_function();
}