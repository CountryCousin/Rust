// add image to the cargo.toml file using the command: cargo add image

fn get_nth_arg(n: usize) -> String {
    std::env::args().nth(n).unwrap()
}
#[derive(Debug)]
pub struct Args {
    pub image_1: String,
    pub image_2: String,
    pub output: String
}

impl Args {
    pub fn new()-> Self{
        let image_1 = get_nth_arg(1);
        let image_2 = get_nth_arg(2);
        let output = get_nth_arg(3);

        Args {
            image_1,
            image_2,
            output
        }
    }
}