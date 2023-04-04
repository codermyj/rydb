pub mod api;

use api::*;

use std::collections::HashMap;
use std::io;
use crate::storage::chunk::DataRow;

/// 操作码枚举类型
/// SET：set操作
/// GET：get操作
/// None: 无任何操作
/// Invalid：非法操作
pub enum OpCode<'a> {
    /// Set操作
    SET(&'a str, &'a str),
    GET(&'a str),
    None,
    Invalid,
}

/// 按照空格分割输入的命令，返回动态数组
pub fn split_buf(buf: &str) -> Vec<&str> {
    let bufs:Vec<&str> = buf.split_whitespace().collect();
    if bufs.len() == 0 {
        return vec![""];
    }
    bufs
}

/// 解析set、get指令
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

pub fn op(map: &mut HashMap<String, String>, opcode: OpCode, path: &str) -> Result<String, io::Error>{
    use OpCode::*;
    let str = match opcode {
        SET(key, value) => {
            let mut data = DataRow::new(key, value, 0);
            set(map, key, &mut data, path)?;
            "SET operate successful".to_string()
        },
        GET(key) => get(map, key) ,
        None => "".to_string(),
        Invalid => "Invalid Operate!".to_string()
    };

    Ok(str)
}