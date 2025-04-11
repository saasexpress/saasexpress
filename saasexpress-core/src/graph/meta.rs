pub struct NodeMeta {
    pub id: String,
    pub name: String,
    pub description: String,
    pub tags: Vec<String>,
    pub properties: Vec<String>,
    pub logo: Vec<u8>,
}

impl NodeMeta {
    pub fn new(id: String) -> Self {
        NodeMeta {
            id,
            name: "".to_string(),
            description: "".to_string(),
            tags: Vec::new(),
            properties: Vec::new(),
            logo: Vec::new(),
        }
    }
}
