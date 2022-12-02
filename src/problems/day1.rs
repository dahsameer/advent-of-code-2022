use crate::utils;
use itertools::sorted;

pub fn execute(){
	let content = utils::read_file(utils::get_file("day1_input.txt"));
	let calories_arr: Vec<i32> = content.split("\n\n")
		.map(|x| x.split("\n").map(|s| s.parse::<i32>().unwrap()).sum())
		.collect();
	
	let sorted_arr: Vec<i32> = sorted(calories_arr).rev().collect();
	
	println!("Top 1: {}", sorted_arr.get(0).unwrap());
	println!("Top 3: {}", sorted_arr[0..3].into_iter().sum::<i32>());
}