#[allow(unused_imports)]
use std::io::{BufRead, BufReader, Error, Write};
use std::net::TcpListener;
use std::thread;

fn handle_client(mut stream: std::net::TcpStream) {
    let reader = BufReader::new(&mut stream);
    let lines: Vec<String> = reader
        .lines()
        .map(|line| line.unwrap_or_default())
        .take_while(|line| !line.is_empty())
        .collect();

    let var = lines[0].split_ascii_whitespace().nth(1).unwrap();
    let resp = match var {
        "/" => {
            "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: 0\r\n\r\n"
            .to_string()
        }
        p if p.starts_with("/echo") => {
            let content = &var[6..];
            format!(
                "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
                content.len(),
                content
            )
        }
        "/user-agent" => {
            let user_agent = lines[lines.len() - 1]
                .split_ascii_whitespace()
                .nth(1)
                .unwrap();
            format!(
                "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
                user_agent.len(),
                user_agent
            )
        }
        p if p.starts_with("/files") => {
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
        _ => "HTTP/1.1 404 Not Found\r\n\r\n".to_string(),
    };
    stream.write_all(resp.as_bytes()).unwrap();
}

fn main() -> Result<(), Error> {
    let listener = TcpListener::bind("127.0.0.1:4221")?;

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream);
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
    Ok(())
}
