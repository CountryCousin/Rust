// Day 2 of Rust, I started with Control flow which are "if", "else if" and "else".

// fn main() {
//     let toms_age = 500;

//     if toms_age > 700 {
//         if toms_age > 850 {
//             println!(" {toms_age} ?, Tom must be an alien");
//         } else {
//             println!(" {toms_age} ?, Tom must be an vampire");
//         }
//     } else if toms_age > 400 {
//         println!(" {toms_age}?, Tom must be an alien sha");
//     } else {
//         println!("{toms_age} ?, Tom is a hmmmmm!");
//     }
// }

// There are two types of loops in Rust: the "while loop" and the "infinite loop(uses the "loop"keyword)""

fn main() {
    // let a = 0;
    // let b = -5;

    fn add_dis(a: i32, b: i32) -> i32 {
        a + b
    }
    let mut summ = add_dis(0, -5);

    // the "inifinite loop"
    loop {
        if summ == 6 {
            break;
        }
        println!("{:?}", summ);
        summ += 1;
    }

    // the "while" loop.
    let mut first_value = 0;

    while first_value != 5 {
        println!("{:?}", first_value);
        first_value = first_value + 1;
    }
}
