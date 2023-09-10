// Day 21 of Rust, I checked out Option combinators.
// Option combinators makes manipulations of Options easier. some of the commonly
// used functions are as follows
fn main() {
    let a: Option<i32> = Some(4);

    // "is_some()" returns true if there is a data and false if there no data
    let check_is_some = a.is_some();

    //  "is_none" checks to see we have no data, if there no data it returns true and vice versa
    let check_is_none = a.is_none();

    // "map" only applies if there is some data else it won't execute at all
    let check_map = a.map(|num| num + 1);

    // "filter" just like map takes a closure, the body of the function needs to return true
    // or false. if returned value is true, we keep the optional data and if the returned data is
    // false, we discard the optional data(the returned value becomes NONE). filter by default borrows the parameter of the closure
    let check_filter = a.filter(|num| num == &1); //here we only need a number thats equal to 1

    // here if "a" is already a data, then nothing will happen else we return "Some(5)"
    // "or_else" is used in a situation where you have no data but you want it to be set to something
    // accepts a closure that takes no argument
    let a_or_else = a.or_else(|| Some(5));

    // also accepts a closure that takes no argument,
    // we can return a default value(0 in this case which gets returned if a was None).
    // "unwrap_or_else" takes away the optional ability of a data and returns it as non-optional data
    let check_unwrapped = a.unwrap_or_else(|| 0); //this return 4 not Some(4)
}
