//! Basic example.
// extern crate bid_tracker_rs;

// use bid_tracker_rs;
use std::{boxed::Box, error::Error, process};

fn example() -> Result<(), Box<dyn Error>> {
	println!("Hello world");
	Ok(())
}

fn main() {
	if let Err(err) = example() {
		println!("error running example: {}", err);
		process::exit(1);
	}
}
