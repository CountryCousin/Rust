// // Day 31 of Rust, Traits is a way to specify that some functionality exists.
// // They are used to stardardize functionality across multiple different types.
// // stardardization permits functions to operate on multiple different types

// trait Noise {
//     fn make_noise(&self);
//     fn keep_quiet(&self);
//     fn all_quiet(&self) -> bool;
// }

// struct Person;
// impl Noise for Person {
//     fn make_noise(&self) {
//         println!("Say Loud Hello");
//     }
//     fn keep_quiet(&self) {
//         println!("All is quiet");
//     }
//     fn all_quiet(&self) -> bool {
//         true
//     }
// }

// fn sound(noisy: impl Noise) {
//     noisy.make_noise();
//     noisy.keep_quiet();
//     noisy.all_quiet();
// }
// fn main() {
//     sound(Person {})
// }

// My trial
// trait TryTraits {
//     fn raining(&self);
//     fn dry_season(&self);
// }

// struct Weather;
// impl TryTraits for Weather {
//     fn raining(&self) {
//         println!("I think its raining tho");
//     }

//     fn dry_season(&self) {
//         println!("Well its dry season");
//     }
// }

// fn prints_raining(rain: impl TryTraits) {
//     rain.raining();
//     rain.dry_season()
// }

// fn main() {
//     prints_raining(Weather {});
//     // prints_raining(Weather {});
// }

trait Perimeter {
    fn peri_square(&self, lenght: i32);
    fn peri_traingle(&self, side_a: i32, side_b: i32, side_c: i32);
}

struct Square;
impl Perimeter for Square {
    fn peri_square(&self, lenght: i32) {
        let result = 4 * lenght;
        println!("The perimeter of  the square is: {:?}", result);
    }
    fn peri_traingle(&self, side_a: i32, side_b: i32, side_c: i32) {
        let peri_tri = side_a + side_b + side_c;
        println!(
            "The peri of the tri with sides {:?}, {:?}, {:?} is {:?}",
            side_a, side_b, side_c, peri_tri
        );
    }
}

fn display_peri(perimeter: impl Perimeter) {
    perimeter.peri_square(4);
    perimeter.peri_traingle(2, 3, 5)
}

fn main() {
    display_peri(Square {})
}
