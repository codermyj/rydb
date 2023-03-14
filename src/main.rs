mod storage;
mod utils;
mod server;
mod op;

use std::io;
use std::io::{stdin, stdout, Write};
use storage::chunk::{DataRow, Reader};
use crate::server::load_chunk::load;
use op::*;

fn main() -> Result<(), io::Error>{

    let path = "./data";

    let mut reader = Reader::new(path);

    let mut map = load(&mut reader);

    loop {
        print!("> ");
        stdout().flush()?;
        let mut buffer = String::new();
        stdin().read_line(&mut buffer)?;
        // println!("{:?}", cmd);

        let op_code = opcode(&mut buffer);

        op(&mut map, op_code);

    }
}
