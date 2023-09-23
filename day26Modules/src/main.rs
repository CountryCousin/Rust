// Day 26 of Rust, Modules is used to organise code, it helps keep code self contained
// and easier to follow and comprehend. Each module can be thought of as a different file.
// if we wanna use say hashMap within a mod, we must import the HashMap libray within
// the mod scope

mod greet {
    //import must be within the mod
    //use std::collections::HashMap; // how to use HashMap within a mod
    pub fn hello() {
        println!("Say hello to Jon Snow");
    }

    fn goodbye() {
        println!("Goodbye to the Red Woman");
    }
}

mod maths {
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    pub fn sub(a: i32, b: i32) {
        let result = a - b;
        println!("{:?}", result);
    }
}

fn main() {
    use greet::hello; //make mod availaible for usuage by importing it, its good practice to
                      // list the function you wanna use
                      //use greet::*; //gives access to all the functions within the greet mod.
    hello();

    // alternatively, this sythax works too:
    // greet::hello(); //calling hello from the greet module

    maths::sub(2, 1);
}
