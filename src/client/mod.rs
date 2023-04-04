use std::io;
use std::io::{Read, stdout, Write};
use std::net::{TcpStream};

pub fn client_start() {
    let mut client = match TcpStream::connect("127.0.0.1:9088") {
        Ok(client) => {
            println!("Connected successfully.");
            client
        },
        Err(e) => {
            println!("Error: {}", e.to_string());
            return;
        }
    };

    let mut buffer = [0u8; 1024];
    let mut size;
    loop {
        print!("> ");
        stdout().flush().unwrap();
        size = match io::stdin().read(&mut buffer) {
            Ok(size) => {size - 2},
            Err(e) => {
                println!("{}", e.to_string());
                continue;
            }
        };


        //let mut buffer2 = [0u8; 1024];

        let send: &[u8] = if size == 0 {
            &[0]
        } else {
            &buffer[0..size]
        };
        match client.write(send) {
            Ok(_) => { }
            Err(e) => {
                println!("{}", e.to_string());
            }
        }


        let content = match client.read(&mut buffer) {
            Ok(size) => {
                    match String::from_utf8(buffer[0..size].to_vec()) {
                        Ok(s) => s,
                        Err(e) => e.to_string()
                    }
            }
            Err(e) => e.to_string()
        };

        if content.as_bytes() == &[0] {
        }else {
            println!("{}", content);
        }

    }
}