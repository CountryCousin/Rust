// Day 19 of Rust, I went over Basic Closures which means an
// anonymoius function. since closures are anonymous, they must be
// defined within another function

// traditional function declaration
fn subtract(a: i32, b: i32) -> i32 {
    b - a
}
fn main() {
    // calling the traditional function
    let subtract_it = subtract(6, 2);

    // Declaring a closure
    // let sub = |a: i32, b: i32| -> i32 { a + b }; // This could work but not necessary

    // common method of declaring closures
    let sub = |a, b| a - b;

    // calling our anonymous function
    let difference = sub(4, 1);
    println!("The difference is: {:?}", difference);
}
