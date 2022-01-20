use std::io::prelude::*;
extern crate greeter;
extern crate reader;


fn main() {
    println!("Starting");
    for line in reader::open_input("./assets/hello_world.txt").lines() {
        println!("{}", line.unwrap());
    }
}