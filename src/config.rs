use confy;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MyConfig {
    pub dir: String,
    pub server_name: String,
    pub file_sharing_port: u16,
    pub file_sharing_directory: String,
}

impl std::default::Default for MyConfig {
    fn default() -> Self {
        Self {
            dir: format!("{}/dev", std::env::var("HOME").unwrap()),
            server_name: String::from(""),
            file_sharing_port: 8080,
            file_sharing_directory: String::from(""),
        }
    }
}

pub fn load_config() -> MyConfig {
    let mut cfg: MyConfig = confy::load("sysadmin", "config").unwrap();
    cfg.dir = shellexpand::tilde(&cfg.dir).to_string();
    cfg
}
