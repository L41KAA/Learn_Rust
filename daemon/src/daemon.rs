extern crate fork;
use fork::{fork, Fork};


fn main(){
	Ok(Fork::Parent(child)) => {
		println!("Child Process spawned with pid: {}", child);
	}
	Ok(Fork::Child) => {
		println!("Hello From Child!");
	}
	Err(_) => {
		println!("Fork Failed");
	}

}
