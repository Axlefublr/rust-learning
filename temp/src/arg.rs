use std::env;
use crate::temperature::TempInput;
use crate::temperature::TempTypes;

pub fn get_temp_struct() -> TempInput {
	let input = get_input();
	validate_input(input)
}

fn get_input() -> String {
	let mut args = env::args();
	args.next().unwrap();
	let input = args.next().expect("Please type in the temperature like this: 35c");
	input
}

fn validate_input(input: String) -> TempInput {
	let (index, chr) = input.trim().char_indices().rev().next().unwrap();
	let temp_degree = &input[..index];
	let temp_letter = chr;
	let temp_degree: f64 = temp_degree.parse().expect("Please type in a valid number like 35c");
	let temp_type = match temp_letter {
		'c' => TempTypes::Celcius,
		'f' => TempTypes::Farenheight,
		_ => panic!("Please type in a valid temperature type letter like 35c or 42f")
	};
	TempInput {
		temp_type,
		temp_degree
	}
}