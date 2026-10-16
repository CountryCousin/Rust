use std::env::{args, Args};

// fn main() {
//   let args: Args = args();
//   println!("{:?}", args);
// }

// fn main() {
//   let mut args: Args = args();

//   let first: String = args.nth(1).unwrap();
// }

// fn main(){
//     let mut args: Args = args();
//     // The first argument is the location of the compiled binary, so skip it 
//     let first: String = args.nth(1).unwrap();
//     // After accessing the second argument, the iterator's next element becomes the first
//     let operator: String = args.nth(0).unwrap();
//     let second: String = args.nth(0).unwrap();

//     println!("{} {} {}", first, operator, second);
// }

// Parse Strings into Numbers 
// fn main(){
//     let mut args: Args = args();
//     let first: String = args.nth(1).unwrap();
//     let operator: String = args.nth(0).unwrap();
//     let second:String= args.nth(0).unwrap();

//     // now lets pass the strings into numbers 
//     let first_num: f32 = first.parse::<f32>().unwrap();
//     let second_num: f32 = second.parse::<f32>().unwrap();

//     println!("{} {} {}", first_num, operator, second_num);
// }

// convert operator into a char
fn main(){
    let mut args: Args = args();
    let first: String = args.nth(1).unwrap();
    let operator: char = args.nth(0).unwrap().chars().next().unwrap();
    let second:String= args.nth(0).unwrap();

    // now lets pass the strings into numbers 
    let first_num: f32 = first.parse::<f32>().unwrap();
    let second_num: f32 = second.parse::<f32>().unwrap();
    let result: f32 = operation(operator, first_num, second_num);

    println!("{} {} {} = {}", first_num, operator, second_num, result);
}

// Perform Basic Arithmetic Operations 
fn operation( operator: char, first_num: f32, second_num: f32) -> f32 {
    match operator {
        '+' => first_num + second_num,
        '-' => first_num - second_num,
        '*' | 'X' | 'x' => first_num * second_num,
        '/' => first_num / second_num,
        _=> panic!("Invalid operator")
    }
}

// Format the Output 
fn output(first_num: f32, operator: char, second_num: f32, result: f32) -> String {
  format!(
    "{} {} {} = {}",
    first_num, operator, second_num, result
  )
}