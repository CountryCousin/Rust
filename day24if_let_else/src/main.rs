// Day 24 of Rust, " if let" works just like match expression
// except its more useful if we just wanna capture the true component of an expression.

enum Color {
    Red,
    Blue,
    Green,
}

fn main() {
    let this_user = Some("Tom");

    if let Some(user) = this_user {
        println!("user: {:?}", user);
    } // else block is totally optional here

    let red_or_blue = Color::Blue;
    if let Color::Blue = red_or_blue {
        println!("its Blue");
    } else {
        // else block is totally optional here
        println!("its Not Blue");
    }
}
