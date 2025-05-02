use crate::bootstrap::build_graph;

pub(crate) fn stdin() {
    let mut buffer = String::new();
    loop {
        let mut line = String::new();
        std::io::stdin()
            .read_line(&mut line)
            .expect("Failed to read from stdin");

        if line.trim().is_empty() {
            if buffer.ends_with("\n\n") {
                break;
            }
            buffer.push('\n');
        } else {
            buffer.push_str(&line);
        }
    }
    let yaml = serde_yaml::from_slice::<serde_yaml::Value>(buffer.as_bytes()).unwrap();
    build_graph(yaml);
}
