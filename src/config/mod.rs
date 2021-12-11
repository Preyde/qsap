pub mod class_config;
pub mod freestyle_config;
pub mod program_config;
pub mod strategy;

use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};

pub use crate::net::SAPClient;
use crate::{data::abap_table::ABAPTable, net::Destination};
use async_trait::async_trait;

use self::strategy::DefaultStrategy;

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
    fn create(&self) -> DefaultStrategy;
}
pub trait CopyTo {
    fn copy_to(&mut self, target_name: &str) -> Box<dyn SendWith>;
}
pub trait Delete {
    fn delete(&mut self) -> Box<dyn SendWith>;
}
pub trait CopyToSys {
    fn copy_to_sys<'a>(&'a self, dest: &Destination) -> Box<dyn SendWith + 'a>;
}

pub trait LockHandle {
    fn get_lock_path(&self) -> String;
    fn get_unlock_path(&self) -> Option<String>;
    fn set_lock_handle(&mut self, lock_handle: &str);
}

pub struct LockObject {
    // lock_path: String,
    // unlock_path: String,
    lock_handle: Option<String>,
    path: String,
}
impl LockHandle for LockObject {
    fn get_lock_path(&self) -> String {
        format!("{}?_action=LOCK&accessMode=MODIFY", self.path)
    }
    fn get_unlock_path(&self) -> Option<String> {
        Some(format!(
            "{}?_action=UNLOCK&lockHandle={}",
            self.path,
            self.lock_handle.as_ref()?
        ))
    }
    fn set_lock_handle(&mut self, lock_handle: &str) {
        self.lock_handle = Some(lock_handle.to_string())
    }
}
impl LockObject {
    fn new(path: &str) -> Self {
        LockObject {
            path: path.to_string(),
            lock_handle: None,
        }
    }
    fn get_path(&self) -> Option<String> {
        Some(format!(
            "{}?lockHandle={}",
            self.path,
            self.lock_handle.as_ref()?
        ))
    }
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
pub trait SendWith: Send + Sync {
    async fn send_with(&mut self, client: &mut SAPClient) -> Result<(), AdtError>;
}

pub trait Source {
    fn source(&self) -> Box<dyn Request>;
    fn update_source(&self, source: &str) -> Box<dyn SendWith>;
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
    // async fn send_with(&mut self, client: &mut SAPClient) -> Result<(), AdtError>;
}

pub trait AsReq {
    fn as_req(&self) -> Box<&dyn Request>;
    fn as_req_mut(&mut self) -> Box<&mut dyn Request>
    where
        Self: Sized + Request,
    {
        Box::new(self as &mut dyn Request)
    }
}
impl<T> AsReq for T
where
    T: Request,
{
    fn as_req(&self) -> Box<&dyn Request> {
        Box::new(self as &dyn Request)
    }
}
pub trait SendableConfig: Sendable + Config {}

pub trait AdtResponse {
    fn get_data(self) -> Responses;
}
