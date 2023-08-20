// Day 3 of Rust, The main function acts as the entry point to the application, This means "main function" gets executed first in every Rust program,
//  The basic operators in Rust are, sum, subtraction, division, multiplication and remainder division(modulo)

// fn main() {
//     let sum = 3 + 3;
//     let sub = sub(1, 9);
//     let division = 5 / 3;
//     let mult = 2 * 3;
//     let rem = 7 % 3;

//     println!(" Output: {sum},  {sub}, {division}, {mult}, {rem}");
// }

// fn sub(num_a: i32, num_b: i32) -> i32 {
//     num_a - num_b
// }

// I learnt about descision making in Rust.
// match expressions are similar to if--else expressions except that Match expressions are exhaustive(meaning all options must be accounted for)
// Match expressions are used to add logic to programs

fn main() {
    let value_bool = true;
    match value_bool {
        true => println!("its true"),
        false => println!("its false"),
    }

    let another_int = 3;
    match another_int {
        1 => println!("its 1"),
        2 => println!("its 2"),
        3 => println!("its 3"),
        // underscore(_) means everything else
        _ => println!("its obviously something else"),
    }

    let names = "CC";
    match names {
        "CC" => println!("I belive you called me"),
        "Dom" => println!("This guy over there is the guy you want"),
        "Sara" => println!("Oh, Your wife's name is Sara"),
        _ => println!("Nice to meet ya new coder"),
    }
}

// fn main() {
//     let another_int = 3;
//     match another_int {
//         1 => println!("its 1"),
//         2 => println!("its 2"),
//         3 => println!("its 3"),
//         // underscore(_) means everything else
//         _ => println!("its obviously something else"),
//     }
// }

// fn main() {
//     let names = "CC";
//     match names {
//         "CC" => println!("I belive you called me"),
//         "Dom" => println!("This guy over there is the guy you want"),
//         "Sara" => println!("Oh, Your wife's name is Sara"),
//         _ => println!("Nice to meet ya new coder"),
//     }
// }
