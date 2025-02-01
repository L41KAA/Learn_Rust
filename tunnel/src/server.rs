use std::net::{IpAddr, UdpSocket};
use crate::common;
use std::io::{Result};


// server_shell
pub fn server_shell(cmd: String) -> Result<String>{
	// Bind Command
	if cmd.starts_with("bind") {
		let args: Vec<&str> = cmd.split(' ').collect();
		
		// Error
		if args.len() != 3 {
			common::print_error(String::from("Usage: bind <ip_addr> <port>"));
			return Ok(());
		}

		let ip_str: &str = args[1].trim();
		let port_str: &str = args[2].trim();
		
		// Check IP
		match ip_str.parse::<IpAddr>() {
			Ok(ip) => println!("Parsed: {}", ip),
			Err(_) => {
				println!("Failed to parse IP");
				return Ok(());
			},
		}

		// Check Port
		match port_str.parse::<i32>() {
			Ok(port) => {
				if port > 65535 {
					common::print_error(format!("Invalid Port: {}", port));
					return Ok(())
				}
			},
			Err(e) => {
				eprintln!("Failed to parse string: {}", e);
				return Ok(());
			},
		}

		let bind_str: String = format!("{}:{}", ip_str, port_str);
		return Ok(bind_str);
	}
	return Ok(())
}

// vpn_listen
pub fn vpn_listen(bind_str: String){
	let _socket = UdpSocket::bind(bind_str).expect("Failed to bind");
	
}
