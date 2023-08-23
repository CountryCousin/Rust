// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

// My first implementation
// enum Falours {
//     Vanilla,
//     Orange,
//     Chocolate,
// }

// struct Flavor {
//     vanilla: f64,
//     orange: f64,
//     chocolate: f64,
// }
// fn print_flavor_ounces(flavor: Falours, flavo: Flavor) {
//     let struct_flavor = Flavor {
//         vanilla: 2.3,
//         orange: 1.2,
//         chocolate: 3.1,
//     };
//     match flavor {
//         Falours::Vanilla => println!(" {} ", struct_flavor.vanilla),
//         Falours::Orange => println!("{}", struct_flavor.orange),
//         Falours::Chocolate => println!("{}", struct_flavor.chocolate),
//     }
// }
// fn main() {
//     let struct_flavor = Flavor {
//         vanilla: 2.3,
//         orange: 1.2,
//         chocolate: 3.1,
//     };
//     print_flavor_ounces(Falours::Chocolate, struct_flavor);
// }

// more concise implementation
// enum Flavors {
//     Vanilla,
//     Orange,
//     Chocolate,
// }

// struct Flavor {
//     vanilla: f64,
//     orange: f64,
//     chocolate: f64,
// }

// fn print_flavor_ounces(flavor: Flavors, flavo: Flavor) {
//     match flavor {
//         Flavors::Vanilla => println!("Vanilla: {}", flavo.vanilla),
//         Flavors::Orange => println!("Orange: {}", flavo.orange),
//         Flavors::Chocolate => println!("Chocolate: {}", flavo.chocolate),
//     }
// }

// fn main() {
//     let struct_flavor = Flavor {
//         vanilla: 2.3,
//         orange: 1.2,
//         chocolate: 3.1,
//     };
//     print_flavor_ounces(Flavors::Chocolate, struct_flavor);
// }

// what He wanted:

enum Flavor {
    Sparkling,
    Sweet,
    Fruity,
}

struct Drink {
    flavor: Flavor,
    fluid_oz: f64,
}
fn print_drink(drink: Drink) {
    match drink.flavor {
        Flavor::Sparkling => println!("flavor: sparkling"),
        Flavor::Sweet => println!("flavor: sweet"),
        Flavor::Fruity => println!("flavour: fruity"),
    }

    println!("oz: {:?},", drink.fluid_oz);
}

fn main() {
    let sweet = Drink {
        flavor: Flavor::Sweet,
        fluid_oz: 2.3,
    };

    let fruity = Drink {
        flavor: Flavor::Fruity,
        fluid_oz: 1.9,
    };

    print_drink(sweet);
    print_drink(fruity);
}
