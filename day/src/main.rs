// Day 18 of Rust, I looked Hashmap Data structures, Hashmap is a collection that stores
// data as key-value pairs.
// Data is located using the key and the data is the value. hashmap are very fast to retrive
// data using the key. In hashmaps, Data is stored Randomly contrary to vector where everything comes in same order
// we use the "mut" keyword because we will have to manually insert the data

// fn main() {
//     // use std::collections::HashMap;
//     // use std::collections::HashMap;
//     use std::collections::hash_map::HashMap;
//     let mut people = HashMap::new();
//     people.insert("Susan", 21);
//     people.insert("Ed", 13);
//     people.insert("Will", 14);
//     people.insert("Cathy", 22);
//     people.remove("Susan");

//     match people.get("Ed") {
//         Some(age) => println!("age = {:?}", age),
//         None => println!("Not found"),
//     }
// }

use std::collections::hash_map::HashMap;

#[derive(Debug)]
struct Contents {
    content: String,
}

fn main() {
    // we use the "mut" keyword because we will have to manually insert the data into the hashmap
    let mut lockers = HashMap::new();
    lockers.insert(
        1,
        Contents {
            content: "polo".to_owned(),
        },
    );
    lockers.insert(
        2,
        Contents {
            content: "gym_shorts".to_owned(),
        },
    );
    lockers.insert(
        3,
        Contents {
            content: "shoes".to_owned(),
        },
    );

    // iterating over the map
    for (locker_id, element) in lockers.iter() {
        println!("id: {:?}, element: {:?} ", locker_id, element)
    }
}
