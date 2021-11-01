use reqwest::{header::HeaderMap, Client, Response};
use std::collections::HashMap;

use crate::config::program_config::Config;

pub struct Session {
    pub csrf_token: String,
    pub session_cookie: String,
}

pub struct SAPClient {
    client: Client,
    headers: Option<HashMap<String, Option<String>>>,
    session: Option<Session>,
    host: String,
    // app_config: AppConfig,
    cookies: HashMap<String, String>,
}

impl SAPClient {
    pub fn new(host: &str) -> SAPClient {
        // let mut app_conf = AppConfig::init();
        SAPClient {
            client: reqwest::Client::builder()
                .cookie_store(false)
                .build()
                .unwrap(),
            host: host.to_string(),
            headers: None,
            session: None,
            cookies: HashMap::new(),
            // app_config: app_conf,
        }
    }
    pub fn from_session(host: &str, session: Session) -> Self {
        SAPClient {
            // app_config: AppConfig::init(),
            client: reqwest::Client::builder()
                .cookie_store(false)
                .build()
                .unwrap(),
            cookies: HashMap::new(),
            session: Some(session),
            host: host.to_string(),
            headers: None,
        }
    }

    pub fn get_session(&self) -> Option<Session> {
        let session = self.session.as_ref()?;
        Some(Session {
            csrf_token: String::from(session.csrf_token.clone()),
            session_cookie: String::from(session.session_cookie.clone()),
        })
    }

    async fn fetch_csrf_token(&mut self) {
        let res = &self
            .client
            .get(format!(
                "{}{}",
                &self.host, "/sap/bc/adt/programs?sap-client=300"
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
                println!("{}", cookie_name.clone());
                println!("{}", cookie_value.clone());
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
    pub async fn send(&mut self, config: &impl Config) -> Response {
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
                session_cookie: self.cookies.get("SAP_SESSIONID_ITK_300").unwrap().clone(), //  session_cookie:
            });
        }
        // println!("{:?}", self.get_headers_as_headermap());
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
            .body(String::from(config.get_body()))
            .send()
            .await
            .unwrap()
    }

    pub async fn get<T: Config>(&mut self, config: T) -> Response {
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
                session_cookie: self.cookies.get("SAP_SESSIONID_ITK_300").unwrap().clone(), //  session_cookie:
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
            .body(String::from(config.get_body()))
            .send()
            .await
            .unwrap()
    }
}
