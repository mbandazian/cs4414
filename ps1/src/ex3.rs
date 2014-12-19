use std::os;

fn main() {
	if os::args().len() < 2 {
		println!("usage: {} [number]", os::args()[0]);
		return;
	}
	
	let i = from_str::<int>(os::args()[1].as_slice()).unwrap();
	println!("{}", collatz_with_steps(1, i));
}

fn collatz_with_steps(n: int, steps: int) -> int {
	if collatz(n) == steps {
		return n;
	} 
	else {
		return collatz_with_steps(n+1, steps);
	}
}

fn collatz(n: int) -> int {
	if n == 1 {
		return 0;
	}
	match n % 2 {
		0 => 1 + collatz(n / 2),
		_ => 1 + collatz(n * 3 + 1),
	}
}