// Operator.Engine(NodeId)
#[derive(Debug, Clone)]
pub struct NodeMeta {
    pub graph: String,
    pub name: String,
    pub operator: String,
    pub engine: Option<String>,
    pub description: String,
    pub tags: Vec<String>,
    pub properties: Vec<String>,
    pub logo: Vec<u8>,
}

impl NodeMeta {
    pub fn new(graph: &str, name: &str, operator: String) -> Self {
        NodeMeta {
            graph: graph.to_string(),
            name: name.to_string(),
            operator,
            engine: None,
            description: "".to_string(),
            tags: Vec::new(),
            properties: Vec::new(),
            logo: Vec::new(),
        }
    }

    pub fn fqn(&self) -> String {
        let engine = if self.engine.is_some() {
            format!(".{}", self.engine.as_ref().unwrap())
        } else {
            "".to_string()
        };

        format!("{}.{}({}{})", self.graph, self.name, self.operator, engine)
    }

    pub fn base_env_vars_settings(&self, node_meta: &NodeMeta) -> String {
        format!("{}_{}_", node_meta.graph, node_meta.name)
            .replace("-", "_")
            .to_uppercase()
    }
}
