// Operator.Engine(NodeId)
#[derive(Debug)]
pub struct NodeMeta {
    pub id: String,
    pub name: String,
    pub engine: Option<String>,
    pub description: String,
    pub tags: Vec<String>,
    pub properties: Vec<String>,
    pub logo: Vec<u8>,
}

impl NodeMeta {
    pub fn new(id: &str, name: String) -> Self {
        NodeMeta {
            id: id.to_string(),
            name,
            engine: None,
            description: "".to_string(),
            tags: Vec::new(),
            properties: Vec::new(),
            logo: Vec::new(),
        }
    }
}
