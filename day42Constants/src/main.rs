// Day 41 of Rust, const keyword is used to create constant values within a program, constants aids in getting
// rid off magice numbets, To define a constant , we use all caps, also they defined as global variables

const TOP_SPEED: i32 = 10000; // it must be annotated accordingly

fn stop_speed(speed: i32) -> i32 {
    if speed > TOP_SPEED {
        TOP_SPEED
    } else {
        speed
    }
}
fn main() {
    println!("Hello, CONST!");
}
