// Day 12 of Rust,I learnt about #[derive(Debug)]. functionalities can be automatically implemented
// for enums and structs by using "derive" macro, here we are implementing "Debug" functionality
// derive is a special macro thats applied to enums and structs.
// if enums is contained in the enums, then both the enums and stucts must
// implement #[derive(Debug)] to avoid compile time error
// in addition, I looked up clone and copy derive: This instructs the compiler to make a copy
// of a data say an enum when we want to store it in a struct meaning ownerhsip is no longer moved but its copied

// #[derive(Debug)]
// enum Position {
//     Manager,
//     Supervisor,
//     Worker,
// }

// #[derive(Debug)] //aids to print the entire struct
// struct Employee {
//     position: Position,
//     work_hours: i64,
// }
// fn main() {
//     let me = Employee {
//         position: Position::Worker,
//         work_hours: 40,
//     };

//     println!("Them Position is: {:?}", me.position); //prints a postion from the enum

//     println!("The entire Struct is: {:?} ", me); //prints the entire struct
// }

#[derive(Debug, Clone, Copy)]
enum Position {
    Manager,
    Supervisor,
    Worker,
}

#[derive(Debug, Clone, Copy)] //aids to copy the entire struct
struct Employee {
    position: Position,
    work_hours: i64,
}

fn print_employee(emplyee: Employee) {
    //note: "&" was not used here
    println!("{:?}", emplyee)
}
fn main() {
    let me = Employee {
        position: Position::Worker,
        work_hours: 40,
    };
    print_employee(me); //makes a copy of "me"
    print_employee(me); //makes a copy of "me"
}
