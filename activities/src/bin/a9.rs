// Topic: Data management using tuples
//
// Requirements:
// * Print whether the y-value of a cartesian coordinate is
//   greater than 5, less than 5, or equal to 5
//
// Notes:
// * Use a function that returns a tuple
// * Destructure the return value into two variables
// * Use an if..else if..else block to determine what to print

fn return_tuple() -> (i32, i32) {
    (45, 90)
}

fn main() {
    let (x_value, y_value) = return_tuple();
    if y_value > 5 {
        println!("{y_value:?} is greater than 5");
    } else if y_value < 5 {
        println!("{y_value }  is less than 5")
    } else {
        println!("{y_value} is equal to 5")
    }
}
