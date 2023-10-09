// Day of 37 of Rust, Trait objects offer a way to dynamically change program behavior at runtime.
//  They are dynamically allocated objects whose type are calculated at runtime and not compile time,
//   they could be thought of as "Runtime generics" but more flexible than generics. To determine
//   information of traits Objects at runtime we use "Dynamic Dispatch" while that of normal traits are called "Static Dispatch ".

// One benefit of using Trait Object is it allows mixed types in a collection(normally collection require each item be of the same type)
// but with trait object each item can have different types and exists in same collection. Trait objects makes it easier to work with similar
//  data types that have similar purposes for example, we can place employee, managers and supervisors in the same vector.

// Trait object enables polymorphic behavior since we can have a vector with multiple different types, we can create a program that behaves differently for each type in the vector.

// Traits objects works well with rapidly evolving program requirements, we create a new structure, we can turn it into a trait object and just use it with other trait objects

// Creating A Trait Object

// first define a trait
trait Clicky {
    fn click(&self);
}

// create an first Object
struct Keyboard;

// implement the trait on the first Object
impl Clicky for Keyboard {
    fn click(&self) {
        println!("click Enter")
    }
}

// utilizing traits Object in a function using the borrow synthax
fn borrow_clicky(obj: &dyn Clicky) {
    obj.click();
}

// utilizing traits Object in a function using the move synthax
// if you wanna move object, use "Box trait"
fn move_clicky(obj: Box<dyn Clicky>) {
    obj.click();
}

fn main() {
    // creating an instance of the Trait Object above using first synthax
    let keyb: Keyboard = Keyboard;
    let keyb_obj: &dyn Clicky = &keyb;

    // creating an instance of the Trait Object above using second synthax
    //this sythax is not really recommended because storing a reference in a vector is not always useful
    let keyb: &dyn Clicky = &Keyboard;

    // creating an instance of the Trait Object above using third synthax
    // putting something in a box gives us the ability to move things around
    //notice we borrowed the "Keyboard" object in the two sythaxs above
    let keyb: Box<dyn Clicky> = Box::new(Keyboard); //creates a keyboard and place it in a box

    // using borrowed trait object with in a function
    // This works because "Keyboard" implemented the "click trait"
    let keyb = Keyboard;
    borrow_clicky(&keyb);

    // using moved trait object with in a function
    let keyb = Box::new(Keyboard);
    move_clicky(keyb);
}
