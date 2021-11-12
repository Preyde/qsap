pub mod class_config;
pub mod freestyle_config;
pub mod program_config;

use std::fmt::Error;

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
    Program(()),
    Class(()),
}

#[async_trait]
pub trait Config {
    fn get_path(&self) -> String;
    fn get_body(&self) -> String;
    // async fn send_with<T, E>(&mut self, client: &mut SAPClient) -> Result<T, E>
    // where
    //     T: AdtResponse<Responses>,
    //     E: AdtError;
}
#[async_trait]
pub trait Sendable<T, E>
where
    T: AdtResponse,
    E: AdtError,
{
    async fn send_with(&mut self, client: &mut SAPClient) -> Result<T, E>;
}
pub trait AdtResponse {
    fn get_data(self) -> Responses;
}

// trait SendableConfig:
//     Config + Sendable<(dyn AdtResponse<Responses> + 'static), (dyn AdtError + 'static)>
// {
// }
pub trait AdtError {}

// struct Config {
//     body: String,
//     path: String
// }
