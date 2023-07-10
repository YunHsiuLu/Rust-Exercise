#![allow(dead_code)] 
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]

use std::fmt::{self, Display, Formatter, Result};

fn reverse(pair: (f32, f32)) -> (f32, f32) {
	let (first, second) = pair;

	(second, first)
}

#[derive(Debug, Clone)]
struct Matrix(f32, f32, f32, f32);
impl Display for Matrix {
	fn fmt(&self, f: &mut Formatter) -> Result {
		write!(f, "( {} {} )\n( {} {} )", self.0, self.1, self.2, self.3)
	}
}

fn Transpose(matrix: Matrix) -> Matrix {
	let (t0, t1) = reverse((matrix.1, matrix.2));

	return Matrix(matrix.0, t0, t1, matrix.3);
}

fn main() {
	let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
	let transpose = Transpose(matrix.clone());
	println!("Matrix:\n{}", matrix);
	println!("Transpose:\n{}", transpose);

}
