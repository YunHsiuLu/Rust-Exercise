use std::fmt::{self, Formatter, Display};

#[derive(Debug)]
struct Color {
	red: u8,
	green: u8,
	blue: u8,
}

impl Display for Color {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		
		write!(f, "RGB ({:>3}, {:>3}, {:>3}) 0x{:0>2X}{:0>2X}{:0>2X}", self.red, self.green, self.blue, self.red, self.green, self.blue)
	}
}

fn main() {
	for color in [
		Color { red: 128, green: 255, blue:90 },
		Color { red: 0, green: 3, blue: 254 },
		Color { red: 0, green: 0, blue: 0 },
	] {
		println!("{}", color);
	}
}
