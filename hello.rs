// Must declare a module
use std::fmt;

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}

impl fmt::Display for Person<'_> { // anonymous lifetime
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "My name is {0}, I'm {1} years old", self.name, self.age)
    }
}

fn main() {
    let world = "World";
    println!("Hello {}!", world);
    println!("I'm a 초보 Rustacean!"); // Test UTF-8

    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    // println!("{}", peter);
    //                ^^^^^ `Person<'_>` cannot be formatted with the default formatter
    println!("{:?}", peter);
    println!("{:#?}", peter);
}
