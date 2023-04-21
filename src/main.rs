use std::io::{self, Write, Read};
use std::net::TcpStream;

/// It reads a line and remove \r and \n to the returned string.
fn read_line() -> String {
    //TODO: Reuse the same input buffer instead of allocating a new string at every prompt.
    let mut input_buf = String::new();
    io::stdin()
    .read_line(&mut input_buf) 
    .expect("Failed to read line");

    input_buf.trim().to_string()
}

fn main() {
    let mut stream: Option<TcpStream> = None;

    loop {
        print!("  eztcp | ");
        match &stream {
            Some(s) => print!("Connected to {}\n", s.peer_addr().unwrap()),
            None => print!("Not connected\n")
        }
        println!("1]  Connect to server");
        println!("2]  Send a message & print response");
        println!("3]  Disconnect to server");
        println!("4]  Exit");

        let input = if let Ok(i) = read_line().parse::<i32>() {
            i
        } else {
            0
        };

        match input {
            1 => {
                print!("IP address to connect to: ");
                io::stdout().flush().unwrap();
                let addr = read_line();

                match TcpStream::connect(addr) {
                    Ok(s) => {
                        stream = Some(s);
                        println!(
                            "\n 📡 Connect successfully to `{}`!",
                            stream.as_ref().unwrap().peer_addr().unwrap()
                        );
                    }
                    Err(err) => {
                        println!("ERR: {}", err);
                        stream = None;
                    }
                }
            }
            2 => {
                match stream.as_mut() {
                    Some(_s) => {
                        print!("Message to send: ");
                        io::stdout().flush().unwrap();

                        let msg = read_line();
                        match _s.write(msg.as_bytes()) {
                            Ok(_a) => {
                                let mut buf = [0; 1024];
                                match _s.read(&mut buf) {
                                    Ok(_v) => {
                                        println!("Response: `{}`", String::from_utf8_lossy(&buf));
                                    }
                                    Err(err) => {
                                        println!("ERR: {}", err);
                                        stream = None
                                    }
                                }
                            }
                            Err(err) => {
                                println!("ERR: {}", err);
                                stream = None
                            }
                        }
                    }
                    None => {
                        println!("\n ❌ You're currently not connected to a tcp server!");
                    }
                }
            }
            3 => {
                match stream.as_mut() {
                    Some(_s) => {
                        match _s.shutdown(std::net::Shutdown::Both) {
                            Ok(..) => {
                                stream = None;
                                println!("\n 📡 Disconnect successfully!");
                            }
                            Err(err) => {
                                println!("ERR: {}", err);
                            }
                        }
                    }
                    None => {
                        println!("\n ❌ You're currently not connected to a tcp server!");
                    }
                }
            }
            4 => {
                println!("\n 👋 Goodbye");
                break;
            },
            _ => println!(" Enter a correct number between 1 and 4."),
        }

        println!(); // It's here because it looks prettier with it
    }
}
