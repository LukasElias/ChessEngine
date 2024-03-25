extern crate regex;

use {
    std::{
        fs,
        io::{self, BufRead, Write},
        net,
    },
    regex::Regex,
};

pub fn start_server() -> std::io::Result<()> {
    let listener = net::TcpListener::bind("127.0.0.1:7000")?;
    
    for stream in listener.incoming() {
        handle_request(stream?);
    }

    Ok(())
}

fn handle_request(mut stream: net::TcpStream) {
    let buf_reader = io::BufReader::new(&stream);

    let http_request: String = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect::<Vec<String>>()
        .join("\n");

    println!("{}", http_request);

    let pattern = Regex::new(r"GET\s\/\S*").unwrap();

    println!("{:?}", pattern.find(&http_request));


    let file_path: String = format!("frontend{}", match pattern.find(&http_request) {
        Some(match_value) => {
            let path = &match_value.as_str()[4..]; // Extract path starting from index 4
            if path == "/" {
                "/index.html".to_string()
            } else {
                path.to_string()
            }
        }
        None => "/index.html".to_string(), // Default to index.html if no path is found
    });

    println!("{:?}", file_path);

    let file_content = match fs::read(&file_path) {
        Ok(content) => content,
        Err(_) => {
            let not_found_response = b"HTTP/1.1 404 NOT FOUND\r\n\r\n";
            stream.write_all(not_found_response).unwrap();
            return;
        }
    };

    let http_response = [b"HTTP/1.1 200 OK\r\n\r\n", file_content.as_slice()].concat();
    stream.write_all(&http_response).unwrap();
}
