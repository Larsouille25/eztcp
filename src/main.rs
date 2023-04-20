use std::io;
use std::net::TcpStream;

fn read_line() -> String {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => input.to_owned().replace("\r", "").replace("\n", ""),
        Err(err) => {
            panic!("ERR: {}", err);
        }
    }
}

fn main() {
    let mut stream: TcpStream;

    loop {
        println!("📜  Menu :");
        println!("1] 📡 Connect to server");
        println!("2] 📨 Send a message");
        println!("3] 👂 Listen for response of the server");
        println!("4] ❌ Disconnect to server");
        println!("5] 👋 Exit");

        let input = if let Ok(i) = read_line().parse::<i32>() {
            i
        } else {
            0
        };

        match input {
            1 => {
                let addr = read_line();

                match TcpStream::connect(addr) {
                    Ok(s) => {
                        stream = s;
                        println!(" 📡 Connect successfully to `{:?}`!", stream);
                    }
                    Err(err) => {
                        println!("ERR: {}", err);
                    }
                }
            }
            5 => break,
            _ => println!(" Enter a correct number between 1 and 5."),
        }

        println!(""); // It's here because it looks prettier with it
    }
}
