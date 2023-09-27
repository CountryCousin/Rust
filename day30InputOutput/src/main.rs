// Day 30 of Rust, user input can be gathered using the "io" module so that programs can
// be interactive.

use std::io;

// program that returns two inputs from a user

fn get_input() -> io::Result<String> {
    // buffer is some space set aside that some other functionality can use and operate with
    let mut buffer = String::new();

    // "?" will automatically return an error if the function fails.
    io::stdin().read_line(&mut buffer)?; // "read_line" function reads input from the terminal

    // "trim()" will remove any white space( white space here invcludes spaces, tab and new-line)
    Ok(buffer.trim().to_owned()) //"trim()" returns a borrowed slice string, use "to_owned" for owned string
}

fn main() {
    let mut all_input = vec![];
    let mut times_input = 0;

    while times_input < 2 {
        match get_input() {
            Ok(words) => {
                all_input.push(words);
                times_input += 1;
            }
            Err(e) => println!("The error is: {:?}", e),
        }
    }

    for input in all_input {
        println!(
            "Original input: {:?}, capitalized input: {:?}",
            input,
            input.to_uppercase()
        );
    }
}
