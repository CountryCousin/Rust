// Day 20 of Rust, I learnt about Map combinator and how it can be used easily to translate
// data. The awesome thing with map is, it only applies if there is a value there, so if the
// value is NONE, it won't execute

// function that returns number
fn num_num() -> Option<i32> {
    Some(3)
}

// function that checks for a word
fn check_word() -> Option<String> {
    Some("Biggie".to_owned())
}

fn main() {
    // using map and closures
    let add_one = num_num().map(|num| num + 1);
    println!("Final value is: {:?}", add_one);

    //    maps can be chained
    let check_length = check_word().map(|word| word.len()).map(|length| length * 3);
    println!("Length of word is: {:?}", check_length);
}
