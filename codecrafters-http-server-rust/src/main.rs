// Uncomment this block to pass the first stage
use std::{
    fmt::format,
    io::{prelude::*, BufReader, Write},
    net::{TcpListener, TcpStream},
};

fn handle_client(mut stream: TcpStream) {
    // Sending a response to the client
    let buf_reader = BufReader::new(&mut stream);
    let mut lines = buf_reader.lines();
    let request_line = lines.next().unwrap().unwrap();
    if request_line == "GET / HTTP/1.1" {
        let response = "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\n\r\n";
        stream.write_all(response.as_bytes()).unwrap();
    } else if request_line.contains("GET /echo") {
        let string_route = request_line.split_whitespace().nth(1).unwrap();
        let string_in_the_route = string_route.trim_start_matches("/echo/");

        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\n Content-Length: {}\r\n{}",
            string_in_the_route.len(),
            string_in_the_route
        );
        println!("{}", response);
        stream.write_all(response.as_bytes()).unwrap();
    } else if request_line.contains("GET /user-agent") {
        let user_agent = lines.nth(2).unwrap().unwrap();
        println!("{}", user_agent);
        let string_in_the_route = user_agent.trim_start_matches("USER-AGENT: ");

        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\n Content-Length: {}\r\n{}",
            string_in_the_route.len(),
            string_in_the_route
        );
        println!("{}", response);
        stream.write_all(response.as_bytes()).unwrap();
    } else {
        let response = "HTTP/1.1 404 Not Found\r\nContent-Type: text/plain\r\n\r\n";
        stream.write_all(response.as_bytes()).unwrap();
    }
    stream.flush().unwrap();
}
fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    //
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => {
                println!("accepted new connection");
                handle_client(_stream);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
