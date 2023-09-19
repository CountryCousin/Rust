// Day 23 of Rust, Range is an automated way to create a range of values.
// Ranges are usefull when you need a list of consecutive values
fn main() {
    let range_num = 1..=3;

    let range = 1..4; // does not include 4

    for num in 1..4 { // does not include 4
        println!("{:?} ", num);
    }

    for character in  'a'..='f'{ //includes f
        println!("{:?}", character)
    }
}
