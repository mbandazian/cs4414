//Exercise 2.1.
// Implement a function, increment that takes as input a vector of integers and returns a new vector of integers that has the values of the original list each incremented by one. 

fn increment(v: &[int]) -> Vec<int> {
	let mut v2 = Vec::new();
	for x in v.iter() {
		v2.push(*x + 1);
	}
	return v2;
}

fn main() {
	let v = [1, 2];
	let v2 = increment(&v);
	
	for x in v2.iter() {
		println!("{}", x);
	}
}