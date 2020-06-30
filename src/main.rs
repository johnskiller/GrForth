extern crate log;
extern crate simple_logger;

use crate::core::ForthCore;
use crate::dictionary::OpCode;
use std::io::prelude::*;
use std::io::{stdout, Write};

mod core;
mod stack;
mod primv;
mod dictionary;
mod word;

fn test() {
    println!("Hello, world!");
    let mut core = ForthCore::new();
    //core.init();
    core.add_udw("**".to_string(), vec!["dup", "*"]);
    println!("{:?}", core);
    let s = "3 2 * . : 3x 3 * ; 4 3x . cr";
    let input = s.to_string();
    core.interpret2(s);
    //println!("{:?}", core);

    loop {
        print!("Ok. ");
        stdout().flush();
        let line = readline();
        core.interpret2(line.as_ref());
    }
    //println!("{:?}", core);
}

fn readline() -> String {
    let stdin = std::io::stdin();

    let input = stdin.lock().lines().next();

    input
        .expect("No lines in buffer")
        .expect("Failed to read line")
        .trim()
        .to_string()

}

fn main() {
    println!("Opcode {:?}, as usize {}",OpCode::MUL, OpCode::MUL as usize);
    simple_logger::init().unwrap();
    test()
}
