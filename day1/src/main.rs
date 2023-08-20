// PrintLine(println) macro is used to display values to termianl

fn main() {
    // Sequence of Tokens indicates that we are able to take some external values and include it within a macro, tokens start with "{" and ends with "}"(ie {}).
    // {:?} represent a dubug print(debug prints are prints available to the developer, not mearnt to be seen by end users)

    let time = "time to go";

    println!("The time is {:?} {:?}", time, time);

    // alternatively

    println!("The time is {time:?} {time:?}");

    // {} without ":?" means we want to output something for the end users eg

    println!("It's {time}");
}
