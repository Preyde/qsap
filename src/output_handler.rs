// use sap_bindings::net::{AdtError, Responses};
use sap_bindings::net::{AdtError, Responses};

pub mod output_handler {}

pub fn handle_output(response: Responses) {
    match response {
        Responses::Default(v) => println!("{}", v),
        Responses::Table(v) => println!(""),
    }
    // match response {
    //     // Responses::Program(v) => println!("{}", v),
    //     // Responses::Class(v) => println!("{}", v),
    //     // Responses::FreeStyle(t) => t.display(),
    // }
}

pub fn handle_error(error: &AdtError) {
    println!("{}", error);
}
