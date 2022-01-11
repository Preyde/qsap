// use crate::config::Sendable;
use reqwest::{header::HeaderMap, Client, Response};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::request::{LockHandle, Request};
#[derive(Debug)]
pub struct Session {
    pub csrf_token: String,
    pub session_cookie: String,
    pub session_type: String,
}
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Destination {
    pub sys_id: String,
    pub host: String,
    pub port: u16,
    pub uname: String,
    pub passwd: String,
    pub mandt: String,
    pub lang: String,
}
pub struct SAPClient {
    client: Client,
    headers: Option<HashMap<String, Option<String>>>,
    session: Option<Session>,
    host: String,
    content_type: String,

    stateful: bool,
    dest: Destination,

    cookies: HashMap<String, String>,
}

impl SAPClient {
    pub fn new(dest: &Destination) -> SAPClient {
        SAPClient {
            client: reqwest::Client::builder()
                .cookie_store(true)
                .build()
                .unwrap(),
            dest: dest.clone(),
            headers: None,
            session: None,
            cookies: HashMap::new(),
            content_type: "text/html".to_string(),
            stateful: false, // app_config: app_conf,
            host: format!("{}:{}", dest.host, dest.port),
        }
    }
    pub async fn test_destination(&self) -> Result<(), reqwest::Error> {
        let url = format!("{0}{1}", &self.host, "/sap/bc/adt/compatibility/graph");
        // println!("{}", self.append_mandt_to_url(&url));
        self.client
            .get(self.append_mandt_to_url(&url))
            .send()
            .await?;
        Ok(())
    }
    pub fn clear_session(&mut self) {
        self.session = None;
    }
    pub fn set_stateful(&mut self, stateful: bool) {
        self.stateful = stateful;
    }
    pub fn set_content_type(&mut self, content_type: &str) {
        self.content_type = content_type.to_string();
    }
    pub fn set_destination(&mut self, dest: &Destination) {
        self.dest = dest.clone();
        self.host = format!("{}:{}", dest.host, dest.port);
    }
    pub fn from_session(dest: &Destination, session: Session) -> Self {
        SAPClient {
            client: reqwest::Client::builder()
                .cookie_store(true)
                .build()
                .unwrap(),
            cookies: HashMap::new(),
            session: Some(session),
            dest: dest.clone(),
            headers: None,
            content_type: String::from("text/html"),
            stateful: false,
            host: format!("{}:{}", dest.host, dest.port),
        }
    }
    fn append_mandt_to_url(&self, url: &str) -> String {
        if url.contains("?") {
            format!("{}&sap-client={}", url, &self.dest.mandt)
        } else {
            format!("{}?sap-client={}", url, &self.dest.mandt)
        }
    }
    pub fn get_session(&self) -> Option<Session> {
        let session = self.session.as_ref()?;
        Some(Session {
            csrf_token: String::from(session.csrf_token.clone()),
            session_cookie: String::from(session.session_cookie.clone()),
            session_type: String::from(session.session_cookie.clone()),
        })
    }

    async fn fetch_csrf_token(&mut self) {
        let result = &self
            .client
            .get(format!(
                "{}{}",
                &self.host,
                format!(
                    "/sap/bc/adt/compatibility/graph?sap-client={}",
                    self.dest.mandt
                )
            ))
            .basic_auth(&self.dest.uname, Some(&self.dest.passwd))
            .header("x-csrf-token", "Fetch")
            .send()
            .await;

        let res = match result {
            Ok(res) => self.set_headers_from_headermap(res.headers()),
            Err(e) => {
                if e.is_connect() {
                    println!("Couldn't connect to Server. Maybe you forgot turning your VPN on?");
                    std::process::exit(0);
                }
            }
        };
    }
    fn set_headers_from_headermap(&mut self, headers: &HeaderMap) {
        let mut hashmap: HashMap<String, Option<String>> = HashMap::new();
        headers.iter().for_each(|(name, value)| {
            if name.to_string() == "set-cookie" {
                let s = String::from(value.to_str().unwrap());
                let v: Vec<&str> = s.split("=").collect();
                let cookie_name = v.get(0).unwrap();
                let cookie_value_string = String::from(v.get(1).unwrap().clone());
                let cookie_value: &str = cookie_value_string
                    .split(";")
                    .collect::<Vec<&str>>()
                    .get(0)
                    .unwrap();
                // println!("{}", cookie_name.clone());
                // println!("{}", cookie_value.clone());
                self.cookies.insert(
                    String::from(cookie_name.clone()),
                    String::from(cookie_value.clone()),
                );
            }

            hashmap.insert(name.to_string(), Some(value.to_str().unwrap().to_string()));
        });
        self.headers = Some(hashmap);
    }

