use crate::common;
use std::io::{Result};

// client_shell 
pub fn client_shell(cmd: String) -> Result<()>{
	if cmd.starts_with("add") {
		let args: Vec<&str> = cmd.split(' ').collect();
		println!("Got {:?}", args);
	}
	return Ok(())
}
