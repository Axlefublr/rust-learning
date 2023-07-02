pub enum TempTypes {
	Farenheight,
	Celcius
}

pub struct TempInput {
	pub temp_type: TempTypes,
	pub temp_degree: f64
}

pub fn get_converted_temp(temperature: TempInput) -> String {
	match temperature.temp_type {
		TempTypes::Celcius => format_faren(temperature.temp_degree),
		TempTypes::Farenheight => format_celcius(temperature.temp_degree)
	}
}

fn format_faren(celcius: f64) -> String {
	format!("{:.1}°F", convert_to_faren(celcius))
}

fn format_celcius(faren: f64) -> String {
	format!("{:.1}°C", convert_to_celcius(faren))
}

fn convert_to_faren(celcius: f64) -> f64 {
	celcius * 1.8 + 32.0
}

fn convert_to_celcius(faren: f64) -> f64 {
	(faren - 32.0) / 1.8
}