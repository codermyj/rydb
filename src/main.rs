mod storage;
mod utils;
mod server;

use std::io;
use std::io::{stdin, stdout, Write};
use storage::chunk::{DataRow, Reader};
use crate::server::load_chunk::load;

fn main() -> Result<(), io::Error>{

    let path = "./data";

    let mut reader = Reader::new(path);

    let mut map = load(&mut reader);

    loop {
        print!("> ");
        stdout().flush()?;
        let mut buffer = String::new();
        stdin().read_line(&mut buffer)?;
        let cmd: Vec<&str> = buffer.split_whitespace().collect();
        // println!("{:?}", cmd);

        if cmd.len() == 0 {
            continue;
        }

        match cmd[0] {
            "set" => {
                if cmd.len() == 3 {
                    let mut data = DataRow::new(cmd[1], cmd[2], 0);
                    map.insert(cmd[1].to_string(), cmd[2].to_string());
                    data.write(path)?;
                }else {
                    println!("非法的数据！")
                }
            },
            "get" => {
                if cmd.len() == 2 {
                    let key = cmd[1];
                    let value = match map.get(key) {
                        Some(s) => s,
                        None => "Not Found The Key!",
                    };
                    println!("{}", value);
                }else {
                    println!("非法的操作!");
                }
            },
            _ => println!("非法操作!"),
        }


    }
}
