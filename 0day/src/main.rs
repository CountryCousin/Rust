// This is a Rust program that prints "Hello, world!" to the console.
fn main() {
    // This is a macro that prints text to the console.
    println!("Hello, world!");

    // Variables in Rust are immutable by default.
    // To make a variable mutable, use the 'mut' keyword.
    let mut x = 5;

    // Rust has type inference, so you don't always need to specify types.
    // However, you can use the 'let' keyword to explicitly declare a variable's type.
    let y: i32 = 10;

    // Rust also has constants, which are always immutable and must be explicitly typed.
    const PI: f32 = 3.14159;

    // Rust has a strong focus on safety, so it has a unique ownership system.
    // The following line will give an error because we are trying to use a variable that we have already moved.
    // let z = x;
    // println!("z: {}", z);

    // Instead, we can use references to allow multiple variables to access the same data.
    // '&' creates a reference to a value, and the 'mut' keyword allows us to modify the referenced value.
    let mut y = 5;
    let y_ref = &mut y;
    *y_ref += 5;

    // Rust also has a powerful error handling system, using the 'Result' type and the '?' operator.
    // Here we try to parse a string as an integer, and handle any errors that might occur.
    let input_string = "123";
    let input_int: Result<i32, std::num::ParseIntError> = input_string.parse();
    match input_int {
        Ok(n) => println!("Parsed integer: {}", n),
        Err(e) => println!("Error parsing integer: {}", e),
    }
}
