pub mod api;

use api::*;

use std::collections::HashMap;
use crate::storage::chunk::DataRow;

pub enum OpCode<'a> {
    SET(&'a str, &'a str), //Set操作
    GET(&'a str),          //Get操作
    None,                  //无任何操作，即回车
    Invalid,               //非法的操作
}

pub fn split_buf(buf: &str) -> Vec<&str> {
    let bufs:Vec<&str> = buf.split_whitespace().collect();
    if bufs.len() == 0 {
        return vec![""];
    }
    bufs
}


pub fn opcode(buf: &str) -> OpCode {
    use OpCode::*;
    let bufs = split_buf(buf);
    match bufs[0].to_lowercase().as_str() {
        "set" => {
            if bufs.len() == 3 {
                SET(bufs[1], bufs[2])
            }else {
                Invalid
            }

        },
        "get" => {
            if bufs.len() == 2 {
                GET(bufs[1])
            }else {
                Invalid
            }
        },
        "" => {
            None
        }
        _ => Invalid,
    }
}

pub fn op(map: &mut HashMap<String, String>, opcode: OpCode) {
    use OpCode::*;
    match opcode {
        SET(key, value) => {
            let mut data = DataRow::new(key, value, 0);
            set(map, key, &mut data);
        },
        GET(key) => {
            let value = get(map, key);
            println!("{}", value);
        },
        None => {},
        Invalid => {
            println!("非法操作!");
        }
    }
}