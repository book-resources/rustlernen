fn main() {
	let arr = [1,2,3,4,5];
	let slice = &arr[2..];
	println!("slice = {:?}", slice);
}