use chrono::{DateTime, Utc};
use ini::configparser::ini::Ini;
use sap_adt_bindings::net::Session;
use serde::Deserialize;
use serde_json::from_str;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::str::FromStr;
pub mod app_config {}

pub struct AppConfig {
    config: Ini,
    destinations: Vec<Destination>,
}
impl AppConfig {
    pub fn init() -> Self {
        let mut conf = AppConfig {
            config: Ini::new(),
            destinations: vec![],
        };
        if conf.config.load("sapClient.ini").is_err() {
            std::fs::File::create("sapClient.ini");
        }

        conf.read_destination_file();
        conf
    }
    pub fn get_session_from_sys(&mut self, sys_id: &str) -> Option<Session> {
        let section = format!("session_{0}", sys_id);
        let expires_string = self.config.get(&section, "expires")?;
        let expires: DateTime<Utc> = DateTime::from_str(&expires_string).ok()?;

        if expires - Utc::now() <= chrono::Duration::zero() {
            // Session is over
            return None;
        }

        Some(Session {
            csrf_token: self.config.get(&section, "csrf_token")?,
            session_cookie: self.config.get(&section, "session_cookie")?,
            session_type: "stateless".to_string(),
        })
    }
    pub fn set_session_for_sys(&mut self, sys_id: &str, session: &Session) {
        let section = format!("session_{0}", sys_id);
        self.config
            .set(&section, "csrf_token", Some(session.csrf_token.clone()));
        self.config.set(
            &section,
            "session_cookie",
            Some(session.session_cookie.clone()),
        );
        self.config.set(
            &section,
            "expires",
            Some(
                Utc::now()
                    .checked_add_signed(chrono::Duration::minutes(15))
                    .unwrap()
                    .to_string(),
            ),
        );
    }

    pub fn update_file(&mut self) {
        self.config.write("sapClient.ini");
    }
    pub fn get_destination_from_sys(&mut self, sys_id: &str) -> Option<&Destination> {
        self.destinations.iter().find(|dest| dest.sys_id == sys_id)
    }
    fn read_destination_file(&mut self) {
        self.destinations = from_str(
            &read_to_string(r#"C:\Users\103925pafr\Projekte\sapClient\destinations.json"#).unwrap(),
        )
        .unwrap()
    }
}
#[derive(Debug, Deserialize)]
pub struct Destination {
    sys_id: String,
    host: String,
    port: u16,
    uname: String,
    passwd: String,
    mandt: String,
    lang: String,
}
