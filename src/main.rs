use std::io::prelude::*;
extern crate utilities as utils;


fn main() {
    println!("Starting");
    for line in utils::open_input("./assets/hello_world.txt").lines() {
        println!("{}", line.unwrap());
    }
}