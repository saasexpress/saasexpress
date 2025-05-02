use basic::BasicProcessor;

use super::graph::Graph;
use super::message::Message;
use std::fmt::Debug;

pub(crate) mod basic;
pub(crate) mod port;

pub(crate) trait XProcessor: Send + Sync + Debug {
    async fn wait(&mut self) -> Message;
    async fn req_reply(&mut self) -> Message;
}

#[derive(Debug)]
pub enum ProcessorType {
    Basic { processor: BasicProcessor },
}
impl ProcessorType {
    pub fn new_basic(graph: &mut Graph) -> Self {
        let basic = BasicProcessor::new(graph);

        ProcessorType::Basic { processor: basic }
    }
    // pub fn new_complex(graph: &mut Graph) -> Self {
    //     let complex = ComplexProcessor::new(graph);

    //     ProcessorType::Complex { processor: complex }
    // }

    pub async fn req_reply(&mut self) -> Message {
        match self {
            ProcessorType::Basic { processor } => processor.req_reply().await,
            _ => {
                panic!("Processor is not basic");
            }
        }
    }
}
