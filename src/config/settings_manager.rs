use ini::configparser::ini::Ini;

pub struct SettingsManager {
    path: String,
    config: Ini,
}

impl SettingsManager {
    pub fn init(path: &str) -> Self {
        let mut this = SettingsManager {
            config: Ini::new(),
            path: path.to_string(),
        };

        if this.config.load(&this.path).is_err() {
            std::fs::File::create(&this.path);
        }

        this
    }
    pub fn set_default_sys(&mut self, sys_id: &str) {
        self.config.set("_default", "sys", Some(sys_id.to_string()));
    }
    pub fn update_file(&self) -> std::io::Result<()> {
        self.config.write(&self.path)
    }
    pub fn get_default_sys(&self) -> Option<String> {
        self.config.get("_default", "sys")
    }
}
