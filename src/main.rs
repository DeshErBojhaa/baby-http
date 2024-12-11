#[allow(unused_imports)]
use std::io::{BufRead, BufReader, Error, Write};
use std::net::TcpListener;

fn main() -> Result<(), Error>{
    let listener = TcpListener::bind("127.0.0.1:4221")?;

    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                let reader = BufReader::new(&mut _stream);
                let line = reader.lines().next().unwrap()?;
                let var = line.split_ascii_whitespace().nth(1).unwrap();
                if var == "/" {
                    let resp = "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: 0\r\n\r\n";
                    _stream.write_all(resp.as_bytes())?;
                    continue;
                }
                if !var.starts_with("/echo") {
                    let resp = "HTTP/1.1 404 Not Found\r\n\r\n".to_string();
                    _stream.write_all(resp.as_bytes())?;
                    continue;
                }
                let var = &var[6..];
                let resp = format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}", var.len(), var);
                _stream.write_all(resp.as_bytes())?;
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
    Ok(())
}
