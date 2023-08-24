// Topic: Working with expressions
//
// Requirements:
// * Print "its big" if a variable is > 100
// * Print "its small" if a variable is <= 100
//
// Notes:
// * Use a boolean variable set to the result of
//   an if..else expression to store whether the value
//   is > 100 or <= 100
// * Use a function to print the messages
// * Use a match expression to determine which message
//   to print

// Expressions
fn print_messages(var: bool) {
    match var {
        true => println!("it bigger sha too"),
        false => println!("its small sha"),
    }
}

fn main() {
    let bool_var = 100;

    let boolean_var = if bool_var > 100 { true } else { false };
    print_messages(boolean_var);

    // alternatively
    let bol_var = bool_var > 100;
    print_messages(bol_var);
}
