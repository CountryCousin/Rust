// Day 27 of Rust, Testing is an important part of software development
// its allows you to know your code is working properly

fn change_cap(word: &str) -> String {
    word.to_uppercase()
}

fn main() {
    change_cap("Beuatiful People");
}

// setting up mod for test

//cfg = configuration: ""cfg" along with "test" tells the compiler this is only
// to be used in test scenarios
#[cfg(test)]
mod test {
    // collection of codes in Rust is called crates
    // in this instance, codes refers to the "main and change_cap" functions above
    use crate::*;

    #[test]
    fn check_all_caps() {
        let result = change_cap("Try me");
        let expected = String::from("TRY ME");
        // assert_eq!: this makes an assrtion that one value is equal to another value.
        assert_eq!(result, expected, "case not what we wanted");
    }
}
