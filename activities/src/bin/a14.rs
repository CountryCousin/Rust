// Topic: Strings
//
// Requirements:
// * Print out the "name" and "favorite colors" of people "aged 10 and under"
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function
Ticket { price: 50, .. } => println!("event @ 30: {:?} ", 50)

struct Person {
    name: String,
    age: i32,
    color: String,
}

fn print_name(name: &str) {
    println!("name is: {:?}", name);
}
fn print_color(color: &str) {
    println!("color is: {:?}", color)
}

fn main() {
    let persons = vec![
        Person {
            name: "Luiz".to_owned(),
            age: 23,
            color: "Red".to_owned(),
        },
        Person {
            name: "Lakan".to_owned(),
            age: 13,
            color: "Blue".to_owned(),
        },
        Person {
            name: "Raghav".to_owned(),
            age: 2,
            color: String::from("Brown"),
        },
    ];

    // * Iterate through the vector using a for..in loop
    // * Use an if expression to determine which person's info should be printed
    // * The name and colors should be printed using a function

    for person in persons {
        if person.age <= 10 {
            print_name(&person.name);
            print_color(&person.color)
        }
    }
}
