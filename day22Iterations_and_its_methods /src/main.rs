//Day 22 of Rust, Iterators are ways to traverse and manipulate data structures.
// iter()  and collect() operates generically on any type of data structures.
// iterators don't execute anything, its just a configuration step.

fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    // regular methods
    // let mut times_two= vec![];
    // for num in numbers {
    //     times_two.push(num * 2);
    // }
    // println!("The output is : {:?}", times_two);

    // alternatively:
    // collect() converts the output to a vector(it collects the items into a new vector since thats what we specified in the anotation)

    // map method:
    let times_two: Vec<_> = numbers.iter().map(|num| num * 2).collect();
    println!("The output of times two: {:?}", times_two);

    // filter method: filters items and keeps the ones you want.
    let filter_num: Vec<_> = numbers.iter().filter(|num| num <= &&3).collect();
    println!("The output of filter num: {:?}", filter_num);

    // find method:
    let find_num = numbers.iter().find(|num| num == &&3); //this returns an optional type(Reason: its possible the num we are looking for doesn't exist in the vector)
                                                          // let find_num: Option<i32> = numbers.iter().find(|num| num == &&3).copied(); // this works too
    println!("The output of find num: {:?}", find_num);

    // count methods: counts the number of elements in the vector
    let count_num = numbers.iter().count();
    println!(" The total count is: {:?}", count_num);

    // last method returns the last element in the iteration
    let last_num = numbers.iter().last(); //returns optional type(reason: its possible to create an empty vector)
    println!(" The total last num is: {:?}", last_num);

    // min method
    let min_num = numbers.iter().min(); // this returns optional value
    println!(" The min num is: {:?}", min_num);

    let max_num = numbers.iter().max(); // returns an optional value
    println!(" The max num is: {:?}", max_num);

    // take method returns the desired number of items we want.
    let take_num: Vec<_> = numbers.iter().take(3).collect();
    println!("The taken values are: {:?}", take_num)
}
