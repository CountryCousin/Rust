use rand::random;

fn main() {

    let rand_float: f64 = random(); // generate a random float between 0.0 and 1.0
    println!("Random float is : {}", rand_float * 100.0); // multiply by 100 to get a float between 0.0 and 100.0

    let rand_int: u8 = random::<u8>(); // generate a random integer between 0 and 255
    println!("Random integer is : {}", rand_int);
}
