#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]

use std::fmt::{self, Display, Formatter, Result};

#[derive(Debug)]
struct Point {
	x: f32,
	y: f32,
}

struct Rectangle {
	top_left: Point,
	bottom_right: Point,
}

impl Display for Rectangle {
	fn fmt(&self, f: &mut Formatter) -> Result {
		write!(f, "top left point:     ({}, {})\n", self.top_left.x, self.top_left.y)?;
		write!(f, "bottom right point: ({}, {})", self.bottom_right.x, self.bottom_right.y)
	}
}

fn rect_area(rect: Rectangle) -> f32 {
	let length = rect.bottom_right.x - rect.top_left.x;
	let width  = rect.top_left.y - rect.bottom_right.y;

	return length*width;
}

fn main() {
	let origin: Point = Point { x: 0., y: 0. };
	let p1 = Point { x: origin.x, y: 5. };
	let p2 = Point { x: 8., y: origin.y };
	let rect: Rectangle = Rectangle { top_left: p1, bottom_right: p2};
	println!("Rectangle points:\n{}", rect);
	let Area = rect_area(rect);
	println!("Area is : {}", Area);
}
