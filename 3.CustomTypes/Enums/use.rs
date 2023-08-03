#![allow(dead_code)] 
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]

enum Status {
	Rich,
	Poor,
}

enum Work {
	Civilian,
	Soldier,
}

fn main() {
	// Explicity `use` each name so they are available without
	// manual scoping.
	use crate::Status::{Poor, Rich};
	// Automatically `use` each name inside `Work`
	use crate::Work::*;

	// Equivalent to `Status::Poor`.
	let status = Poor;
	let work = Civilian;

	match status {
		Rich => println!("The rich have lots of money!"),
		Poor => println!("The poor have no money..."),
	}

	match work {
		Civilian => println!("Civilians work!"),
		Soldier => println!("Solders fight!"),
	}
}
