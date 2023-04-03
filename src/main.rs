mod storage;
mod utils;
mod server;
mod op;

use std::io;

use op::*;
use server::server_start;

fn main() {
    server_start();
}
