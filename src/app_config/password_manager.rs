use ini::configparser::ini::Ini;
use rustc_serialize::base64::FromBase64;

/// Interface to shadow File which holds the encrypted password and the auto generated nonce
pub struct PasswordManager {
    config: Ini,
    path: String,
}

impl PasswordManager {
    pub fn init(path: String) -> Self {
        let mut this = PasswordManager {
            config: Ini::new(),
            path,
        };

        if this.config.load(&this.path).is_err() {
            std::fs::File::create(&this.path);
        }

        this
    }
    pub fn write_entry(
        &mut self,
        sys_id: &str,
        base64_passwd: &str,
        // base64_key: &str,
        base64_nonce: &str,
    ) {
        self.config
            .set(sys_id, "passwd", Some(String::from(base64_passwd)));

        self.config
            .set(sys_id, "nonce", Some(String::from(base64_nonce)));
    }
    pub fn get_shadow_passwd(&self, sys_id: &str) -> Option<String> {
        self.config.get(sys_id, "passwd")
    }

    pub fn get_shadow_nonce(&self, sys_id: &str) -> Option<String> {
        self.config.get(sys_id, "nonce")
    }
    pub fn write(&self) {
        self.config.write(&self.path);
    }
}
