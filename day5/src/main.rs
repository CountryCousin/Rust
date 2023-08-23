// Day 5 of Rust, struct(structure) is a data type that contains multiple
// pieces of data, Each piece of data wihtin a struct is call a "field".
// data within a struct can be grouped together as they travel together
// through different parts of the code.

// struct ShippingBox {
//     depth: i32,
//     width: i32,
//     height: i32,
// }

// fn main() {
//     // instanciating the struct
//     let my_box = ShippingBox {
//         depth: 3,
//         width: 2,
//         height: 5,
//     };
//     let tall = my_box.height;
//     println!("the box is {:?} units tall", tall);
// }

struct GroceryItem {
    stock: i32,
    price: f64,
}

fn main() {
    let cereal = GroceryItem {
        stock: 10,
        price: 3.56,
    };

    println!(" stock: {:?}", cereal.stock);
    println!(" price: {:?} ", cereal.price);
}
