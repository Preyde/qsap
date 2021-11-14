pub mod class_config;
pub mod freestyle_config;
pub mod program_config;

use std::{
    error::Error,
    fmt::{self, Display, Formatter},
    future::Future,
};

use crate::data::abap_table::ABAPTable;
pub use crate::net::SAPClient;
use async_trait::async_trait;
use reqwest::Response;

use self::{
    class_config::ClassResponse,
    freestyle_config::{FreeStyleConfig, FreeStyleResponse},
    program_config::ProgramResponse,
};

pub trait LockHandle {
    fn get_lock_handle_path(&self) -> String;
}
// #[async_trait]
// pub trait SendWith {
//     async fn send_with(&self, client: &mut SAPClient) -> Response;
// }
// pub fn xxx(){
//     FreeStyleConfig::new("", 1).send_with(client)
// }
pub enum Responses {
    FreeStyle(ABAPTable),
    Program(String),
    Class(String),
}

pub trait Config {
    fn get_path(&self) -> String;
    fn get_body(&self) -> String;
    // async fn send_with<T, E>(&mut self, client: &mut SAPClient) -> Result<T, E>
    // where
    //     T: AdtResponse<Responses>,
    //     E: AdtError;
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
// pub struct XXX {}
// #[async_trait]
// impl Sendable for XXX {
//     async fn send_with(&mut self, client: &mut SAPClient) -> Result<Responses::Program, Error> {
//         Ok(Responses::Program(String::from("")))
//     }
// }
// async fn testxx(xx: &mut XXX) {
//     let x = xx.send_with(&mut SAPClient::new("")).await.unwrap();
// }
pub trait SendableConfig: Sendable + Config {}
// #[async_trait]
// pub trait Sendable<T, E>
// where
//     T: AdtResponse,
//     E: AdtError,
// {
//     async fn send_with<T>(&mut self, client: &mut SAPClient) -> Result<T, E>;
// }
pub trait AdtResponse {
    fn get_data(self) -> Responses;
}

// trait SendableConfig:
//     Config + Sendable<(dyn AdtResponse<Responses> + 'static), (dyn AdtError + 'static)>
// {
// }
// pub trait AdtError {}

// struct Config {
//     body: String,
//     path: String
// }
