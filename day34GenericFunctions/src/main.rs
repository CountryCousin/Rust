// Day 34 of Rust, To use Generic functions, first we define a trait and its implementation
// Then we write a generic function that implements the trait.

trait CheckIn {
    fn check_in(&self);
    fn process(&self);
}

struct Pilot;
impl CheckIn for Pilot {
    fn check_in(&self) {
        println!("checked in as Pilot");
    }

    fn process(&self) {
        println!("Pilot enters the cockpit");
    }
}
struct Passenger;
impl CheckIn for Passenger {
    fn check_in(&self) {
        println!("checked in as Passenger");
    }

    fn process(&self) {
        println!("Passenger takes a seat");
    }
}
struct Cargo;
impl CheckIn for Cargo {
    fn check_in(&self) {
        println!("cargo checked in");
    }

    fn process(&self) {
        println!("cargo moved to storage");
    }
}
// Below "T" is the function parameter which must implement trait "CheckIn"
fn process_item<T: CheckIn>(item: T) {
    item.check_in();
    item.process();
}
fn main() {
    let obi = Passenger;
    let ada = Pilot;
    let load_one = Cargo;
    let load_two = Cargo;

    process_item(obi);
    process_item(ada);
    process_item(load_one);
    process_item(load_two);
}
