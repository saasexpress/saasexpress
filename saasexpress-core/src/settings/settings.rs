use std::env;

use dotenv::dotenv;
use serde::Serialize;
use tracing::info;

#[derive(Clone, Debug, Serialize)]
pub struct Setting {
    pub key: String,
    pub value: String,
}

pub fn env_settings(base: String) -> Vec<Setting> {
    dotenv().ok();

    let mut settings = Vec::new();
    info!("Looking for env vars with prefix: {}", base);
    for (key, value) in env::vars().filter(|(k, _)| k.starts_with(&base)) {
        let setting = Setting {
            key: key[base.len()..].to_string(),
            value,
        };
        info!("[{}] key: {}, value: {}", base, setting.key, setting.value);
        settings.push(setting);
        info!("{:?}", settings);
    }
    settings
}

pub trait ToHashMap {
    fn to_hash_map(&self) -> std::collections::HashMap<String, String>;
}

impl ToHashMap for Vec<Setting> {
    fn to_hash_map(&self) -> std::collections::HashMap<String, String> {
        let mut map = std::collections::HashMap::new();
        for setting in self {
            map.insert(setting.key.clone(), setting.value.clone());
        }
        map
    }
}
