mod request;
mod sap_client;

pub use request::behavior;
pub use request::object;
pub use request::SendWith;
pub use request::{AdtError, Response, Responses};
// pub use request::{class::Class, program::Program, table::Table};
pub use sap_client::{Destination, SAPClient, Session};
