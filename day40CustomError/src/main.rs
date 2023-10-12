// Day 40 of Rust, custom errors allows you to be specific about errors that might occur within your
// program.
// Reasons to use custom errors
// 1. functions may fail in more than one way and its important know the reason
// Uisng Error enumeration allows errors to be easily defined and we can "match" on the
// enums later to handle specific error conditions

// Error Requirements
// Implement the "Debug trait": displays what happens in the context of developemt and debuging
// Implement the "Display trait": displays error info in user contexts
// implement the "Error trait": so we can get interoperability with other errors

/////////////
// Manual Error implementation
//////////
#[derive(Debug)]
enum LockError {
    MechanicalError(i32),
    NetworkError,
    NotAuthorized,
}

use std::error::Error;
impl Error for LockError {} //using "{}" indicates we want default values of Error trait.

use std::fmt;
impl fmt::Display for LockError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            // write macro is just like printlin macro
            //write macro is what the println uses behind the scene to format everything
            // with the "write macro", "f which is the formatter" must be provided
            Self::MechanicalError(code) => write!(f, "Mechanical Error: {}", code),
            Self::NetworkError => write!(f, "Network Error"),
            Self::NotAuthorized => write!(f, "Not Authorized"),
        }
    }
}

///////////////
// Implementaion Using Crate: The "thiserror" Crate
///////////////

// first add the "thiserror" crate to the "cargo.toml" under dependencies
///
use thiserror::Error;

#[derive(Debug, Error)]
enum LockError {
    #[error("Mechanical Error: {0}")] // these annotation authomatically impl display traits
    MechanicalError(i32),
    #[error("Network Error")] //annotation with the error message to be displayed
    NetworkError,
    #[error("Not Authorized")]
    NotAuthorized,
}

//////////////
/// Usage
/////////////

fn locl_door() -> Result<(), LockError> {
    // ---some code ---
    Err(LockError::NetworkError)
}

////////////////////////
/// Error Conversion
/////////////////////////
use thiserror::Error;
#[derive(Debug, Error)]
enum NetworkError {
    #[error("Connection timed out")]
    TimeOut,
    #[error("Unreachable")]
    Unreachable,
}

enum LockError {
    #[error("Mechanical Error: {0}")]
    MechanicalError(i32, i32),
    #[error("Network Error")]
    Network(#[from] NetworkError), // the magic happens here
    #[error("Not Authorized")]
    NotAuthorized,
}
fn main() {}
