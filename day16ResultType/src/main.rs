// Day 16 of Rust, I looked up the Result Data type,
// Result is a type that contans one of two types of data("sucessful" or "Erro" data).
// its useful in scenarios where an action needs to be taken, but has a possiblity of failure

// Definition of Result

// similar to option, Result is an Enum with two variants
// enum Result<T, E> {
//     Ok(T),  // "Ok" represent the operation was succsessfully completed
//     Err(E), // "Err" means the operation failed
// }

// let SoundData = "yell".to_owned();

// fn get_sound(name: &str) -> Result<SoundData, String> {
//     if name == "alert" {
//         Ok(SoundData::new("alert"))
//     } else {
//         Err("unable to find sound data".to_owned())
//     }
// }
// fn main() {
//     let sound = get_sound("alert");
//     match sound {
//         Ok(_) => println!("sound data located"),
//         Err(e) => println!("error: {:?}", e),
//     }
// }

#[derive(Debug)]
enum MenuSelect {
    MainMenu,
    Start,
    Quit,
}

// function that selects choice
fn get_choice(input: &str) -> Result<MenuSelect, String> {
    match input {
        "mainmenu" => Ok(MenuSelect::MainMenu),
        "start" => Ok(MenuSelect::Start),
        "quit" => Ok(MenuSelect::Quit),
        _ => Err("Menu choice not found at the moment".to_owned()),
    }
}

//function that prints choice
fn print_choice(choice: &MenuSelect) {
    println!("choice : {:?} ", choice);
}

fn main() {
    let choice = get_choice("mainmen");
    match choice {
        Ok(main_selection) => print_choice(&main_selection),
        Err(e) => println!("error = {:?}", e),
    }
    // println!("choice : {:?} ", choice);
}
