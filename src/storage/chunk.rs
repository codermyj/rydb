use std::io::Error;
use std::fs;
use std::fs::{OpenOptions};
use std::io::{Write};
use chrono::Local;

#[derive(Debug, Clone)]
pub struct DataRow{
    /// 校验字段：crc: i64,
    pub timestamp: i64,
    pub key_len: u32,
    pub value_len: u32,
    pub key: String,
    pub value: String,
    pub flag: u8,
}

impl DataRow {
    pub fn new(key: &str, value: &str, flag: u8) -> DataRow {

        let timestamp = Local::now().timestamp_millis();
        let key = String::from(key);
        let value = String::from(value);
        let key_len = key.len() as u32;
        let value_len = value.len() as u32;

        DataRow { timestamp, key_len, value_len, key, value, flag }
    }

    pub fn write(&mut self, path: &str) -> Result<(), Error> {
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .append(true)
            .read(true)
            .open(path)?;
        file.write(self.to_bytes().as_slice())?;
        Ok(())
    }

    pub fn to_bytes(&mut self) -> Vec<u8> {
        let len = 8 + 4 + 4 + self.key_len + self.value_len + 1;
        let mut vecs: Vec<u8> = Vec::with_capacity(len as usize);
        vecs.append(self.timestamp.to_be_bytes().to_vec().as_mut());
        vecs.append(self.key_len.to_be_bytes().to_vec().as_mut());
        vecs.append(self.value_len.to_be_bytes().to_vec().as_mut());
        vecs.append(self.key.clone().into_bytes().as_mut());
        vecs.append(self.value.clone().into_bytes().as_mut());
        vecs.push(self.flag);

        vecs
    }
}

pub struct Reader {
    data: Vec<u8>,
    pos: usize,
}

impl Reader {
    pub fn new(path: &str) -> Reader {
        let data = match fs::read(path) {
            Ok(d) => d,
            Err(e) => e.to_string().into_bytes()
        };
        let pos = 0;

        Reader { data, pos }
    }

    // 读取一个字节
    pub fn read_byte(&mut self) -> u8 {
        let byte = self.data[self.pos];
        self.pos += 1;

        byte
    }

    // 读取n个字节
    pub fn read_bytes(&mut self, n: usize) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(n);
        for _ in 0..n {
            bytes.push(self.read_byte());
        }
        bytes
    }

    // 大端法读取timestamp字段
    pub fn read_timestamp(&mut self) -> i64 {
        let mut n: i64 = 0;
        let mut m = 8;

        while m > 0 {
            m -= 1;
            n = n | ((self.read_byte() as i64) << (m * 8));
        }
        n
    }

    // 大端法读取u32类型
    pub fn read_u32(&mut self) -> u32 {
        let mut n: u32 = 0;
        let mut m= 4;

        while m > 0 {
            m -= 1;
            n = n | ((self.read_byte() as u32) << (m * 8));
        }

        n
    }

    // 读取字符串
    pub fn read_string(&mut self, n: u32) -> String {
        let bytes = self.read_bytes(n as usize);
        let str = match String::from_utf8(bytes) {
            Ok(s) => s,
            Err(e) => e.to_string()
        };

        str
    }

    pub fn read_data_row(&mut self) -> DataRow {
        let timestamp = self.read_timestamp();
        let key_len = self.read_u32();
        let value_len = self.read_u32();
        let key = self.read_string(key_len);
        let value = self.read_string(value_len);
        let flag = self.read_byte();

        DataRow { timestamp, key_len, value_len, key, value, flag }
    }

    pub fn read_data_all(&mut self) -> Vec<DataRow> {
        let mut data_all: Vec<DataRow> = Vec::with_capacity(1024);

        loop {
            if self.pos < self.data.len() {
                data_all.push(self.read_data_row());
            }else {
                break;
            }
        }

        data_all
    }
}

