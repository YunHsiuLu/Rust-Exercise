fn main() {
	println!("Running formatprint.rs......");
	let x = 37829;
	println!("Base 10 (decimal):     {}", x);
	println!("Base 2 (binary):       {:b}", x);
	println!("Base 8 (octal):        {:o}", x);
	println!("Base 16 (hexadecimal): {:x}", x);
	println!("Base 16 (HEXADECIMAL): {:X}", x);
	println!();
	println!("{{number:>5}} : {number:>5}", number=1);
	println!("{{number:0<5}}: {number:0<5}", number=1);
	println!("{{number:0>width$}}: {number:0>width$}", number=1, width=5);
	println!();
	/*
	let number: f64 = 1.0;
	let width: usize = 5;
	println!("{number:>width$}");
	*/
}
