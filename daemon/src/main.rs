use fork::{setsid, fork, Fork};
use std::thread;
use std::time::Duration;


fn main(){
	

	match fork(){
		// First Fork
		Ok(Fork::Parent(child)) => {
			println!("Child Pid: {}", child);
		}

		// Child
		Ok(Fork::Child) => {
			println!("Child Process!");
			// Double Fork
			match fork() {
				// Daemon
				Ok(Fork::Child) => {
					println!("Grandchild");
					// Set SessionID 
					let sess = setsid();
					match sess {
						Ok(pidt) => println!("pidt {}", pidt),
						Err(_) => println!("setsid() failed"),
					}
					loop {
						println!("Loop");
						thread::sleep(Duration::new(4, 0));

					}
				}
				Ok(Fork::Parent(grandchild)) => {
					println!("GC pid: {}", grandchild);
				}
				Err(_) => {
					println!("Failed to Fork!");
				}

			}
		}
		Err(_) => {
			println!("Failed to Fork!");
		}
	}

}
