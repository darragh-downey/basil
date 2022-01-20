extern crate greeter;


fn main() {
    let msg = greeter::Greeter::new("Welcome aboard the");
    println!("{}", msg.greet("Rusty Container!!"));
}