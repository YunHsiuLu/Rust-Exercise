use std::fmt; // Import `fmt`

// Define a structure where the fields are nameable for comparison.

#[derive(Debug)]
struct Complex {
	real: f64,
	imag: f64,
}

impl fmt::Display for Complex {
	fn fmt(&self, f: &mut fmt:: Formatter) -> fmt::Result {
		// Customize so only `x` and `y` are denoted.
		write!(f, "{} + {}i", self.real, self.imag)
	}
}

fn main() {
	let complex = Complex { real: 3.3, imag: 7.2 };

	println!("Compare complex:");
	println!("Display: {}", complex);
	println!("Debug: {:?}", complex);
}
