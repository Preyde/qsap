use aes;
use aes::cipher::generic_array::GenericArray;
use aes::BlockEncrypt;
use aes::NewBlockCipher;
use chrono::{DateTime, Utc};
use crossterm::event::read;
use crossterm::event::Event;
use crossterm::event::KeyCode;
use crossterm::event::KeyEvent;
use crossterm::event::KeyModifiers;
use crypto::aessafe;
use ini::configparser::ini::Ini;
use sap_adt_bindings::net::Destination;
use sap_adt_bindings::net::Session;
use serde::Deserialize;
use serde_json::from_str;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::io::stdin;
use std::io::stdout;
use std::str::FromStr;
// use text_io::read;

use std::io;
use std::io::prelude::*;

pub mod app_config {}

pub struct AppConfig {
    config: Ini,
    shadow_config: Ini,
    // default_sys: String,
    sessions_config: Ini,
    destinations: Vec<Destination>,
    // destination_string: &'static str,
}
impl AppConfig {
    pub fn init() -> Self {
        let mut conf = AppConfig {
            config: Ini::new(),
            sessions_config: Ini::new(),
            shadow_config: Ini::new(),
            destinations: vec![],
            // destination_string: ,
        };
        if conf.config.load("settings.ini").is_err() {
            std::fs::File::create("settings.ini");
            conf.set_default_sys("ITK");

            conf.update_file();
        }
        if conf.sessions_config.load("sessions.ini").is_err() {
            std::fs::File::create("sessions.ini");
        }
        if conf.shadow_config.load("shadow.ini").is_err() {
            std::fs::File::create("shadow.ini");
        }

        conf.read_destination_file();
        conf.check_destinations();
        conf
    }
    fn check_destinations(&self) {
        let dest_plain_text_passwd: Vec<&Destination> = self
            .destinations
            .iter()
            .filter(|dest| dest.passwd != format!("{0}", dest.sys_id))
            .collect();

        let mut systems_string = String::new();
        dest_plain_text_passwd
            .iter()
            .for_each(|dest| systems_string.push_str(&format!("{}, ", &dest.sys_id)));
        // .collect();
        systems_string.pop();
        systems_string.pop();
        println!(
            "Looks like you got plain text passwords in destination file for systems {}",
            systems_string
        );
        println!("better encrypt them right now? ;)");
        println!("Press [ENTER] to encrypt or any other key to continue");
        // println!("Any other key to cancel");
        // stdout
        // write!(stdout(), "Press any key to continue...").unwrap();
        // stdout().flush().unwrap();

        // // Read a single byte and discard
        // let x = stdin().read(&mut [0u8]).unwrap();
        // println!("{}", x)
        let no_modifiers = KeyModifiers::empty();

        loop {
            //--code--

            match read().unwrap() {
                Event::Key(KeyEvent {
                    code: KeyCode::Enter,
                    modifiers: no_modifiers,
                }) => println!("Alright time to encrypt"),
                _ => break,
            }

            //--code--
        }
        // readline();
        // let mut stdout = stdout().into_raw_mode().unwrap();
        // stdout.flush().unwrap();
        // stdin().events().next();
        // let answer: i32 = read!("{}\n");
        // println!("{}", answer);

        // println!("Looks like you got plain text passwords in destination file.. better encrypt them right now? ;)")
    }
    fn write_to_shadow_file(key: &str, sys_id: &str, passwd: &str) {
        let key = GenericArray::from_slice(&[0u8; 16]);
        let cipher = aes::Aes256::new(&key);

        let mut block = aes::Block::from_slice(passwd.as_bytes()).clone();

        cipher.encrypt_block(&mut block);
        // let block = aes::Block::default();
        // aes::cbc_encryptor(key_size, key, iv, padding)
    }
    pub fn get_default_destination(&mut self) -> Destination {
        self.destinations
            .iter()
            .find(|dest| dest.sys_id == self.get_default_sys())
            .unwrap()
            .clone()
    }

    pub fn get_session_from_sys(&mut self, sys_id: &str) -> Option<Session> {
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
        self.sessions_config.write("sessions.ini");
        self.config.write("settings.ini");
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
    pub fn get_default_sys(&self) -> String {
        self.config.get("_default", "sys").unwrap()
    }
    pub fn set_default_sys(&mut self, sys_id: &str) {
        self.config
            .set("_default", "sys", Some(String::from(sys_id)));
    }
}
// #[derive(Debug, Deserialize)]
// pub struct Destination {
//     sys_id: String,
//     host: String,
//     port: u16,
//     uname: String,
//     passwd: String,
//     mandt: String,
//     lang: String,
// }
