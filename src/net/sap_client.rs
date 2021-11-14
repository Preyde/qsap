// use crate::config::Sendable;
use crate::config::{AdtError, AdtResponse, Config, LockHandle, Responses, SendableConfig};
use reqwest::{header::HeaderMap, Client, Response};
use serde::Deserialize;
use std::{collections::HashMap, fmt::Display};
#[derive(Debug)]
pub struct Session {
    pub csrf_token: String,
    pub session_cookie: String,
    pub session_type: String,
}
#[derive(Debug, Deserialize, Clone)]
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
    // host: String,
    stateful: bool,
    dest: Destination,
    // app_config: AppConfig,
    cookies: HashMap<String, String>,
}

impl SAPClient {
    pub fn new(dest: &Destination) -> SAPClient {
        // let mut app_conf = AppConfig::init();
        SAPClient {
            client: reqwest::Client::builder()
                .cookie_store(true)
                .build()
                .unwrap(),
            dest: dest.clone(),
            headers: None,
            session: None,
            cookies: HashMap::new(),
            stateful: false, // app_config: app_conf,
            host: format!("{}:{}", dest.host, dest.port),
        }
    }

    pub fn set_stateful(&mut self, stateful: bool) {
        self.stateful = stateful;
    }
    pub fn from_session(dest: &Destination, session: Session) -> Self {
        SAPClient {
            // app_config: AppConfig::init(),
            client: reqwest::Client::builder()
                .cookie_store(true)
                .build()
                .unwrap(),
            cookies: HashMap::new(),
            session: Some(session),
            dest: dest.clone(),
            headers: None,
            stateful: false,
            host: format!("{}:{}", dest.host, dest.port),
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
        let res = &self
            .client
            .get(format!(
                "{}{}",
                &self.host, "/sap/bc/adt/compatibility/graph?sap-client=300"
            ))
            .basic_auth("pfrank", Some("Start123!"))
            .header("x-csrf-token", "Fetch")
            .send()
            .await
            .unwrap();

        self.set_headers_from_headermap(res.headers());
    }
    fn set_headers_from_headermap(&mut self, headers: &HeaderMap) {
        let mut hashmap: HashMap<String, Option<String>> = HashMap::new();
        headers.iter().for_each(|(mut name, value)| {
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
    // fn get_headers_as_headermap(&mut self) -> HeaderMap {
    //     let mut cookies: Vec<String> = vec![];
    //     let mut map = HeaderMap::new();

    //     let headers = self.headers.clone().unwrap();
    //     for (key, val) in headers.iter() {
    //         if key.contains("set-cookie") {
    //             let mut new_val = val.clone().unwrap();
    //             new_val = new_val + ";";
    //             cookies.push(new_val);
    //         } else {
    //             map.append(
    //                 HeaderName::from_str(key).unwrap(),
    //                 HeaderValue::from_str(val.as_ref().unwrap()).unwrap(),
    //             );
    //         }
    //     }
    //     let cookie = cookies.concat();
    //     let xxx = &cookie[0..cookie.len() - 1];
    //     map.append(
    //         HeaderName::from_str("Cookie").unwrap(),
    //         HeaderValue::from_str(&xxx).unwrap(),
    //     );
    //     map
    // }
    pub async fn delete(&mut self, config: &impl SendableConfig) -> Response {
        if self.session.is_none() {
            self.fetch_csrf_token().await;
            println!("{:?}", self.headers.as_ref().unwrap());
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
                session_cookie: self.cookies.get("SAP_SESSIONID_ITK_300").unwrap().clone(), //  session_cookie:
            });
        }

        let url = format!("{0}{1}", &self.host, &config.get_path());

        self.client
            .delete(&url)
            .basic_auth("pfrank", Some("Start123!"))
            .header(
                "x-csrf-token",
                self.session.as_ref().unwrap().csrf_token.clone(),
            )
            .header(
                "Cookie",
                format!(
                    "SAP_SESSIONID_ITK_300={0}",
                    self.session.as_ref().unwrap().session_cookie
                ),
            )
            .header(
                "X-sap-adt-sessiontype",
                &self.session.as_ref().unwrap().session_type,
            )
            .body(String::from(config.get_body()))
            .send()
            .await
            .unwrap()
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
                session_cookie: self.cookies.get("SAP_SESSIONID_ITK_300").unwrap().clone(), //  session_cookie:
            });
        }

        let url = format!("{0}{1}", &self.host, lock_handle.get_lock_handle_path());

        self.client
            .post(&url)
            .basic_auth("pfrank", Some("Start123!"))
            .header(
                "x-csrf-token",
                self.session.as_ref().unwrap().csrf_token.clone(),
            )
            .header(
                "Cookie",
                format!(
                    "SAP_SESSIONID_ITK_300={0}",
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
    // pub async fn send<C, T, E>(&mut self, config: C) -> Response
    // where
    //     C: Config<T, E>,
    //     T: AdtResponse<Responses>,
    //     E: AdtError,
    // {
    pub async fn send(&mut self, config: &impl SendableConfig) -> Response {
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
                session_cookie: self.cookies.get("SAP_SESSIONID_ITK_300").unwrap().clone(), //  session_cookie:
                session_type: if self.stateful {
                    "stateful".to_string()
                } else {
                    "stateless".to_string()
                },
            });
        }

        let url = format!("{0}{1}", &self.host, &config.get_path());

        self.client
            .post(&url)
            .basic_auth("pfrank", Some("Start123!"))
            .header(
                "x-csrf-token",
                self.session.as_ref().unwrap().csrf_token.clone(),
            )
            .header(
                "Cookie",
                format!(
                    "SAP_SESSIONID_ITK_300={0}",
                    self.session.as_ref().unwrap().session_cookie
                ),
            )
            .header(
                "X-sap-adt-sessiontype",
                &self.session.as_ref().unwrap().session_type,
            )
            .body(String::from(config.get_body()))
            .send()
            .await
            .unwrap()
    }

    pub async fn get(&mut self, config: &impl SendableConfig) -> Response {
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
                session_cookie: self.cookies.get("SAP_SESSIONID_ITK_300").unwrap().clone(), //  session_cookie:
                session_type: if self.stateful {
                    "stateful".to_string()
                } else {
                    "stateless".to_string()
                },
            });
        }
        let url = format!("{0}{1}", &self.host, &config.get_path());

        self.client
            .get(&url)
            .basic_auth("pfrank", Some("Start123!"))
            .header(
                "x-csrf-token",
                self.session.as_ref().unwrap().csrf_token.clone(),
            )
            .header(
                "Cookie",
                format!(
                    "SAP_SESSIONID_ITK_300={0}",
                    self.session.as_ref().unwrap().session_cookie
                ),
            )
            .header(
                "X-sap-adt-sessiontype",
                &self.session.as_ref().unwrap().session_type,
            )
            .body(String::from(config.get_body()))
            .send()
            .await
            .unwrap()
    }
}
