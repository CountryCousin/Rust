// started Day 9 of Rust with understanding "impl" keyword
// impl keyword allow us to implement a certain functionality on specific enums or structs
// This  enhances the organisation of your code and makes your programs easier to follow.

////////////////////////////////////////////////////////
// Normal way to output the content of a struct
/////////////////////////////////////////////////////////
// struct Temperature {
//     degree_f: f64,
// }

// fn show_temp(temp: Temperature) {
//     println!("{:?} degree F", temp.degree_f);
// }

// fn main() {
//     let hot = Temperature { degree_f: 99.9 };
//     show_temp(hot)
// }

////////////////////////////////////////////////////////////
// implementing "impl" functionality on the "temperature " struct
/////////////////////////////////////////////////////////////
// struct Temperature {
//     degree_f: f64,
// }

// impl Temperature {
//     fn show_temp(temp: Temperature) {
//         println!("{:?} degree F", temp.degree_f);
//     }
// }

// fn main() {
//     let hot = Temperature { degree_f: 99.9 };
//     Temperature::show_temp(hot)
// }
//////////////////////////////////////////
// refactoring the code with the "self" keyword
// self refers to the struct "Temperature" and its already created fields
///////////////////////////////////////////////
// struct Temperature {
//     degree_f: f64,
// }

// impl Temperature {
//     fn show_temp(self: &Temperature) {
//         println!("{:?} degree F", self.degree_f);
//     }
// }

// fn main() {
//     let hot = Temperature { degree_f: 99.9 };
//     // Temperature::show_temp(hot)
//     hot.show_temp();
// }

///////////////////////////////////////////////
/// Uing 'Self': Self is just the struct or enum we are referring to
/// within the implementation block
///////////////////////////////////////////
// we can also use implementation blocks to create specific types of Temperature
// The diffence between "self" and "Self" is "self" indicates we already have a value
// ie temperature(degree_f) created somewhere in the program while "Self" indicates we don't
// something and we want to create it(create new temperature in this instance)
// hence we can think of "Self" as the struct" Temperature".
//

// struct Temperature {
//     degree_f: f64,
// }

// impl Temperature {
//     // creating instance of "Temperature struct" using "Self"
//     fn freezing() -> Self {
//         Self { degree_f: 43.0 }
//     }

// This synthax below could works as well but
// if we change the name of the struct which is "Temperature",
// then we will have to change it all over the program,
// "Self" saves us time and is more efficient
// fn freezing() -> Temperature {
//     Temperature { degree_f: 43.0 }
// }

//     fn show_temp(self: &Temperature) {
//         println!("{:?} degree F", self.degree_f);
//     }
// }

// fn main() {
//     let hot = Temperature { degree_f: 99.9 };
//     // Temperature::show_temp(hot)  // works as well
//     hot.show_temp();

//     // how to call the freezing function:
//     let cold = Temperature::freezing();
//     cold.show_temp()
// }

// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics

// my implementation of excercise a12
struct Box {
    dimensions: Dimensions,
    weight: f64,
    color: Color,
}

impl Box {
    fn new_box(dimensions: Dimensions, weight: f64, color: Color) -> Self {
        Self {
            dimensions,
            weight,
            color,
        }
    }

    fn print_box(self: &Box) {
        self.color.print();
        println!("Weight; {:?}", self.weight);
        self.dimensions.print();
    }
}

struct Dimensions {
    height: f64,
    width: f64,
    length: f64,
}
impl Dimensions {
    fn print(self: &Dimensions) {
        println!("height: {:?}", self.height);
        println!("width: {:?}", self.width);
        println!("length: {:?}", self.length);
    }
}
enum Color {
    Red,
    BrickYello,
}

impl Color {
    fn print(self: &Color) {
        match self {
            Color::Red => println!("its Red"),
            Color::BrickYello => println!("its BrickYello"),
        }

        // alternatively

        // if let Color::Red = self {
        //     println!("its red");
        // } else {
        //     println!("its BrownYellow")
        // }
    }
}

fn main() {
    let dimension1 = Dimensions {
        height: 6.0,
        width: 7.1,
        length: 4.1,
    };

    let box1 = Box::new_box(dimension1, 4.0, Color::Red);
    box1.print_box();
}
