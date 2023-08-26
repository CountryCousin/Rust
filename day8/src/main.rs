// Managing Memory continues
// Day 8 of Rust is all about ownership.
// In Rust, the Memory can either be "moved" or "borrowed" from the owner
// we can borrow or reference data by using "&"
// The "owner" of memory is responsible for cleaning up the memory, "owner" in Rust is simply a function.
// Rust utilizes an "ownership" model to manage memory, ownership help ensre that compiled code
// execute correctly under differnt circumstances, programs must track their memory usage, without this a memory "leak" occurs
// memory leak is when a program fails to track which memory is been used.

// Moving Data In Rust
// enum Neon {
//     Bright,
//     Dull,
// }

// fn display_neon_light(light: Neon) {
//     match light {
//         Neon::Bright => println!("Its bright"),
//         Neon::Dull => println!("its dull"),
//     }
// } // at this stage, display_neon_light() deletes data(it's the end of the function) it owned
// fn main() {
//     let bright = Neon::Bright; //"bright" is owned by the main function here

//     display_neon_light(bright); //"bright" was moved from "main" into "display_neon_light()"
//     display_neon_light(bright); //"bright" is no longer available here
// }

// Borrowing or Referencing In Rust
enum Neon {
    Bright,
    Dull,
}

fn display_neon_light(light: &Neon) {
    match light {
        Neon::Bright => println!("Its bright"),
        Neon::Dull => println!("its dull"),
    }
} // at this stage, display_neon_light() can't delete data because it doesn't own the data

fn main() {
    let bright = Neon::Bright; //"bright" is owned by the main function here

    display_neon_light(&bright); //"bright" was borrowed from "main()" by "display_neon_light()"
    display_neon_light(&bright); //"bright" is always available, you can borrow as many as you wish
}
