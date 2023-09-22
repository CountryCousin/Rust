// Day 25 of Rust, "while let" is used to perfome looping on some specific types of day.
// "while let" is important while working with iterators espcially when they could
// return optional data

fn main() {
    let mut some_data = Some(2);

    while let Some(i) = some_data {
        println!("loop number: {:?}", i);
        some_data = None; // sets the value of "some_data" to Nothing thus breaking out of the loop
    }

    // using while "let with" iterators
    let numbers = vec![8, 3, 9, 10];
    let mut iter_number = numbers.iter();

    while let Some(num) = iter_number.next() {
        //".next" returns an optional value as long as there is value in the vector
        // once we run out of values, "None" kick in and hence breaks the loop.
        println!("num =: {:?}", num);
    }

    println!("Done");
}
