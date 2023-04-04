mod load_chunk;

use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use crate::op::{op, opcode};
use crate::server::load_chunk::load;
use crate::storage::chunk::Reader;


pub fn server_start() {
    let server = match TcpListener::bind("127.0.0.1:9088") {
        Ok(s) => {
            s
        },
        Err(e) => {
            println!("{}", e.to_string());
            return;
        }
    };

    let path = "./data";
    let mut reader = Reader::new(path);
    let mut map = load(&mut reader);

    while let Ok((stream, ip)) = server.accept() {
        handle(stream, ip, &mut map, path);
    }
}

fn handle(mut stream: TcpStream, ip: SocketAddr, map: &mut HashMap<String, String>, path: &str) {
    let mut buf = [0u8; 1024];
    loop {
        let size = match stream.read(&mut buf) {
            Ok(size) => {size},
            Err(e) => {
                println!("Error: {}", e.to_string());
                0
            }
        };

        let mut content = if &buf[0..size] == &[0] {
            "".to_string()
        } else {
                match String::from_utf8(buf[0..size].to_vec()) {
                    Ok(c) => c,
                    Err(e) => e.to_string()
            }
        };

        println!("{:?}", content.as_bytes());

       // println!("command: {} ------- ip: {}", content, ip);
        let op_code = opcode(&mut content);
        let rt_s = match op(map, op_code, path) {
            Ok(s) => s,
            Err(e) => e.to_string()
        };


        println!("{}", rt_s.len());
        let send: &[u8] = if rt_s.len() == 0 {
            &[0]
        }else {
            rt_s.as_bytes()
        };

        match stream.write(send) {
            Ok(_) => {},
            Err(_) => {}
        }
    }
}