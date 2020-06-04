use std::io::{BufRead, BufReader};
use std::os::unix::net::{UnixStream,UnixListener};

fn main() {
    let listener = UnixListener::bind("/dockerdoom.socket").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(val) => {
                let s = BufReader::new(val);
                for line in s.lines() {
                    println!("{}", line.unwrap());
                }
            },
            Err(err) => {
                println!("Error: {}", err);
                break;
            }
        }
    }
}
