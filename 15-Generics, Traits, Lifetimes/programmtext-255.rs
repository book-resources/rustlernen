fn some_function<T, U>(a: T, b: U)
	where T: Trait1 + Trait2,
	      U: Trait2 + Trait3
{
	// ...
}