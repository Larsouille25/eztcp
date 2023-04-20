use std::io::{self, Write};
use std::net::TcpStream;


/// It reads a line and remove \r and \n to the returned string.
fn read_line() -> String {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => input.to_owned().replace(['\r', '\n'], ""),
        Err(err) => {
            panic!("ERR: {}", err);
        }
    }
}

fn main() {
    let mut stream: Option<TcpStream> = None;

    loop {
        print!("  eztcp | ");
        match stream {
            Some(ref s) => print!("Connected to {}", s.peer_addr().unwrap()),
            None => print!("Not connected")
        }
        println!("1]  Connect to server");
        println!("2]  Send a message");
        println!("3]  Listen for response of the server");
        println!("4]  Disconnect to server");
        println!("5]  Exit");

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
                        println!(" 📡 Connect successfully to `{}`!", stream.unwrap().peer_addr().unwrap());
                    }
                    Err(err) => {
                        println!("ERR: {}", err);
                    }
                }
            }
            2 => {
                // let msg = read_line();
                // match stream.write(msg.as_bytes()) {
                //     Ok(a) => {
                //         println!("a = {a}");
                //     }
                //     Err(err) => println!("ERR: {}", err)
                // }
            }
            5 => break,
            _ => println!(" Enter a correct number between 1 and 5."),
        }

        println!(); // It's here because it looks prettier with it
    }
}
