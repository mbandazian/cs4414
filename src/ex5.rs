/* http://aml3.github.io/RustTutorial/html/02.html
Exercise 2.2. Implement a function, incrementMut that takes as input a vector of integers and modifies the values of the original list by incrementing each value by one. For example:
fn main() {
   let mut p = ~[1, 2, 3];
   incrementMut(p);
   for &x in p.iter() {
      print!("{:d} ", x);
   }
}
should print out 2 3 4.
*/

fn increment_mut(v : &mut Vec<int>) {
	for ix in range(0, (*v).len()) {
		(*v)[ix] += 1;
	}
}

fn main() {
	let mut v = vec![5i, 1i];
	increment_mut(&mut v);
	for &x in v.iter() {
		println!("{}", x);
	}
}