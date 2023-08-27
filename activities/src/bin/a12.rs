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

// struct Box {
//     dimensions: i32,
//     weight: i32,
//     color: Color,
// }
// #[derive(Debug)]
// enum Color {
//     Red,
// }

// impl Box {
//     fn new_box() -> Self {
//         Self {
//             dimensions: 3,
//             weight: 1,
//             color: Color::Red,
//         }
//     }

//     fn print_Box(self: &Box) {
//         println!("{:?}", self.dimensions);
//         println!("{:?}", self.weight);
//         println!("{:?}", self.color);
//     }
// }
// fn main() {
//     let box1 = Box {
//         dimensions: 4,
//         weight: 89,

//         color: Color::Red,
//     };

//     let box2 = Box::new_box();
//     box1.print_Box();
//     box2.print_Box()
// }

enum Color {
    Brown,
    Red,
}

impl Color {
    fn print(&self) {
        match self {
            Color::Brown => println!("its Brown"),
            Color::Red => println!("its Red"),
        }
    }
}

struct Dimensions {
    width: f64,
    height: f64,
    depth: f64,
}

impl Dimensions {
    fn print(&self) {
        println!("width:{:?}", self.width);
        println!("height:{:?}", self.height);
        println!("depth:{:?}", self.depth);
    }
}

struct ShippingBox {
    color: Color,
    weight: f64,
    dimensions: Dimensions,
}

impl ShippingBox {
    // 'new' function use to create new structure or enumeration
    fn new(weight: f64, color: Color, dimensions: Dimensions) -> Self {
        Self {
            weight,
            color,
            dimensions,
        }
    }

    fn print(&self) {
        self.color.print();
        self.dimensions.print();
        println!("weight: {:?}", self.weight);
    }
}

fn main() {
    let dimension = Dimensions {
        width: 1.0,
        height: 2.0,
        depth: 2.5,
    };

    let first_box = ShippingBox::new(9.0, Color::Brown, dimension);
    first_box.print();
}
