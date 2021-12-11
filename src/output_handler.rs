use sap_adt_bindings::config::{AdtError, Responses};

pub mod output_handler {}

pub fn handle_output(response: Responses) {
    match response {
        Responses::Program(v) => println!("{}", v),
        Responses::Class(v) => println!("{}", v),
        Responses::FreeStyle(t) => t.display(),
    }
}

pub fn handle_error(error: &AdtError) {
    println!("{}", error);
}
