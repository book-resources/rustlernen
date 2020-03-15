struct ClosureCache<T>
	where T: Fn(i32) -> i32
{
	closure: T,
	value: Option<i32>
}