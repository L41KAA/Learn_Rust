use std::io;
use std::io::{Result, Write};

pub fn print_error(msg: String) {
	println!("[!] {}", msg);
}

pub fn print_info(msg: String) {
	println!("[*] {}", msg);
}


// shell
pub fn shell(context: String) -> Result<String> {
	let mut buffer = String::new();
	print!("{}> ", context);
	io::stdout().flush().expect("Failed to Flush");
	io::stdin().read_line(&mut buffer).expect("Failed to read stdin");

	return Ok(buffer)
}
