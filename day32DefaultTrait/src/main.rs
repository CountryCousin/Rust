// Day 32 of Rust, Default traits is used to create new structures and enumerations with
// a default value. One of the reasons to use default is, there are other crate within the
// "std lib" that might attempt to use "defaults" when applicable. In general its a great
// idea to implement defaults for enums and structs where it would make sense to have a default value
// because it makes code easier to use, there are no downsides to having defaults as long as
// it makes sense for your use case

struct Package {
    weight: f64,
}

// if a user does not supply a specific weight, Defualt property comes in.
impl Package {
    fn new(weight: f64) -> Self {
        Self { weight }
    }
}

// implementing default for the Package structure.
impl Default for Package {
    // "default" is the only function available withing the Defualt Traits
    fn default() -> Self {
        Self { weight: 3.0 } //default value set.
    }
}
fn main() {
    let package = Package::default();
}
