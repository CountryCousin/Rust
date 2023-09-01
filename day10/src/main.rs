// Day 10 of Rust, looked up Vector; vector is a data structure for storing multiple
// pieces of Data, the data must be of the same type, They are used for list of
// information, you can add, remove and traverse the entries.

// NB: macros in Rust expands to actual codes

fn main() {
    let them_numbers = vec![1, 2, 3];

    let mut them_numbers = Vec::new();
    them_numbers.push(1);
    them_numbers.push(2);
    them_numbers.push(3);
    them_numbers.pop(); //removes the last element in the vector
    them_numbers.len(); // returns the lenght of the vector

    //accessing item from a vector using the slice notation
    let second = them_numbers[1]; // returns 2

    // iterating over a vector
    for them in them_numbers {
        println!("{}", them);
    }
}