    pub async fn request<'a>(&mut self, config: Box<&'a dyn Request>) -> Response {
        if self.session.is_none() {
            self.fetch_csrf_token().await;
            // println!("{:?}", self.headers.as_ref().unwrap());
            self.session = Some(Session {
                csrf_token: self
                    .headers
                    .as_ref()
                    .unwrap()
                    .get("x-csrf-token")
                    .unwrap()
                    .clone()
                    .unwrap(),
                session_type: if self.stateful {
                    "stateful".to_string()
                } else {
                    "stateless".to_string()
                },
                session_cookie: self
                    .cookies
                    .get(&self.get_session_cookie_name())
                    .unwrap()
                    .clone(), //  session_cookie:
            });
        }
        // println!("{:?}", config.get_body());
        let url = format!("{0}{1}", &self.host, &config.get_path());
        // println!("{}", self.append_mandt_to_url(&url));
        self.client
            .request(config.get_method(), self.append_mandt_to_url(&url))
            .basic_auth(&self.dest.uname, Some(&self.dest.passwd))
            .header(
                "x-csrf-token",
                self.session.as_ref().unwrap().csrf_token.clone(),
            )
            .header(
                "Cookie",
                format!(
                    "{}={}",
                    &self.get_session_cookie_name(),
                    self.session.as_ref().unwrap().session_cookie
                ),
            )
            .header(
                "X-sap-adt-sessiontype",
                &self.session.as_ref().unwrap().session_type,
            )
            .header("Content-Type", &self.content_type)
            .body(config.get_body())
            .send()
            .await
            .unwrap()
    }
    fn get_session_cookie_name(&self) -> String {
        format!("SAP_SESSIONID_{}_{}", self.dest.sys_id, self.dest.mandt)
    }
    pub async fn lock<T>(&mut self, lock_handle: &T) -> Response
    where
        T: LockHandle,
    {
        if self.session.is_none() {
            self.fetch_csrf_token().await;
            // println!("{:?}", self.headers.as_ref().unwrap());
            self.session = Some(Session {
                csrf_token: self
                    .headers
                    .as_ref()
                    .unwrap()
                    .get("x-csrf-token")
                    .unwrap()
                    .clone()
                    .unwrap(),
                session_type: if self.stateful {
                    "stateful".to_string()
                } else {
                    "stateless".to_string()
                },
                session_cookie: self
                    .cookies
                    .get(&self.get_session_cookie_name())
                    .unwrap()
                    .clone(), //  session_cookie:
            });
        }

        let url = format!("{0}{1}", &self.host, lock_handle.get_lock_path());
        // println!("{}", url);
        self.client
            .post(self.append_mandt_to_url(&url))
            .basic_auth(&self.dest.uname, Some(&self.dest.passwd))
            .header(
                "x-csrf-token",
                self.session.as_ref().unwrap().csrf_token.clone(),
            )
            .header(
                "Cookie",
                format!(
                    "{}={}",
                    &self.get_session_cookie_name(),
                    self.session.as_ref().unwrap().session_cookie
                ),
            )
            .header(
                "X-sap-adt-sessiontype",
                &self.session.as_ref().unwrap().session_type,
            )
            .send()
            .await
            .unwrap()
    }
    pub async fn unlock<T>(&mut self, lock_handle: &T) -> Response
    where
        T: LockHandle,
    {
        if self.session.is_none() {
            self.fetch_csrf_token().await;
            // println!("{:?}", self.headers.as_ref().unwrap());
            self.session = Some(Session {
                csrf_token: self
                    .headers
                    .as_ref()
                    .unwrap()
                    .get("x-csrf-token")
                    .unwrap()
                    .clone()
                    .unwrap(),
                session_type: if self.stateful {
                    "stateful".to_string()
                } else {
                    "stateless".to_string()
                },
                session_cookie: self
                    .cookies
                    .get(&self.get_session_cookie_name())
                    .unwrap()
                    .clone(), //  session_cookie:
            });
        }

        let url = format!("{0}{1}", &self.host, lock_handle.get_unlock_path().unwrap());

        self.client
            .post(self.append_mandt_to_url(&url))
            .basic_auth(&self.dest.uname, Some(&self.dest.passwd))
            .header(
                "x-csrf-token",
                self.session.as_ref().unwrap().csrf_token.clone(),
            )
            .header(
                "Cookie",
                format!(
                    "{}={}",
                    &self.get_session_cookie_name(),
                    self.session.as_ref().unwrap().session_cookie
                ),
            )
            .header(
                "X-sap-adt-sessiontype",
                &self.session.as_ref().unwrap().session_type,
            )
            .send()
            .await
            .unwrap()
    }
}
