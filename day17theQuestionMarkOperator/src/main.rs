// Day 17 of Rust, instead of "matching"
// the result, I learnt an alternative method of accessing data from the "MenuSelect" enum . Using the question mark operator("?") will automatically handle scenarios
// for both the "Ok and Err variants" from the Result type. So far I think the Result type is a bit tricky, lol
//
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

// function that uses question mark operator
// "() is called the unit type", it means return nothing from a function

fn pick_choice(input: &str) -> Result<(), String> {
    let choice: MenuSelect = get_choice(input)?; //"?" automatically performs a match operations and returns the correct variant
    print_choice(&choice);
    Ok(())
}

fn main() {
    pick_choice("start");
}
