use chrono::{DateTime, Utc};
use ini::configparser::ini::Ini;
use sap_adt_bindings::net::Session;
use std::str::FromStr;

pub mod app_config {}

pub struct AppConfig {
    config: Ini,
}
impl AppConfig {
    pub fn init() -> Self {
        let mut conf = AppConfig { config: Ini::new() };
        if conf.config.load("sapClient.ini").is_err() {
            std::fs::File::create("sapClient.ini");
        }

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
}
