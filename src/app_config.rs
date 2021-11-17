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
use std::ascii::AsciiExt;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::io::stdin;
use std::io::stdout;
use std::str::FromStr;
// use text_io::read;

use std::io;
use std::io::prelude::*;

use crate::crypt::Crypt;

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
        conf.decrypt_passwords();
        conf.check_destinations();
        conf
    }

    fn decrypt_passwords(&mut self) {
        let mut new_destinations: Vec<Destination> = vec![];
        let x = format!("{{{}}}, ", &self.destinations[0].sys_id.to_lowercase());
        println!("{}", x);
        for dest in self.destinations.iter() {
            if dest.passwd != format!("{{{}}}", &dest.sys_id.to_lowercase()) {
                println!("XXX");
                new_destinations.push(dest.clone());
                continue;
            }
            let mut crypt = Crypt::from_base64_key_nounce(
                "password",
                &self.get_shadow_nonce(&dest.sys_id).unwrap(),
            );
            let decrypted_passwd = crypt.decrypt(&self.get_shadow_passwd(&dest.sys_id).unwrap());
            println!("{}", decrypted_passwd);
            let mut new_dest = dest.clone();
            println!("{}", &decrypted_passwd);
            new_dest.passwd = decrypted_passwd;

            new_destinations.push(new_dest);
        }
        println!("{:?}", &new_destinations);
        self.destinations = new_destinations;
    }

    fn check_destinations(&mut self) {
        let destinations = &self.destinations.clone();

        let unencrypted_destinations: Vec<&Destination> = destinations
            .iter()
            .filter(|dest| dest.passwd != format!("{{{}}}", dest.sys_id.to_lowercase()))
            .collect();

        if unencrypted_destinations.is_empty() {
            return;
        }
        let mut systems_string = String::new();
        unencrypted_destinations
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
                }) => {
                    self.write_to_shadow_file(unencrypted_destinations);
                    break;
                    // for dest in unencrypted_destinations.iter() {
                    //     let base64_passwd = Crypt::new_random().encrypt(&dest.passwd);
                    //     self.write_shadow_entry(dest, &base64_passwd);
                    // }
                }
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
    fn get_shadow_passwd(&self, sys_id: &str) -> Option<String> {
        self.shadow_config.get(sys_id, "passwd")
    }
    fn get_shadow_key(&self, sys_id: &str) -> Option<String> {
        self.shadow_config.get(sys_id, "key")
    }
    fn get_shadow_nonce(&self, sys_id: &str) -> Option<String> {
        self.shadow_config.get(sys_id, "nonce")
    }

    fn write_to_shadow_file(&mut self, destinations: Vec<&Destination>) {
        for dest in destinations.iter() {
            let mut crypt = Crypt::new_random("password");

            self.write_shadow_entry(
                dest,
                &crypt.encrypt(&dest.passwd),
                &crypt.get_key_base64(),
                &crypt.get_nonce_base64(),
            );
        }
    }

    fn write_shadow_entry(
        &mut self,
        destination: &Destination,
        base64_passwd: &str,
        base64_key: &str,
        base64_nonce: &str,
    ) {
        self.shadow_config.set(
            &destination.sys_id,
            "passwd",
            Some(String::from(base64_passwd)),
        );
        self.shadow_config
            .set(&destination.sys_id, "key", Some(String::from(base64_key)));
        self.shadow_config.set(
            &destination.sys_id,
            "nonce",
            Some(String::from(base64_nonce)),
        );
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
        self.shadow_config.write("shadow.ini");
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
