pub mod class_config;
pub mod freestyle_config;
pub mod program_config;
pub mod strategy;

use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};

use crate::data::abap_table::ABAPTable;
pub use crate::net::SAPClient;
use async_trait::async_trait;

pub trait LockHandles {
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

pub trait Create {
    fn create(&mut self) -> Box<dyn Request>;
}
pub trait CopyTo {
    fn copy_to(&mut self, target_name: &str) -> Box<dyn Request>;
}
pub trait Delete {
    fn delete(&'static self) -> Box<dyn Request>;
}

pub trait LockHandle {
    fn get_lock_path(&self) -> String;
    fn get_unlock_path(&self) -> Option<String>;
    fn set_lock_handle(&mut self, lock_handle: &str);
}
pub trait Lock {
    fn lock();
}

pub trait Unlock {
    fn unlock();
}

impl<T> Unlock for T
where
    T: Lock + LockHandle,
{
    fn unlock() {}
}

#[async_trait]
pub trait SendWith {
    async fn send_with(&mut self, client: &mut SAPClient) -> Result<(), AdtError>;
}

pub trait Source {
    fn source(&self) -> Box<dyn Request>;
    fn update_source(&'static mut self, source: &str) -> Box<dyn Request>;
    fn get_source(&self) -> String;
}

#[async_trait]
pub trait Sendable {
    async fn send_with(&mut self, client: &mut SAPClient) -> Result<(), AdtError>;
    fn get_response(&self) -> Option<Responses>;
}
// macro_rules! Config {
//     ($t:ident) => {
//         impl Config for $t {
//             fn get_body(&self) -> String {
//                 self.body
//             }
//             fn get_path(&self) -> String {
//                 self.path
//             }
//         }
//     };
// }
// #[derive(Config)]
#[async_trait]
pub trait Request: Sync + Send {
    /// Get HTTP Method
    fn get_method(&self) -> reqwest::Method;
    fn get_path(&self) -> String;
    fn get_body(&self) -> String;
    async fn send_with(&mut self, client: &mut SAPClient) -> Result<(), AdtError>;
}

impl<T> AsReq for T
where
    T: Request,
{
    fn as_req(&self) -> Box<&dyn Request> {
        Box::new(self as &dyn Request)
    }
}
pub trait AsReq {
    fn as_req(&self) -> Box<&dyn Request>;
}

pub trait SendableConfig: Sendable + Config {}

pub trait AdtResponse {
    fn get_data(self) -> Responses;
}
