struct ClosureCache<T>
	where T: Fn(i32) -> i32
{
	closure: T,
	value: Option<i32>
}

impl <T> ClosureCache<T>
	where T: Fn(i32) -> i32
{
	fn new(closure: T) -> ClosureCache<T> {
		ClosureCache {closure, value: None}
	}

	fn calc_value(&mut self, val: i32) -> i32 {
		match self.value {
			Some(v) => v,
			None => {
				let v = (self.closure)(val);
				self.value = Some(v);
				v
			}
		}
	}
}