use std::mem;

fn main() {
	let xs: [i32; 5] = [1,2,3,4,5];
	let empty_array: [u32; 0] = [];
	assert_eq!(&empty_array, &[]);
	assert_eq!(&empty_array, &[][..]);

	for i in 0..xs.len()+1 {
		match xs.get(i) {
			Some(xval) => println!("{}: {}", i, xval),
			None => println!("Slow down! {} is too far!", i),
		}
	}
	// match = switch in c
	// xs.get() will return a value, which will be `a value` or `None`
	// using this way, we can save the computing resource without using if ...

}
