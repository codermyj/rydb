mod load_chunk;

use std::io::{Read, Write};
use std::net::TcpListener;
use crate::op::{op, OpCode, opcode};
use crate::server::load_chunk::load;
use crate::storage::chunk::Reader;


pub fn server_start() {
    let server = match TcpListener::bind("127.0.0.1:9088") {
        Ok(s) => s,
        Err(e) => {
            println!("{}", e.to_string());
            return;
        }
    };

    let path = "./data";
    let mut reader = Reader::new(path);
    let mut map = load(&mut reader);

    let mut buf = [0u8; 1024];
    let mut content = String::new();
    let mut op_code: OpCode;

    loop {
        match server.accept() {
            Ok((mut stream, mut ip)) => {
                let size = match stream.read(&mut buf) {
                    Ok(size) => size,
                    Err(e) => {
                        println!("Error: {}", e.to_string());
                        0
                    }
                };
                content = match String::from_utf8(buf[0..size].to_vec()) {
                    Ok(c) => c,
                    Err(e) => e.to_string()
                };
                println!("command: {} ------- ip: {}", content, ip);
                op_code = opcode(&mut content);
                let rt_s = match op(&mut map, op_code, path) {
                    Ok(()) => "Successfully!",
                    Err(e) => "Failed!"
                };
                stream.write(rt_s.as_bytes());
            }
            Err(e) => {
                println!("Error: {}", e.to_string());
            }
        }
    }
}