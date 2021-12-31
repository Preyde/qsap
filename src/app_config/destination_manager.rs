use serde_json::from_str;
use std::fs::{read_to_string, write};

use crate::net::Destination;
#[derive(Debug, Clone)]
pub struct DestinationManager {
    dests: Vec<Destination>,
    dests_plain_text_passwd: Vec<Destination>,
    path: String,
}

impl DestinationManager {
    pub fn init(path: String) -> Self {
        let mut this = DestinationManager {
            dests: vec![],
            dests_plain_text_passwd: vec![],
            path,
        };

        DestinationManager::read_destination_file(&mut this);
        this.look_for_unencrypted();
        this
    }
    fn read_destination_file(&mut self) {
        self.dests = from_str(&read_to_string(&self.path).unwrap()).unwrap()
    }
    pub fn has_unencrypted_passwd(&self) -> bool {
        !self.dests_plain_text_passwd.is_empty()
    }

    pub fn get_destinations(&self) -> &Vec<Destination> {
        &self.dests
    }
    pub fn get_destination(&self, sys_id: &str) -> Option<Destination> {
        Some(
            self.dests
                .iter()
                .find(|dest| dest.sys_id == sys_id)?
                .clone(),
        )
    }
    fn look_for_unencrypted(&mut self) {
        self.dests_plain_text_passwd = self
            .dests
            .iter()
            .filter(|dest| {
                dest.passwd.to_lowercase() != format!("{{{}}}", dest.sys_id.to_lowercase())
            })
            .map(|dest| dest.clone())
            .collect();
    }
    pub fn hide_passwd(&mut self, sys_id: &str) {
        self.dests
            .iter_mut()
            .find(|d| d.sys_id == sys_id)
            .unwrap()
            .passwd = format!("{{{}}}", sys_id);
    }

    pub fn write(&self) {
        write(&self.path, serde_json::to_string(&self.dests).unwrap());
    }

    pub fn get_dests_plain_passwd(&self) -> &Vec<Destination> {
        &self.dests_plain_text_passwd
    }
}
