use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    let address = "127.0.0.1:7878";
    let listener = TcpListener::bind(address).unwrap();

    println!("Waiting on http://{}", address);
    println!("Waiting for connections...");

    for stream in listener.incoming() {
        let current_stream = stream.unwrap();

        println!("Connection established");

        handle_connection(current_stream);
    }

    fn handle_connection(mut stream: TcpStream) {
        let buf_reader = BufReader::new(&mut stream);
        // let http_request: Vec<_> = buf_reader
        //     .lines()
        //     .map(|result| result.unwrap())
        //     .take_while(|line| !line.is_empty())
        //     .collect();

        let request_line = buf_reader.lines().next().unwrap().unwrap();

        let (status_line, file_name) = if request_line == "GET / HTTP/1.1" {
            ("HTTP/1.1 200 OK", "index.html")
        } else {
            ("HTTP/1.1 404 NOT FOUND", "404.html")
        };
        send_response(&mut stream, status_line, file_name);
    }

    fn send_response(stream: &mut TcpStream, status_line: &str, file_name: &str) {
        let contents = fs::read_to_string(file_name).unwrap();
        let length = contents.len();

        let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

        stream.write_all(response.as_bytes()).unwrap();
    }
}
