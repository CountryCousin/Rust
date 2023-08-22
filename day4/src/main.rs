// fn main() {
//     let mut count = 1;
//     loop {
//         println!("{count}. Hello Mr. Loop");
//         if count == 4 {
//             break;
//         }
//         count += 1;
//     }
// }

// fn main() {
//     let mut count = 7;

//     loop {
//         println!("{count}. Countdown to extinction");

//         if count == 0 {
//             break;
//         }
//         count -= 1;
//     }
//     println!("Extinted!");
// }

// while loop

// fn main() {
//     let mut i = 1;
//     while i <= 3 {
//         println!("{i:?}");
//         i += 1;
//     }
// }

// Day 4 of Rust, I explored Enums, structs and tuples
// Enums(Enumerations) allows us to work with data in a type safe manner
// Enum is one data type of different possibilities(it contains different members) called variants.
// enum Location {
//     Up,
//     Down,
//     Left,
//     Right,
// }

// fn main() {
//     fn tell_location(vee: Location) {
//         match vee {
//             Location::Up => "up",
//             Location::Down => "Down",
//             Location::Left => "Left",
//             Location::Right => "Right",
//         }
//     }
// }

enum Direction {
    Left,
    Right,
}

fn main() {
    let go = Direction::Left;

    match go {
        Direction::Left => println!("go left"),
        Direction::Right => println!("go right"),
    }
}
