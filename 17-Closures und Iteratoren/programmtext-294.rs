use std::collections::HashMap;
use std::collections::hash_map::Entry;

struct ClosureCache<T>
	where T: Fn(i32) -> i32
{
	closure: T,
	values: HashMap<i32, i32>
}

impl <T> ClosureCache<T>
	where T: Fn(i32) -> i32
{
	fn new(closure: T) -> ClosureCache<T> {
		ClosureCache {closure, values: HashMap::new()}
	}

	fn calc_value(&mut self, val: i32) -> i32 {
		match self.values.entry(val) {
			Entry::Occupied(e) => *e.into_mut(),
			Entry::Vacant(e) => {
				*e.insert((self.closure)(val))
			}
		}
	}
}

fn main() {
	let mut c = ClosureCache::new(|v| {
		println!("Calculating {} * {}...", v, v);
		v * v
	});

	println!("value = {}", c.calc_value(5));
	println!("value = {}", c.calc_value(10));
	println!("value = {}", c.calc_value(5));
}