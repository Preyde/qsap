pub mod class_config;
pub mod freestyle_config;
pub mod program_config;

use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};

use crate::data::abap_table::ABAPTable;
pub use crate::net::SAPClient;
use async_trait::async_trait;

pub trait LockHandle {
    fn get_lock_handle_path(&self) -> String;
    fn get_unlock_path(&self) -> Option<String>;
}

pub enum Responses {
    FreeStyle(ABAPTable),
    Program(String),
    Class(String),
}

pub trait Config: std::fmt::Debug {
    fn get_path(&self) -> String;
    fn get_body(&self) -> String;
}
#[derive(Debug)]
pub struct AdtError {
    details: String,
}

impl AdtError {
    fn new(msg: &str) -> Self {
        AdtError {
            details: msg.to_string(),
        }
    }
}

impl Display for AdtError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for AdtError {
    fn description(&self) -> &str {
        &self.details
    }
}

#[async_trait]
pub trait Sendable {
    async fn send_with(&mut self, client: &mut SAPClient) -> Result<(), AdtError>;
    fn get_response(&self) -> Option<Responses>;
}

pub trait SendableConfig: Sendable + Config {}

pub trait AdtResponse {
    fn get_data(self) -> Responses;
}
