// Day 11 of Rust, strings are used to store text infomation such
// as names and words. there are multiple types of strings in Rust
// two most commonly used are:
// String-owned and
// &str- borrowed string slice,
// strings stored in a struct must be owned string
// &str is used when passing string as a parameter to a function(its more efficient)

fn print_str(data: &str) {
    println!("{:?}", data);
}
fn main() {
    print_str("its a string slice"); // by default strings created this way are automatically referenced/borrowed .

    // if we want to create an owned string:
    let owned_string = "owned string".to_owned(); //used "to_owned()" to create owned string
    let second_owned = String::from("second string"); //used "String::from()"

    // calling the methods
    print_str(&owned_string);
    print_str(&second_owned)
}

// struct PersonItem {
//     name: String,
//     count: i32,
// }

// fn print_name(name: &str) {
//     println!("name: {:?}", name)
// }

// fn main() {
//     let persons = vec![
//         PersonItem {
//             name: "Tom".to_owned(),
//             count: 4,
//         },
//         PersonItem {
//             name: String::from("Odell"),
//             count: 2,
//         },
//     ];

//     for person in persons {
//         print_name(&person.name);
//         println!(" count: {:?}", person.count);
//     }
// }
