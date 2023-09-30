// Day 33 of Rust, Generic functions allows you to write functions that
// work with muliple types of data, its a function that allows multiple different data
// types to be used as a function parameter instead of providing a data type as we
// normally would as a function parameter, we provide a trait

// Generic synthax
// first synthax 
fn function<T: Trait1, U: Trait2>(param1: T, param2: U) {
    /*body */
}

// Secong synthax 
fn function(param: impl Trait) {}

// Alternatively
fn function<T, U>(param1: T, param2: U)
where
    T: Trait1 + Trait2,
    U: Triat1 + Trait2 + Trait3,
{
    /*body */
}

fn main() {}
