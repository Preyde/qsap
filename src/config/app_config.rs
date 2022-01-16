use crate::config::PasswordManager;
use crate::net::{Destination, Session};
use chrono::{DateTime, Utc};
use crossterm::event::read;
use crossterm::event::Event;
use crossterm::event::KeyCode;
use crossterm::event::KeyEvent;
use crossterm::event::KeyModifiers;
use ini::configparser::ini::Ini;
use open;
use std::str::FromStr;

use super::destination_manager::DestinationManager;
use crate::crypt::Crypt;

pub mod app_config {}
#[derive(Clone, Debug)]
pub struct AppConfig {
    config: Ini,
    password_manager: PasswordManager,
    sessions_config: Ini,
    destination_manager: DestinationManager,
}

impl AppConfig {
    fn get_path(filename: &str) -> String {
        format!(
            "{}\\sapCli\\{}",
            std::env::var("APPDATA").unwrap(),
            filename
        )
    }
    pub fn open_destination_file() {
        open::that(AppConfig::get_path("destinations.json")).unwrap_or_else(|e| println!("Could not open destination file. Make sure it is at path: C:\\Users\\[Your username]\\AppData\\Roaming\\sapCli\\destinations.json"))
    }
    pub fn init() -> Self {
        // let get_path = |filename: &str| {
        //     format!(
        //         "{}\\sapCli\\{}",
        //         std::env::var("APPDATA").unwrap(),
        //         filename
        //     )
        // };
        // println!("{}", get_path(""));
        if !std::path::Path::new(&AppConfig::get_path("")).exists() {
            std::fs::create_dir(&AppConfig::get_path(""));
        }

        let mut conf = AppConfig {
            config: Ini::new(),
            sessions_config: Ini::new(),
            password_manager: PasswordManager::init(AppConfig::get_path("shadow.ini")),
            destination_manager: DestinationManager::init(AppConfig::get_path("destinations.json")),
        };

        // println!("{}", get_path("sessions.ini"));
        if conf
            .config
            .load(&AppConfig::get_path("settings.ini"))
            .is_err()
        {
            std::fs::File::create(&AppConfig::get_path("settings.ini"));
            conf.set_default_sys("ITK");

            conf.update_file();
        }
        if conf
            .sessions_config
            .load(&AppConfig::get_path("sessions.ini"))
            .is_err()
        {
            std::fs::File::create(&AppConfig::get_path("sessions.ini"));
        }

        // let dir_path = format!(
        //     "{}\\sapCli\\{}",
        //     std::env::var("APPDATA").unwrap(),
        //     filename
        // );

        conf.check_destinations();

        conf
    }

    fn decrypt_password(&self, dest: &mut Destination) {
        let mut crypt = Crypt::from_base64_key_nounce(
            "password",
            &self
                .password_manager
                .get_shadow_nonce(&dest.sys_id)
                .unwrap(),
        );
        let decrypted_passwd = crypt.decrypt(
            &self
                .password_manager
                .get_shadow_passwd(&dest.sys_id)
                .unwrap(),
        );

        dest.passwd = decrypted_passwd;
    }

    fn check_destinations(&mut self) {
        if self.destination_manager.has_unencrypted_passwd() {
            self.ask_to_encrypt();
        }
    }
    /// Ask the user if encrypting passwords is ok. Yes = true, No = false
    fn ask_to_encrypt(&mut self) {
        let mut systems_string = String::new();

        self.destination_manager
            .get_dests_plain_passwd()
            .iter()
            .for_each(|dest| systems_string.push_str(&format!("{}, ", &dest.sys_id)));

        systems_string.pop();
        systems_string.pop();

        println!(
            "Unencrypted passwords in destination file for systems: {}",
            systems_string
        );
        println!("Press [ENTER] to encrypt or any other key to continue");

        let no_modifiers = KeyModifiers::empty();

        loop {
            match read().unwrap() {
                Event::Key(KeyEvent {
                    code: KeyCode::Enter,
                    modifiers: no_modifiers,
                }) => {
                    self.encryption_process();
                    break;
                }
                _ => break,
            }
        }
    }
    /// Encrypts the passwords in destination.json and puts them into the shadow.ini file.
    /// The passwords in the destination.json file are replaced by identifiers
    fn encryption_process(&mut self) {
        for dest in self.destination_manager.get_dests_plain_passwd().clone() {
            let mut crypt = Crypt::new_random("password");

            self.password_manager.write_entry(
                &dest.sys_id,
                &crypt.encrypt(&dest.passwd),
                &crypt.get_nonce_base64(),
            );
            self.destination_manager.hide_passwd(&dest.sys_id);
            // system_ids.push(&dest.sys_id);
        }

        self.password_manager.write();
        self.destination_manager.write();
    }

    pub fn get_default_destination(&self) -> Destination {
        let mut dest = self
            .destination_manager
            .get_destination(&self.get_default_sys())
            .unwrap();

        self.decrypt_password(&mut dest);

        dest
    }

    pub fn get_session_from_sys(&self, sys_id: &str) -> Option<Session> {
        let section = format!("session_{0}", sys_id);
        let expires_string = self.sessions_config.get(&section, "expires")?;
        let expires: DateTime<Utc> = DateTime::from_str(&expires_string).ok()?;

        if expires - Utc::now() <= chrono::Duration::zero() {
            // Session is over
            return None;
        }

        Some(Session {
            csrf_token: self.sessions_config.get(&section, "csrf_token")?,
            session_cookie: self.sessions_config.get(&section, "session_cookie")?,
            session_type: "stateless".to_string(),
        })
    }
    pub fn set_session_for_sys(&mut self, sys_id: &str, session: &Session) {
        let section = format!("session_{0}", sys_id);
        self.sessions_config
            .set(&section, "csrf_token", Some(session.csrf_token.clone()));
        self.sessions_config.set(
            &section,
            "session_cookie",
            Some(session.session_cookie.clone()),
        );
        self.sessions_config.set(
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
        let get_path = |filename: &str| {
            format!(
                "{}\\sapCli\\{}",
                std::env::var("APPDATA").unwrap(),
                filename
            )
        };
        self.sessions_config.write(&get_path("sessions.ini"));
    }
    pub fn get_destination_from_sys(&self, sys_id: &str) -> Option<Destination> {
        let mut dest = self
            .destination_manager
            .get_destinations()
            .iter()
            .find(|dest| dest.sys_id == sys_id)?
            .clone();

        self.decrypt_password(&mut dest);
        // println!("{:?}", dest);
        Some(dest)
    }
    pub fn get_all_destinations(&self) -> Vec<Destination> {
        self.destination_manager.get_destinations().clone()
    }

    pub fn get_default_sys(&self) -> String {
        self.config.get("_default", "sys").unwrap()
    }
    pub fn set_default_sys(&mut self, sys_id: &str) {
        self.config
            .set("_default", "sys", Some(String::from(sys_id)));
    }
}
