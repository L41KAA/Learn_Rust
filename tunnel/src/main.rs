use std::{env, process};
use tun::{AbstractDevice, Configuration, Device, Layer};
use std::io::{Result};

mod common;
mod server;
mod client;

// create_tun_dev
fn create_tun_dev(dev: &mut Option<Device>){ 
	let mut config = Configuration::default();
	config.address("10.0.0.6");	
	config.netmask("255.255.255.0");	
	config.mtu(1500);
	config.layer(Layer::L3);
	config.tun_name("rust-ng");
	config.up();

	let mut new_dev = tun::create(&config).unwrap();
	new_dev.persist().unwrap();
	let tun_name = new_dev.tun_name().unwrap();
	let tun_ip = new_dev.address().unwrap();
	common::print_info(format!("Created Device: '{}'", tun_name));
	common::print_info(format!("Device IP: {}", tun_ip));
	*dev = Some(new_dev);
}

// get_args
fn get_args() -> Result<String> {
	let args: Vec<String> = env::args().collect();
	if args.len() <= 1 {
		common::print_error(format!("Usage: {} <client|server> ", args[0]));
		process::exit(1);
	}

	if args[1].find("client") != None{
		return Ok(args[1].clone());
	}
	if args[1].find("server") != None {
		return Ok(args[1].clone());
	}
	process::exit(1);

}

// main
fn main() {
	let mode: String = get_args().expect("[!] You must provide a run-mode");
	common::print_info(format!("Mode: {}", mode));
	
	let mut dev: Option<Device> = None;

	// Determine Which Shell Handler to use
	let shell_func: fn(String) -> Result<()>;
	if mode == "client" {
		create_tun_dev(&mut dev);
		shell_func = client::client_shell;
	} else {
		shell_func = server::server_shell;
	}
	
	// Loop Shell
	loop {
		let mut res: String = common::shell(mode.clone()).unwrap();
		let _ = shell_func(res);
	}
}
