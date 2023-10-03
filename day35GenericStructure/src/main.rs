//  Day 35 of Rust, Generic structures allows you to store any type of data within a structure
// Trait bounds(generic constraints) restrict the type of data the structure can utilize,
// its useful when creating your own data collection(ie custom data collection)

// Synthax Of Generic Structures:

// first sythax
struct Name<T: Trait1 + Trait2, U: Trait3> {
    field1: T,
    field2: U,
}

// second synthax
struct Name<T, U>
where
    T: Trait1 + Trait2,
    U: Trait3,
{
    field1: T,
    field2: U,
}

// Example of generic structure:

// first define a trait
trait Seat {
    fn show(&self);
}

// then the define a struct that uses the trait as its generic parameter
struct Ticket<T: Seat> {
    location: T,
}

// Defining the types of "Seating" available
#[derive(Clone, Copy)]
enum ConcertSeat {
    FrontRow,
    MidSection(u32),
    Back(u32),
}

#[derive(Clone, Copy)]
enum AirlineSeat {
    BusinessClass,
    Economy,
    FirstClass,
}

// specific implementation for "AirlineSeat"
// fn ticket_info(ticket: Ticket<AirlineSeat>) {
//     ticket.location.show();
// }

// General implementation
fn ticket_info(ticket: Ticket<T>) {
    ticket.location.show();
}

let airline = Ticket{ location: AirlineSeat::Economy};
let airline = Ticket{ location: ConcertSeat::Back(3) };
ticket_info(airline);

fn main() {
    
}
