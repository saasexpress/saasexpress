use serde::Deserialize;
use tracing::info;

use crate::bootstrap::build_graph;

pub(crate) fn config(file_path: String) {
    // check if file is a directory, and if so, read all files in the directory
    if std::fs::metadata(file_path.clone()).unwrap().is_dir() {
        let paths = std::fs::read_dir(file_path).unwrap();
        for path in paths {
            let path = path.unwrap().path();
            if path.is_file() && path.extension().unwrap() == "yaml" {
                read_yaml_file(path.to_str().unwrap().to_string());
            }
        }
    } else {
        read_yaml_file(file_path);
    }
}

fn read_yaml_file(file_path: String) {
    let file = std::fs::read_to_string(file_path).expect("Failed to read file");
    for document in serde_yaml::Deserializer::from_str(&file) {
        let yaml = serde_yaml::Value::deserialize(document).unwrap();
        build_graph(yaml);
    }
}
