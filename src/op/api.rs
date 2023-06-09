use std::collections::HashMap;
use std::io;
use crate::storage::chunk::DataRow;

/// set操作，向库中存入一个kv对
/// 参数：
/// map：内存中的哈希表
/// key：要存入的key值
/// data：DataRow结构数据（）
pub fn set(map: &mut HashMap<String, String>, key: &str, data: &mut DataRow, path: &str) -> Result<(), io::Error>{
    map.insert(key.to_string(), data.value.clone()); // 写入内存
    data.write(path)?; // 写入磁盘
    Ok(())
}

/// get指令，根据key从数据库中查询一个值
pub fn get(map: &mut HashMap<String, String>, key: &str) -> String {
    match map.get(key) {
        Some(value) => value.to_string(),
        None => format!("未找到值, key:{}" , key)
    }
}