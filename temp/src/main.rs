mod arg;
mod temperature;

fn main() {
	let temperature = arg::get_temp_struct();
	let converted_temp = temperature::get_converted_temp(temperature);
	println!("{converted_temp}")
}