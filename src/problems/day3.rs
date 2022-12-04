use crate::utils;

pub fn execute(){
	let content = utils::read_file(utils::get_file("day3_input.txt"));

	let lines: Vec<&str> = content.split("\n").collect();
	let each_lines: Vec<u32> = lines[..].into_iter().map(|x| {
		let middle = x.len()/2;
		let first_part = &x[0..middle];
		let second_part = &x[middle..];
		let common_item = get_common_item2(first_part, second_part);
		if common_item.is_none(){
			return 0;
		}
		let common_item_value = get_common_item_value(common_item.unwrap());
		println!("{} : {}", common_item.unwrap(), common_item_value);
		return common_item_value;
	}).collect();
	println!("Part 1: {}", each_lines.into_iter().sum::<u32>());


	let mut index = 0;
	let limit = lines[..].len();
	let mut total_size_group = 0;
	loop {
		if index >= limit {
			break;
		}
		let first_part = lines.get(index).unwrap();
		let second_part = lines.get(index + 1).unwrap();
		let third_part = lines.get(index + 2).unwrap();
		let common_item = get_common_item3(first_part, second_part, third_part);
		if common_item.is_some(){
			total_size_group += get_common_item_value(common_item.unwrap());
		}
		index += 3;
	}
	println!("Part 2: {}", total_size_group);
}

fn get_common_item3(first_part: &str, second_part: &str, third_part: &str) -> Option<char> {
	for c in first_part.chars() {
		if second_part.find(c).is_some() && third_part.find(c).is_some(){
			return Some(c);
		}
	}
	return None;
}

fn get_common_item2(first_part: &str, second_part: &str) -> Option<char> {
	for c in first_part.chars() {
		let found = second_part.find(c);
		if found.is_some(){
			return Some(c);
		}
	}
	return None;
}

fn get_common_item_value(common_item: char) -> u32 {
	match common_item {
		'a'..='z' => (common_item as u32) - 96,
		'A'..='Z' => (common_item as u32) - 38,
		_ => 0
	}
}