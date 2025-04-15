use rust_embed::RustEmbed;
use serde_yaml::Value;
use tracing::info;

#[derive(RustEmbed)]
#[folder = "src/bootstrap"]
struct Asset;

pub fn gather_files() -> Vec<Value> {
    Asset::iter()
        .filter(|file_name| file_name.ends_with(".yaml"))
        .map(|file_name| {
            let file = Asset::get(file_name.as_ref()).unwrap();
            let yaml = serde_yaml::from_slice::<serde_yaml::Value>(file.data.as_ref()).unwrap();
            info!("YAML: {} : {:?}", file_name, yaml);
            yaml
        })
        .collect()
}
