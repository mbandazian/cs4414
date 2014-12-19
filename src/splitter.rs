use std::rand::random;
use std::os;
use std::io::File;

fn main() {
    let args = os::args();
    if args.len() != 2 {
        println!("Usage: {} <inputfile>", args[0]); 
    } else {
        let fname = args[1].clone();
        let path = Path::new(fname.clone());
        let msg_file = File::open(&path);

        match msg_file {
            Ok(mut msg) => {
                let msg_result = msg.read_to_end();
                let share1_file 
                       = File::create(&Path::new(format!("{}.share1", fname)));
                let share2_file 
                       = File::create(&Path::new(format!("{}.share2", fname)));
                
                match (msg_result, share1_file, share2_file) {
                    (Ok(msg_bytes), Ok(share1), Ok(share2)) => { 
                        split(msg_bytes, share1, share2); 
                        } ,
                    (_, _, _) => panic!("Error opening output files!"),
                }
            } ,
            Err(_) => panic!("Error opening message file: {}", fname)
        }
    }
}

fn xor(a: &Vec<u8>, b: &Vec<u8>) -> Box<Vec<u8>> {
    let mut ret = box vec![];
    for i in range(0, a.len()) {
		ret.push(a[i] ^ b[i]);
    }
    ret
}

fn split(msg_bytes: Vec<u8>, mut share1: File, mut share2: File) {
    let mut random_bytes: Vec<u8> = vec![];
    // This is not cryptographically strong randomness! 
    // (For entertainment purposes only.)
    for _ in range(0, msg_bytes.len()) {
		let random_byte = random();
		random_bytes.push(random_byte);
    }
    
    let encrypted_bytes = xor(&msg_bytes, &random_bytes);
    let write_result_1 = share1.write(&*random_bytes.into_boxed_slice());
    let write_result_2 = share2.write(&*encrypted_bytes.into_boxed_slice());
}