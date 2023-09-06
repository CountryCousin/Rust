// Day 15 of Rust: utilizing the automatic documentation function
//  provided by the Rust toolchain one can generate documentations for programs
// by using "///"(three forward slashes).
// we can generate the docs running "cargo doc --open"

/// A collection of favorite color
enum Color {
    Red,
    Blue,
}

/// A piece of mail
struct Mail {
    address: String,
}
/// Adds two intergers
fn add(a: i32, b: i32) -> i32 {
    a + b
}
fn main() {}
