fn modify(s: &mut String){
	s.push_str(", World");
}

fn main() {
	let mut s = String::from("Hello");
	modify(&mut s);

	println!("{}", s);

	let mut i: i32 = 0;
	let mut j: i32 = 0;
	'outer: loop {
		if i > 5 {
			break 'outer;
		}
		i += 1;
		'inner: loop{
			j += 1;
			if j > 10 {
				break 'inner;
			}
			println!("{}", j);
		}
		println!("{}", i);
	}
}
