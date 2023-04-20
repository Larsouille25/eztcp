use std::net::TcpStream;
use std::io;

fn main() {
    loop {
        println!("📜  Menu :");
        println!("[1] 📡 Connect to server");
        println!("[2] 📨 Send a message");
        println!("[3] 👂 Listen for response of the server");
        println!("[4] ❌ Disconnect to server");
        println!("[5] 👋 Exit");

        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(_len) => {
                match input[..1].parse() {
                    Ok(i) => {
                        let input: i32 = i;
                        match input {
                            1 => {},
                            2 => {},
                            3 => {},
                            4 => {},
                            5 => {
                                println!("\n Goodbye! 👋");
                                break;
                            },
                            _ => {
                                println!("Please choose a number between 1 and 5.");
                            }
                        }
                    },
                    Err(err) => {
                        println!("ERR: {:?}", err);
                        break;
                    }
                }
            }, Err(err) => {
                println!("ERR: {:?}", err);
                break;
            }
        }


        println!("");
    }
}
