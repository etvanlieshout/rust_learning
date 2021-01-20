use std::io;

fn main() {
	
	println!("Temp Converter");
	
	println!("Enter 1 for F to C or 2 for C to F");
	let mut convert_mode = String::new();
	io::stdin()
		.read_line(&mut convert_mode)
		.expect("Failed to read input");
	let convert_mode: u32 = convert_mode.trim().parse().expect("Invalid");

	println!("Enter temp to convert: ");
	let mut start_temp = String::new();
	io::stdin()
		.read_line(&mut start_temp)
		.expect("Failed to read input");
	let start_temp: f64 = start_temp.trim().parse().expect("Nope");
	

	if convert_mode == 1 {
		let end_temp = (5.0/9.0)*(start_temp - 32.0);
		println!("{} F is {} C", start_temp, end_temp);
	}
	else {
		let end_temp = (9.0/5.0)*start_temp + 32.0;
		println!("{} C is {} F", start_temp, end_temp);
	}

}
