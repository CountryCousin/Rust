// Day 13 of Rust, took some time to understand annotations are required by function signatures, we can anotate explicitely or implicitely(Types can be infered by the compiler)
// in Rust
// Also looked up some advanced enums; data can be optionally associated with variants of enums

enum Discount {
    Percent(i32),
    Flat(i32),
}

struct Ticket {
    event: String,
    price: i32,
}

fn main() {
    let n = 5;
    match n {
        3 => println!("its three"),
        // _ => println!("other numbers is: {:?}", n),
        other => println!("other numbers is: {:?}", other), //this another cool alternative
    }

    let flat = Discount::Flat(2);
    match flat {
        // Discount::Flat(_) => println!("its : Flat 2"), //use "_" to ignore passing the value of the variant
        Discount::Flat(2) => println!("its : Flat 2"), //prints 2 only when 2 is passed
        Discount::Flat(amount) => println!("Flat discount: {:?}", amount), //giving it a variable name "amount" makes it dynamic(it can take any value so long its of the variant "Flat")
        _ => (), //() means we will return nothing
    }

    // using match operation alongside enums and struct
    let concert_ticket = Ticket {
        event: "concert".to_owned(),
        price: 50,
    };

    match concert_ticket {
        Ticket { price: 50, event } => println!("event @ 30: {:?} ", event),
        // Ticket { price: 50, .. } => println!("event @ 30: {:?} ", 50), // this will print the price "50"
        Ticket { price, .. } => println!("price = {:?} ", price), // two dots ".." means any other field should be ignored
    }
}

// fn print_it(msg: &str, count: i32) {} //function that's annotated.

// enum Mouse {
//     LeftClick,
//     RightClick,
//     MiddleClick,
// }
// fn main() {
//     let num: i32 = 15; //explicitly stated
//     let a = 'q';
//     let left_click: Mouse = Mouse::LeftClick; //explicitly stated
// }
