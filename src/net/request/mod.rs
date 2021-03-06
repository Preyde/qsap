pub mod behavior;
pub mod object;
pub mod strategy;

// pub mod table;
use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};

use crate::data::abap_table::{AbapTable, SoapResponse};
pub use crate::net::SAPClient;
use async_trait::async_trait;
use reqwest::StatusCode;

pub trait LockHandles {
    fn get_lock_handle_path(&self) -> String;
    fn get_unlock_path(&self) -> Option<String>;
}

#[derive(PartialEq, Eq, Debug)]
pub enum Responses {
    Default(String),
    Table(AbapTable),
    Detail(SoapResponse), // Class(String),
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

/// Used to create a Repository Object. First Type Parameter is the Response Type and the second the data contained in the response
// pub trait Create<T, U>
// where
//     T: Response<U> + TryFromAsync<reqwest::Response>,
// {
//     // fn create(&self) -> DefaultStrategy<T, U>;
//     fn create(&self) -> Box<dyn SendWith<T>>;
// }
// pub trait Create {
//     fn create(&self) -> Box<dyn SendWith>;
// }
// pub trait CopyTo // where
// //     T: Response + TryFromAsync<reqwest::Response>,
// {
//     // type Response: Response;
//     fn copy_to(&self, target_name: &str) -> Box<dyn SendWith>;
// }
// pub trait Delete // where
// //     T: Response + TryFromAsync<reqwest::Response>,
// {
//     // type Response: Response;
//     fn delete(&mut self) -> Box<dyn SendWith>;
// }
// pub trait CopyToSys // where
// //     T: Response + TryFromAsync<reqwest::Response>,
// {
//     // type Response: Response;
//     // fn copy_to_sys<'a>(&'a self, dest: &Destination) -> Box<dyn SendWith<T> + 'a>;
//     fn copy_to_sys<'a>(&'a self, dest: &Destination) -> Box<dyn SendWith + 'a>;
// }

pub trait LockHandle {
    fn get_lock_path(&self) -> String;
    fn get_unlock_path(&self) -> Option<String>;
    fn set_lock_handle(&mut self, lock_handle: &str);
}
#[derive(Debug)]
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
pub trait SendWith: Sync + Send {
    async fn send_with(&mut self, client: &mut SAPClient) -> Result<Box<dyn Response>, AdtError>;
}

use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename = "DATA")]
pub struct LockHandleData {
    pub LOCK_HANDLE: String,
    pub CORRNR: String,
    pub CORRUSER: String,
    pub CORRTEXT: String,
}

#[derive(Debug, Deserialize)]
pub struct LockHandleValues {
    pub DATA: LockHandleData,
}
#[derive(Debug, Deserialize)]
#[serde(rename = "asx:abap")]
pub struct LockHandleResponse {
    // #[serde(rename = "asx:values")]
    pub values: LockHandleValues,
}

#[async_trait]
pub trait Request: Sync + Send {
    // fn new() -> Box<dyn Request>;
    fn get_method(&self) -> reqwest::Method;
    fn get_path(&self) -> String;
    fn get_body(&self) -> String;
    // async fn send_with(&mut self, client: &mut SAPClient) -> Result<(), AdtError>;
}

pub trait Response: Sync + Send // where
// Self: TryFromAsync<reqwest::Response>,
{
    // type Result;
    // fn new(body: &str, status: StatusCode) -> Box<dyn Response>;
    fn get_value(&self) -> Responses;
    fn get_text(&self) -> String;
    fn get_status(&self) -> StatusCode;
}

pub struct DefaultResponse {
    body: String,
    status: StatusCode,
}
#[async_trait]
impl TryFromAsync<reqwest::Response> for DefaultResponse {
    // type Error = AdtError;

    async fn try_from_async(res: reqwest::Response) -> Result<Box<dyn Response>, AdtError> {
        let status = res.status();
        if let Ok(text) = res.text().await {
            // println!("{}", text);
            Ok(Box::new(DefaultResponse { body: text, status }))
        } else {
            Err(AdtError {
                details: String::from("xxx"),
            })
        }
    }
}
impl Response for DefaultResponse {
    // type Result = String;
    fn get_status(&self) -> StatusCode {
        self.status
    }
    fn get_text(&self) -> String {
        self.body.clone()
    }
    fn get_value(&self) -> Responses {
        Responses::Default(self.body.clone())
    }
}
#[async_trait]
pub trait TryFromAsync<T> // where
// Self: Sized,
{
    // type Error;
    async fn try_from_async(_: T) -> Result<Box<dyn Response>, AdtError>;
}
// #[async_trait]
// impl<T> TryFromAsync<reqwest::Response> for T
// where
//     T: Response,
// {
//     // type Error = AdtError;

//     async fn try_from_async(res: reqwest::Response) -> Result<Box<dyn Response>, AdtError> {
//         let status = res.status();
//         if let Ok(text) = res.text().await {
//             // println!("{}", text);
//             Ok(Box::new(DefaultResponse { body: text, status }))
//         } else {
//             Err(AdtError {
//                 details: String::from("xxx"),
//             })
//         }
//     }
// }

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
// pub trait SendableConfig: Sendable + Config {}

pub trait AdtResponse {
    fn get_data(self) -> Responses;
}
