use std::fs;
use std::env;

pub fn read_file(path: String) -> String {
	let contents = fs::read_to_string(path)
		.expect("Failed to read the file");

	contents.replace("\r\n", "\n")
}

fn current_path() -> String {
	env::current_dir().unwrap().display().to_string()
}

pub fn get_file(filename: &str) -> String {
	let curr_path = current_path();
	let sep = std::path::MAIN_SEPARATOR;
	let file_path = format!("{curr_path}{sep}src{sep}problems{sep}{filename}");
	file_path
}