// Day 39 of Rust, Lifetimes are a way to specifiy to the compiler how to handle a borrowed data.
// lifetimes are a way to inform the compiler that borrowed data will be vailid at a
// specific point in time. lifetimes are needed for storing borrowed data in structs
// and enums and returning a borrowed data from functions. All data has a lifetime(compiler does the calculations in many cases)
// This would enable us to store borrowed data in structures and return borrowed data from functions

// Review of Ownership

// #[derive(Debug)]
// enum FrozenItem {
//     IceCube,
// }

// #[derive(Debug)]
// struct Freezer {
//     contents: Vec<FrozenItem>,
// }

// fn place_item(freezer: &mut Freezer, item: FrozenItem) {
//     freezer.contents.push(item);
// }

// fn main() {
//     let mut freezer = Freezer { contents: vec![] };
//     let cube = FrozenItem::IceCube;
//     place_item(&mut freezer, cube); //cube is no longer available
// }


// Lifetime Synthax - Struct
// conventional use of lifetime, 'a, 'b, 'car.
// however "'static" is a reserved keyword.
// 'static data stays in memory until the program terminates

Struct Name<'a>{ //"'a": lifetime is a form of generic
    field: &'a DataType,


}

// machine  Part
enum Part{
    Bolt,
    Panel,
}

// Robot arm that would borrow the Parts
struct RobotArm<'a>{
    part: & 'a Part,
}

struct AssemblyLine{
    parts : Vec<Part>,
}

fn main(){
    let line = AssemblyLine{
        parts: vec![Part::Bolt, Part::Panel],
    };

    {
        let arm = RobotArm{
            part: &line.part[0],
        },
     } //lifetime ends here
}