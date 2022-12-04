mod problems;
mod utils;

use std::io;

fn main() {
	println!("Advent Of Code 2022");
	println!("Select the problem: ");
	let mut input_text = String::new();

	io::stdin()
		.read_line(&mut input_text)
		.expect("Failed to read User Input");

		match input_text.trim().parse::<i32>().unwrap() {
			1 => problems::day1::execute(),
			2 => problems::day2::execute(),
			3 => problems::day3::execute(),
			_ => println!("No such problem")
		};
}
