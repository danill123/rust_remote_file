use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

use std::io;

fn handle_client(mut stream: TcpStream, client_id: usize) {
    // Handle the client connection here
    let mut buffer = [0u8; 1024];
    while let Ok(bytes_read) = stream.read(&mut buffer) {
        if bytes_read == 0 {
            // Connection closed
            break;
        }
        let message = String::from_utf8_lossy(&buffer[..bytes_read]);

        let message_array: Vec<String> = message.split(',').map(|s| s.to_string()).collect();
        println!("Client {}: \n", client_id);
        for substring in &message_array {
            println!(" {} \n", substring);
        }

        let mut command_input = String::new();
        println!("input command : ");
        io::stdin().read_line(&mut command_input).expect("Failed to read line");

        // Process and respond to the client
        let response = command_input;

        stream.write_all(response.as_bytes()).expect("Failed to write to socket");
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind to address");
    println!("Server listening on 127.0.0.1:8080");

    // let mut client_streams: Vec<TcpStream> = Vec::new();

    for (client_id, stream) in listener.incoming().enumerate() {
        match stream {
            Ok(stream) => {
                // Spawn a new thread to handle the client
                // let client_id = client_id + 1;
                // client_streams.push(stream.try_clone().expect("Failed to clone stream"));
                // thread::spawn(move || {
                //     handle_client(stream, client_id);
                // });
                handle_client(stream, client_id);
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }

        // println!("list connection {:?}", client_streams );
    }
}
