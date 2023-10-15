// Day 42 of Rust, New Types partern leverages the rust type system to
// make programs more reliable and easier to manage
// its possible to create "NeverZero" with any value by not using "new" function,
// however this is only possible within a single module. we must use "pub" before
// we can access "new" function in different mod

#[derive(Debug, Copy, Clone)]
struct NeverZero(i32); //tuple structure

impl NeverZero {
    //new type in action
    fn new(i: i32) -> Result<Self, String> {
        if i == 0 {
            Err("cannot be zero".to_owned())
        } else {
            Ok(Self(i))
        }
    }
}

fn divide(a: i32, b: NeverZero) -> i32 {
    let b = b.0;
    a / b
}

fn main() {
    match NeverZero::new(3) {
        Ok(num) => println!("{:?}", divide(10, num)),
        Err(e) => println!("{:?}", e),
    }
}
