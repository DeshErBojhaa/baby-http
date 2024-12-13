use std::collections::HashMap;
use std::io::Read;
#[allow(unused_imports)]
use std::io::{BufRead, BufReader, Error, Write};
use std::net::TcpListener;
use std::thread;

fn handle_client(mut stream: std::net::TcpStream) {
    let mut reader = BufReader::new(&mut stream);
    
    let mut first_line = String::new();
    reader.read_line(&mut first_line).unwrap();
    
    let mut headers = HashMap::new();
    loop {
        let mut line = String::new();
        reader.read_line(&mut line).unwrap();
        if line == "\r\n" || line.is_empty() {
            break;
        }
        if let Some((key, value)) = line.split_once(": ") {
            headers.insert(
                key.trim().to_string(),
                value.trim_end().to_string()
            );
        }
    }

    let var = first_line.split_ascii_whitespace().nth(1).unwrap();
    let verb = first_line.split_ascii_whitespace().next().unwrap();

    let resp = match [verb, var] {
        ["GET", "/"] => {
            "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: 0\r\n\r\n"
            .to_string()
        }
        ["GET", p] if p.starts_with("/echo") => {
            let invalid_encoding = "invalid-encoding".to_string();
            let encoding_header = String::from("Accept-Encoding");
            let gzip = String::from("gzip");
            let empty_encoding = String::from("");
            let mut encoding = headers.get(&encoding_header)
            .unwrap_or(&invalid_encoding);
            if encoding.contains("gzip") {
                encoding = &gzip;
            } else {
                encoding = &empty_encoding;
            }
            let content = &var[6..];
            format!(
                "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Encoding: {}\r\nContent-Length: {}\r\n\r\n{}",
                encoding,
                content.len(),
                content
            )
        }
        ["GET", "/user-agent"] => {
            let user_agent = headers.get("User-Agent").unwrap();
            format!(
                "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
                user_agent.len(),
                user_agent
            )
        }
        ["GET", p] if p.starts_with("/files") => {
            let file_name = &var[7..];
            let directory = std::env::args().nth(2).unwrap_or_else(|| "/tmp/".to_string());
            let file_path = format!("{}{}", directory, file_name);
            
            match std::fs::read_to_string(file_path) {
                Ok(content) => {
                    format!(
                        "HTTP/1.1 200 OK\r\nContent-Type: application/octet-stream\r\nContent-Length: {}\r\n\r\n{}",
                        content.len(),
                        content
                    )
                }
                Err(_) => "HTTP/1.1 404 Not Found\r\n\r\n".to_string(),
            }
        }
        ["POST", p] if p.starts_with("/files") => {
            let file_name = &var[7..];
            let directory = std::env::args().nth(2).unwrap_or_else(|| "/tmp/".to_string());
            let file_path = format!("{}{}", directory, file_name);
            let content_length = headers.get("Content-Length").unwrap().parse::<usize>().unwrap();

            let mut data = vec![0; content_length];
            reader.read_exact(&mut data).unwrap();

            match std::fs::write(file_path, data) {
                Ok(_) => "HTTP/1.1 201 Created\r\n\r\n".to_string(),
                Err(_) => "HTTP/1.1 500 Internal Server Error\r\n\r\n".to_string(),
            }
        }
        _ => "HTTP/1.1 404 Not Found\r\n\r\n".to_string(),
    };
    stream.write_all(resp.as_bytes()).unwrap();
}

fn main() -> Result<(), Error> {
    let listener = TcpListener::bind("127.0.0.1:4221")?;

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {thread::spawn(move || handle_client(stream));}
            Err(e) => {println!("error: {}", e);}
        }
    }
    Ok(())
}
