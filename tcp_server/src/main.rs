use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    println!("Waitting for connections...");

    for stream in listener.incoming() {
        let current_stream = stream.unwrap();

        println!("Connection established");
    }
}
