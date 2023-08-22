// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print

enum Colors {
    Red,
    Yellow,
    Black,
}

fn color_name(color: Colors) {
    match color {
        Colors::Red => println!("Red"),
        Colors::Yellow => println!("Yello"),
        Colors::Black => println!("Black3"),
    }
}
fn main() {
    color_name(Colors::Black);
}
