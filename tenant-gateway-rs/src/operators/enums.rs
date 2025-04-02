use super::{
    buffer_to_json::BufferToJSONNode,
    // httpin::HTTPInNode,
    json_to_buffer::JSONToBufferNode,
    operator::{OperatorExecutor, OperatorNode},
};

pub enum OperatorType {
    //HTTPIn { node: HTTPInNode },
    //ReverseProxy {},
    //Template {},
    BufferToJSON { node: BufferToJSONNode },
    JSONToBuffer { node: JSONToBufferNode },
}

impl OperatorType {
    // pub fn get_it(&self) -> Box<dyn OperatorNode> {
    //     match self {
    //         OperatorType::HTTPIn { node } => {
    //             return Box::new(node);
    //         }
    //         OperatorType::ReverseProxy {} => todo!(),
    //         OperatorType::Template {} => todo!(),
    //     }
    // }
}
