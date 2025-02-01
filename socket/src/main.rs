use std::net::{TcpListener, TcpStream};
use std::os::unix::io::{AsRawFd, RawFd};
use std::io::Read;
use nix::poll::{PollFd};

fn start_poll(fd: i32){
	println!("Ready to poll {}, fd");	
}

fn read_socket(stream: &mut TcpStream){
	// Read
	const BUF_SIZE: usize = 1024;
	let mut client_buf = [0; BUF_SIZE];
	let read_res = stream.read(&mut client_buf);
	match read_res {
		Ok(nb) => println!("Read {} bytes", nb),
		Err(e) => println!("{}", e),
	}

}

fn handle_client(mut stream: TcpStream){
	const BUF_SIZE: usize = 1024;
	let mut buf = [0; BUF_SIZE];

	let client_fd: RawFd = stream.as_raw_fd();
	println!("New Client_FD: {}", client_fd);

	loop {
		// Peek
		let len = stream.peek(&mut buf).expect("Failed to Peek");
		println!("{} Bytes available to read", len);
		if len > 0 {
			read_socket(&mut stream);
		}
	}
}

fn main() -> std::io::Result<()> {
	let listener = TcpListener::bind("127.0.0.1:8888")?;

	for stream in listener.incoming() {
		match stream {
			Ok(good_stream) =>{
				good_stream.set_nonblocking(true).expect("Failed to set NON_BLOCK");
				handle_client(good_stream);
			},
			Err(_) => println!("Failed to Get Stream"),
		}
		//handle_client(stream);
	}
    println!("Hello, world!");

	Ok(())
}
