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

use std::io::fs::File;
use std::io::{TcpListener};
use std::io::{Acceptor, Listener};
use std::str;

fn main() {

    let addr = "127.0.0.1:4414";
    let mut acceptor = TcpListener::bind(addr).listen();
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

fn process_request(request: &str) -> String {
	let split: Vec<&str> = request.split('\n').collect();
	
	// these split elements must exist per HTTP spec
	let split_request: Vec<&str> = split[0].split(' ').collect();
	let uri = resolve_get_path(split_request[1]);
	
	match File::open(&uri) {
		Err(msg) => {
			println!("Error loading {}: {}", uri.display(), msg);
			return String::from_str("HTTP/1.1 404 Not Found\r\n");
		},
		Ok(mut file) => {
			println!("Sending {}", uri.display());
			match file.read_to_string() {
				Err(msg) => {
					println!("Error loading {}: {}", uri.display(), msg);
					return String::from_str("HTTP/1.1 404 Not Found\r\n");
				},
				Ok(string) => {
					return format!(
				        "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n{}", string);
				},
			};
		},
	};
}

// resolve path to relative path and / to index.html
fn resolve_get_path(path: &str) -> Path {
	return match path {
		"/" => Path::new("index.html"),
		_ => Path::new(path.slice_from(1)),
	};
}

#[cfg(test)]
mod test {
	use super::resolve_get_path;
	use super::process_request;
	
	#[test]
	fn test_resolve_get_path_index() {
		let p = resolve_get_path("/");
		match p.as_str() {
			Some(p_str) => assert!(p_str == "index.html"),
			None => panic!("invalid path"),
		}
	}
	
	#[test]
	fn test_process_request() {
		let request = "GET / http/1.1";
		let response = process_request(request);
		println!("resp: {}", response);
		let cmp = format!("HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n
<doctype !html><html><head><title>Hello, Rust!</title>
</head>
<body>
<h1>File</h1>
<h2>index.html</h2>
</body></html>\r\n");
		println!("cmp: {}", cmp);
		assert!(response == cmp);
	}
}