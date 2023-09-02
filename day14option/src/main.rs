// Day 14 of Rust, I explored how optional data are managed using the option type.
// The option type could be one of two things(some data of a specified type or Nothing)
// it is used in scenarios where data may not be required or is unavailable at the time eg:
// when you are unable to find something,
// when you may Ran out of items in a list
// when you have a struct with forms and the form field are not completely field out

// Definition of the Option type

// enum Option<T> {
//     // T is a placeholder for some data type
//     Some(T), //represent some data
//     None,    // represents No data
// }

// struct Customer {
//     age: Option<i32>,
//     email: String,
// }

// fn main() {
//     let mark = Customer {
//         age: Some(34),
//         email: "mark@gamil.com".to_owned(),
//     };

//     let rosie = Customer {
//         age: None,
//         email: "rosie@gmail.com".to_owned(),
//     };

//     match rosie.age {
//         Some(age) => println!("Rosie's age is: {:?}", age),
//         None => println!("Rosie never provided her age."),
//     }
// }

// struct GroceryList {
//     name: String,
//     qty: i32,
// }

// fn find_quantity(name: &str) -> Option<i32> {
//     let groceries = vec![
//         GroceryList {
//             name: "banana".to_owned(),
//             qty: 5,
//         },
//         GroceryList {
//             name: "Cashew".to_owned(),
//             qty: 13,
//         },
//         GroceryList {
//             name: "Orange".to_owned(),
//             qty: 9,
//         },
//     ];

//     for grocery in groceries {
//         if grocery.name == name {
//             return Some(grocery.qty); //return keyword makes it possible to make an early return from a function.
//         }
//     }
//     None
// }

// fn main() {
//     find_quantity("banana");
// }

struct Survey {
    q1: Option<i32>,
    q2: Option<bool>,
    q3: Option<String>,
    others: Option<i32>,
}
fn main() {
    let response = Survey {
        q1: Some(3),
        q2: Some(false),
        q3: Some("A".to_owned()),
        others: None,
    };

    match response.q1 {
        Some(ans) => println!("The answer is : {:?}", ans),
        None => println!("q1 is don't have an answer"),
    }

    match response.q2 {
        Some(ans) => println!("The answer is : {:?}", ans),
        None => println!("q2 is don't have an answer"),
    }

    match response.q3 {
        Some(ans) => println!("The answer is : {:?}", ans),
        None => println!("q3 is don't have an answer"),
    }

    match response.others {
        Some(ans) => println!("The answer is : {:?}", ans),
        None => println!("Others is don't have an answer"),
    }
}
