extern crate greeter;

#[test]
fn test_greeting() {
    let hello = greeter::Greeter::new("Dia");
    assert_eq!("Dia dhuit!", hello.greet("dhuit!"));
}