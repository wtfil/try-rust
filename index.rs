use std::io::{TcpListener, TcpStream};
use std::io::{Acceptor, Listener};

fn main () {

    let socket_address = "127.0.0.1:3000";
    let listener = TcpListener::bind(socket_address);

    let mut acceptor = listener.listen();
    println!("Server started at {}", socket_address);

    fn handle_client(mut stream: TcpStream) {

        let status = 200i;
        let response_type = "text/html";
        let response_text = "hello world";

        let s = format!(
            "HTTP/1.1 {:i} OK\nContent-Type: {:s}; charset=utf-8\nContent-Length: {:i}\n\n{:s}",
            status,
            response_type,
            response_text.len() as int,
            response_text
        );

        stream.write(s.as_bytes());
    }

    for stream in acceptor.incoming() {
        match stream {
            Err(e) => { println!("Error while request {}", e); }
            Ok(stream) => spawn(proc() {
                handle_client(stream)
            })
        }
    }

    drop(acceptor);
}
