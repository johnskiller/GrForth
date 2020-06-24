use crate::core::ForthCore;
use std::io::prelude::*;

mod core;
mod stack;
mod primv;

fn test() {
    println!("Hello, world!");
    let mut core = ForthCore::new(ForthCore::init_dict());
    //core.init();
    core.add_udw("**".to_string(), vec!["dup", "*"]);
    println!("{:?}", core);
    let s = "3 2 * . : 3x 3 * ; 4 3x . cr";
    let input = s.to_string();
    core.interpret();
    //println!("{:?}", core);

    /*
    loop {
        print!("Ok. ");
        let line = readline();
        core.interpret(line);
    }
    */
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
    test()
}
