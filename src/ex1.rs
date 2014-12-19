// http://aml3.github.io/RustTutorial/html/01.html

fn match_exercise(val : (int, bool)) {
	match val {
		(y, x) if x && (y >= 20) && (y <= 26) => println!("true and in range"),
		(_, x) if x => println!("true and out of range"),
		(y, _) if (y >= 40) && (y <= 49) => println!("unknown and in big range"),
		_ => println!("none"),
		
	}
}

fn main() {
	let x = (51, true);
	match_exercise(x);
}