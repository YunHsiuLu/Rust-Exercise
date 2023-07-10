fn reverse(pair: (i32, bool)) -> (bool, i32) {
	let (int_param, bool_param) = pair;

	(bool_param, int_param)
}

fn main() {
	let long_tuple = (1u8, 2u16, 3u32, 4u64, -1i8, -2i16, -3i32, -4i64, 0.1f32, 0.2f64, 'a', true);
	println!("Long tuple: {:?}", long_tuple);
	println!("Long tuple first value: {}", long_tuple.0);
	println!("Long tuple second value: {}", long_tuple.1);

	let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);

	println!("tuple of tuples: {:?}", tuple_of_tuples);

	// But too long tuples cannot be printed (more than 12).

	let pair = (1, true);
	println!("Pair is {:?}", pair);

	println!("The reversed pair is {:?}", reverse(pair));

	// To create one element tuples, the comma is required to tell them apart from a literal surrounded by parentheses.
	println!("One element tuple: {:?}", (5u32,));
	println!("Just an integer: {:?}", (5u32));

	// Tuples can be destructured to create bindings
	let tuple = (1, "Hello", 4.5, true);

	let (a, b, c, d) = tuple;
	println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);
}
