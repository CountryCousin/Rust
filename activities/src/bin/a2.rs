// Topic: Basic arithmetic
//
// Program requirements:
// * Displays the result of the sum of two numbers
//
// Notes:
// * Use a function to add two numbers together
// * Use a function to display the result
// * Use the "{:?}" token in the println macro to display the result

fn add_two_numbers(first_no: i32, second_no: i32) {
    let answer = first_no + second_no;
    println!("{:?}", answer);
}

fn display_result() {
    add_two_numbers(2, 3);
}

fn main() {
    // add_two_numbers(2, 3);
    display_result();
}
