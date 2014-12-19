/* http://aml3.github.io/RustTutorial/html/02.html
Exercise 2.3. Implement the joiner. It should take two file names as its inputs, and output to standard output the result of XOR-ing the bytes in those files. The inputs files must be the same length.
*/

use std::os;
use std::io::File;

fn main() {
    let args = os::args();
    if args.len() != 3 {
        println!("Usage: {} <inputfile> <inputfile>", args[0]); 
    } else {
        let path1 = Path::new(&args[1]);
        let path2 = Path::new(&args[2]);
        let msg_file1 = File::open(&path1);
		let msg_file2 = File::open(&path2);

        match (msg_file1, msg_file2) {
            (Ok(mut msg1), Ok(mut msg2)) => {
                let msg_result1 = msg1.read_to_end();
				let msg_result2 = msg2.read_to_end();
                let join_file 
                       = File::create(&Path::new("plaintext.join"));
                
                match (msg_result1, msg_result2, join_file) {
                    (Ok(msg_bytes1), Ok(msg_bytes2), Ok(join_file)) => { 
                        join(msg_bytes1, msg_bytes2, join_file); 
                        } ,
                    (_, _, _) => panic!("Error opening output files!"),
                }
            } ,
            (Err(_), _) => panic!("Error opening message file: {}", args[1]),
			(_, Err(_)) => panic!("Error opening message file: {}", args[2])
        }
    }
}

fn xor(a: &Vec<u8>, b: &Vec<u8>) -> Vec<u8> {
    let mut ret = vec![];
    for i in range(0, a.len()) {
		ret.push(a[i] ^ b[i]);
    }
    ret
}

fn join(msg_bytes1: Vec<u8>, msg_bytes2: Vec<u8>, mut join_file: File) {
    let plaintext_bytes = xor(&msg_bytes1, &msg_bytes2);
    let write_result = join_file.write(&*plaintext_bytes.into_boxed_slice());
}