use std::io;

fn main() {
	loop {
		print!("Enter the temperature to convert: ");
		let mut input = String::new();
		io::stdin().read_line(&mut input).unwrap();
		match input[input.len()-1..]
			.trim()
			.to_lowercase()
			.parse::<char>()
			.expect("Put c/f as a marker for the temperature type, right after the number like 35c or 35f")
		{
			'c' => {
				let faren = convert_to_faren(
					input
					.trim()[input.len()..]
					.to_lowercase()
					.parse::<f64>()
					.expect("Type in a number in this format: 35c")
				);
				println!("Your temp in farenheight is: {}f", faren);
			},
			'f' => {
				let celcius = convert_to_celcius(
					input
					.trim()[input.len()..]
					.to_lowercase()
					.parse::<f64>()
					.expect("Type in a number in this format: 35f")
				);
				println!("Your temp in celcius is: {}c", celcius);
			},
			_ => {
				eprintln!("Type in 35c to convert 35 celcius to farenheight");
				eprintln!("or 35f to convert 35 farenheight to celcius");
			}
		}
	}
}

fn convert_to_faren(celcius: f64) -> f64 {
	celcius * 1.8 + 32.0
}

fn convert_to_celcius(faren: f64) -> f64 {
	(faren - 32.0) / 1.8
}