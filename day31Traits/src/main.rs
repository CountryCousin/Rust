// Day 31 of Rust, Traits is a way to specify that some functionality exists.
// They are used to stardardize functionality across multiple different types.
// stardardization permits functions to operate on multiple different types

trait Noise {
    fn make_noise(&self);
    fn keep_quiet(&self);
    fn all_quiet(&self) -> bool;
}

struct Person;
impl Noise for Person {
    fn make_noise(&self) {
        println!("Say Loud Hello");
    }
    fn keep_quiet(&self) {
        println!("All is quiet");
    }
    fn all_quiet(&self) -> bool {
        true
    }
}

fn sound(noisy: impl Noise) {
    noisy.make_noise();
    noisy.keep_quiet();
    noisy.all_quiet();
}
fn main() {
    sound(Person {})
}
