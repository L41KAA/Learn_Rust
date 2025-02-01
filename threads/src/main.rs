use std::thread;
use std::sync::{Arc, Mutex};

fn thread_fn(thread_num: i32, lock: Arc<Mutex<i32>>){

	let mut num = lock.lock().unwrap();
	*num += 2;
	println!("Hello from thread: {}", thread_num);
}

fn main() {
	// Create i32 for Threads to change
	let mut val: i32 = 0;
	let shared_val = Arc::new(Mutex::new(val));

	let mut count: i32 = 0;
	let mut handles: Vec<_> = vec![];
	
	// Create 10 Threads
	'threads: loop {
			
		if count >= 10{
			break 'threads;
		}

		// Clone Mutex
		let share_val_clone = Arc::clone(&shared_val);

		// Start Thread
		let handle = thread::spawn(move || {
			thread_fn(count, share_val_clone);
		});
		
		// Keep track of threads
		handles.push(handle);
		count += 1;
	}

	// Join Threads
	for handle in handles {
		handle.join().unwrap();
	}

	// Test for Race
	println!("Final: {}, {}",val,  *shared_val.lock().unwrap());

}
