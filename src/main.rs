//
// zhttpto.rs
//
// Starting code for PS1
// Running on Rust 0.12
//
// Note that this code has serious security risks! You should not run it
// on any system with access to sensitive files.
//
// University of Virginia - cs4414 Spring 2014
// Weilin Xu and David Evans
// Version 0.3

#![feature(globs)]

use std::io::*;
use std::str;

fn main() {

    let addr = "127.0.0.1:4414";
    let mut acceptor = net::tcp::TcpListener::bind(addr).listen();
	let mut rc = 0i;
	
    println!("Listening on [{}] ...", addr);

    for stream in acceptor.incoming() {
		rc += 1;
        match stream {
            Err(_) => (),
            // Spawn a task to handle the connection
            Ok(mut stream) => spawn(move|| {
                match stream.peer_name() {
                    Err(_) => (),
                    Ok(pn) => println!("Received connection from: [{}] / {}", pn, rc),
                }
                let mut buf = [0, ..500];
                let _ = stream.read(&mut buf);
                let request_str = str::from_utf8(&buf);
				match request_str {
					None => println!("Failed reading request"),
					Some(request_val) => {
						let response = process_request(request_val);
		                let _ = stream.write(response.as_bytes());
					}
				}
                
                println!("Connection terminates.");
            }),
        }
    }

    drop(acceptor);
}

fn process_request(str& request_str) -> str {
	let split: Vec<&str> = request_val.split('\n').collect();
	
	// these split elements must exist per HTTP spec
	let split_request: Vec<&str> = split[0].split(' ').collect();
	//let method = split_request[0];
	let as_string = String::from_str(split_request[1]);
	let uri = match split_request[1] {
		"/" => String::from_str("./index.html"),
		_ => "." + as_string,
	};
	
	let _ = match File::open(&Path::new(uri.as_slice())) {
		Err(msg) => println!("couldn't open {}: {}", uri, msg),
		Ok(file) => println!("Hi"),
	};
	
	let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n
         <doctype !html><html><head><title>Hello, Rust!</title>
		 </head>
         <body>
         <h1>File</h1>
		 <h2>{}</h2>
         </body></html>\r\n", uri);
	 return response;
}

#[test]
fn test_process_request() {
	str request = "GET / http/1.1";
	str response = process_request(request);
	assert!(response == 
		"HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n
         <doctype !html><html><head><title>Hello, Rust!</title>
		 </head>
         <body>
         <h1>File</h1>
		 <h2>./index.html</h2>
         </body></html>\r\n")
}