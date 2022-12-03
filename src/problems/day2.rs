use std::collections::HashMap;

use crate::utils;

pub fn execute(){
	let content: String = utils::read_file(utils::get_file("day2_input.txt"));
	let choice_score_map = HashMap::from([
		("X", 1), //rock
		("Y", 2), //paper
		("Z", 3)  //scissor
	]);

	let score_arr: Vec<i32> = content.split("\n")
		.map(|x| {
				let val_arr: Vec<&str> = x.trim().split(" ").collect();
				let away_choice = val_arr.get(0).unwrap();
				let home_choice = val_arr.get(1).unwrap();
				let choice_score = choice_score_map.get(home_choice).unwrap();
				let match_score = get_match_score(away_choice, home_choice);
				choice_score + match_score
			}
		).collect();
	
	println!("Part 1 Score: {}", score_arr.into_iter().sum::<i32>());
	let score_arr: Vec<i32> = content.split("\n")
		.map(|x| {
				let val_arr: Vec<&str> = x.trim().split(" ").collect();
				let away_choice = val_arr.get(0).unwrap();
				let what_to_do = val_arr.get(1).unwrap();
				let home_choice = get_winning_home_choice(away_choice, what_to_do);
				let choice_score = choice_score_map.get(&home_choice[..]).unwrap();
				let match_score = get_match_score(away_choice, &home_choice[..]);
				choice_score + match_score
			}
		).collect();
	println!("Part 2 Score: {}", score_arr.into_iter().sum::<i32>());
}

fn get_match_score(away_choice: &str, home_choice: &str) -> i32 {
	match (away_choice, home_choice) {
		("A", "X") => 3,
		("A", "Y") => 6,
		("A", "Z") => 0,
		("B", "X") => 0,
		("B", "Y") => 3,
		("B", "Z") => 6,
		("C", "X") => 6,
		("C", "Y") => 0,
		("C", "Z") => 3,
		_ => 0
	}
}

fn get_winning_home_choice(away_choice: &str, what_to_do: &str) -> String {
	match (away_choice, what_to_do) {
		("A", "X") => String::from("Z"),
		("A", "Y") => String::from("X"),
		("A", "Z") => String::from("Y"),
		("B", "X") => String::from("X"),
		("B", "Y") => String::from("Y"),
		("B", "Z") => String::from("Z"),
		("C", "X") => String::from("Y"),
		("C", "Y") => String::from("Z"),
		("C", "Z") => String::from("X"),
		_ => String::from("_")
	}
}