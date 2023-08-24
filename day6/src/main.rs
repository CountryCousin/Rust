// Day 6 of Rust, I explored tuples.
// A tuple is a kind of "record"(more like a line of info on a piece of paper)
//  these records can be destructed easily into variables
// Rust is an expression-base language, this means most things are evaluated and
// return some value as a result, expressions can be used for nesting logic.

fn check() -> (i32, i32, i32) {
    (3, 4, 5)
}
fn main() {
    let position = (2, 3);
    // access using dot notation
    println!(" {:?}, {:?}", position.0, position.1);

    let (x, y) = (4, 5);
    // destructuring tuples
    println!(" {:?}, {:?}", x, y);

    let numbers = check();
    let (x, y, z) = check();

    println!(" {:?}, {:?} ", x, numbers.0);
    println!("{:?} {:?} ", y, numbers.1);
    println!("{:?} {:?} ", z, numbers.2);
}

// #[derive(Debug)]
// enum Access {
//     Full,
// }

// fn check() -> (i32, i32, i32) {
//     (3, 4, 5)
// }
// fn main() {
//     let numbers = check();
//     let (x, y, z) = check();
//     println!(" {:?}, {:?} ", x, numbers.0);
//     println!("{:?} {:?} ", y, numbers.1);
//     println!("{:?} {:?} ", z, numbers.2);

//     let (employee, access) = ("Jake", Access::Full);
//     print!(" {employee:? } has {access:?} ")
// }

////////////////////
// EXPRESSIONS
//////////////////
// enum Menu {
//     Burger,
//     Fries,
//     Drink,
// }

// enum Access {
//     Admin,
//     Manager,
//     User,
//     Guest,
// }

// fn main() {
//     // Expressions
//     let your_num = 5;
//     let confirm_your_num = if your_num > 10 {
//         true;
//     } else {
//         false;
//     };

//     // alternatively
//     let confirm_your_num = your_num < 7;

//     let my_num = 3;
//     let message = match your_num {
//         1 => "Hello",
//         _ => "hello",
//     };

//     let paid = true;
//     let item = Menu::Drink;
//     let drink_type = "water";
//     let order_placed = match item {
//         Menu::Drink => {
//             if drink_type == "water" {
//                 true
//             } else {
//                 false
//             }
//         }
//         _ => true,
//     };

//     // access seceret file
//     let clearance_level = Access::Guest;
//     let can_access_file = match clearance_level {
//         Access::Admin => true,
//         _ => false,
//     };
// }
