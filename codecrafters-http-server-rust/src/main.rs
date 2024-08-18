use std::{
    fs,
    io::{prelude::*, BufReader, Read, Write},
    net::{TcpListener, TcpStream},
    str::from_utf8,
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
    } else if request_line.contains("GET /files") {
        let route = request_line.split_whitespace().nth(1).unwrap();
        let file_name = route.trim_start_matches("/files/");
        let response;
        match fs::read_to_string(format!("{}.txt", file_name)) {
            Ok(file_contents) => {
                response = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\n Content-Length: {}\r\n{}",
                    file_contents.len(),
                    file_contents
                );
            }
            Err(e) => {
                response = "HTTP/1.1 404 Not Found\r\nContent-Type: text/plain\r\n\r\n".to_string();
            }
        }
        stream.write_all(response.as_bytes()).unwrap();
    } else if request_line.starts_with("POST /files") {
        // for l in lines {
        //     if l.unwrap().starts_with("Content-Length") {
        //         let sizeplit = l.unwrap().split(":");
        //         for s in sizeplit {
        //             if !(s.starts_with("Content-Length")) {
        //                 size = s.trim().parse::<usize>().unwrap(); //Get Content-Length
        //             }
        //         }
        //     }
        // }
        let mut buffer = [0; 5];

        // Read the data from the stream
        let mut total_size = 0;
        loop {
            match stream.read(&mut buffer[total_size..]) {
                Ok(0) => break, // End of stream
                Ok(n) => {
                    total_size += n;
                }
                Err(e) => {
                    println!("Error reading from stream: {}", e);
                    return;
                }
            }
        }

        // Convert the buffer to a string and handle it
        let request_body = match from_utf8(&buffer[..total_size]) {
            Ok(body) => body,
            Err(_) => {
                println!("Error decoding UTF-8");
                return;
            }
        };
        println!("Request Body: {}", request_body);

        // Create a response
        let response_body = format!("Received POST data: {}", request_body);
        let content_length = response_body.len();
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
            content_length, response_body
        );

        // Send the response
        if let Err(e) = stream.write_all(response.as_bytes()) {
            println!("Error writing response: {}", e);
        }
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
                // thread::spawn(|| {
                handle_client(_stream);
                // });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
