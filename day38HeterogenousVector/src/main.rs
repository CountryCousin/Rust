// Day 38 of Rust, One benefit of using Trait Object is it allows mixed types in a collection(normally collection require each item be of the same type)
// but with trait object each item can have different types and exists in same collection. Trait objects makes it easier to work with similar
//  data types that have similar purposes.
// Trait object enables polymorphic behavior since we can have a vector with multiple different types, we can create a program that behaves differently for each type in the vector.

// Traits objects works well with rapidly evolving program requirements, we create a new structure, we can turn it into a trait object and just use it with other trait objects

// Implementing Heterogenous Vector

// first define a trait
trait Clicky {
    fn click(&self);
}

// create a first Object
struct Keyboard;

// implement the trait on the first Object
impl Clicky for Keyboard {
    fn click(&self) {
        println!("click Enter")
    }
}

// create second object
struct Mouse;

// implement the trait on the second Object
impl Clicky for Mouse {
    fn click(&self) {
        println!("click Mouse");
    }
}

// create function that works with the  trait objects
fn make_clicks(clickeys: Vec<Box<dyn Clicky>>) {
    for clicker in clickeys {
        clicker.click();
    }
}

fn main() {
    //    first synthax of the Heterogenous Vector
    let keyb: Box<dyn Clicky> = Box::new(Keyboard); //creates a keyboard and place it in a box
    let mouse: Box<dyn Clicky> = Box::new(Mouse);
    // let clickers = vec![keyb, mouse];

    // second sythax of the Heterogenous Vector
    let keyb = Box::new(Keyboard);
    let mouse = Box::new(Mouse);
    let clickers: Vec<Box<dyn Clicky>> = vec![keyb, mouse];

    make_clicks(clickers);
}
