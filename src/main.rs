use std::io::{Read, Write};
#[allow(unused_imports)]
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                let mut buf = [0; 1024];
                _ = _stream.read(&mut buf);

                let ss: String = buf.iter().map(|&x| x as char).collect();
                let ss = ss.split("\r\n").collect::<Vec<&str>>()[0].split_ascii_whitespace().collect::<Vec<&str>>()[1];
                let mut response = "HTTP/1.1 200 OK\r\n\r\n";
                if ss != "/" {
                    response = "HTTP/1.1 404 Not Found\r\n\r\n";  
                } 
                _ = _stream.write(response.as_bytes()).unwrap();
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
