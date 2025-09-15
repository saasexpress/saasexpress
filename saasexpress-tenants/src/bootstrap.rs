use rust_embed::RustEmbed;
use serde_yaml::Value;
use tracing::debug;

#[derive(RustEmbed)]
#[folder = "src/bootstrap_all"]
struct Asset;

pub fn gather_files() -> Vec<(String, Value)> {
    Asset::iter()
        .filter(|file_name| file_name.ends_with(".yaml"))
        .map(|file_name| {
            let file = Asset::get(file_name.as_ref()).unwrap();
            let yaml = serde_yaml::from_slice::<serde_yaml::Value>(file.data.as_ref()).unwrap();
            debug!("YAML: {}", file_name);
            let mut parts = file_name.split("/");
            let service_id = parts.nth(0).unwrap().to_string();
            (service_id, yaml)
        })
        .collect()
}
