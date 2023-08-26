// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
// * Create a function to display the quantity, with the struct as a parameter
// * Create a function to display the id number, with the struct as a parameter

struct Grocery {
    quantity: i32,
    id: i32,
}

fn display_quantity(quatity: &Grocery) {
    println!(" The is the quantity: {:?}", quatity.quantity);
}

fn display_id(id: &Grocery) {
    println!("This is the id: {:?}", id.id);
}
fn main() {
    let grocery = Grocery { quantity: 4, id: 5 };
    display_quantity(&grocery);
    display_id(&grocery);
}
