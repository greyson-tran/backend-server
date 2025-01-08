use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() -> () {
    loop {
        let listener = TcpListener::bind("0.0.0.0:19132").expect("Port binding has failed.");

        for stream in listener.incoming() {
            let stream = stream.expect("Stream capture has failed.");
            handle_connection(stream);
        }
    }
}

fn handle_connection(mut stream: TcpStream) -> () {
    let buf_reader = BufReader::new(&stream);

    // let request_line = buf_reader.lines().next().unwrap().unwrap();

    let status_line = "HTTP/1.1 200 OK";
    let contents = fs::read_to_string("site.html").expect("The filesystem access failed.");
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream
        .write_all(response.as_bytes())
        .expect("Message sending has failed.")
}
