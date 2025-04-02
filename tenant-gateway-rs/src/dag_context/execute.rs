use log::{debug, info};
use log::{error, warn};

use crate::operators::enums::OperatorType;
use crate::operators::operator::OperatorNode;
use crate::operators::operator::{Message, MessageContext};

use std::any::type_name_of_val;
use std::sync::Arc;

pub struct GraphContext {
    op_nodes: Vec<Box<dyn OperatorNode + Send>>,
}

impl GraphContext {
    pub fn new(nodes: Vec<Box<dyn OperatorNode + Send>>) -> Self {
        GraphContext { op_nodes: nodes }
    }

    // pub fn setup_actors(&self) {
    //     self.op_nodes.iter().for_each(|node| {
    //         info!("node={}", node.name());
    //         node.setup_actors(self);
    //     });
    // }

    pub async fn execute<T>(
        self,
        node_id: &String,
        in_msg: MessageContext<T>,
    ) -> Result<String, String> {
        let msg = Message {
            state: 1,
            log: Vec::new(),
        };

        info!("Execute {} : {}", node_id, type_name_of_val(&in_msg));

        if let Some(op_node) = self.find(node_id) {
            info!(":::::::::::: Execute {} : {}", node_id, op_node.operator());

            let executor = op_node.as_executor();

            // for actor in executor.actors() {
            //     info!(
            //         "[{}] Actor Answer = {}",
            //         node_id,
            //         actor.get_unique_id().await
            //     );
            // }
            executor.notify(&self, msg);

            // let ex2 = op_node.as_executor_2();
            // match (ex2) {
            //     OperatorType::HTTPIn { node } => {
            //         println!("We got a match {}", node.name());
            //     }
            //     OperatorType::ReverseProxy {} => todo!(),
            //     OperatorType::Template {} => todo!(),
            // }
            /*
            let response = executor.process(&msg);

            //op_node.send_to_children();

            for child_id in op_node.children() {
                if let Some(child_node) = self.find(&child_id) {
                    //child_node.as_executor().notify(&self, response.clone());

                    let msg = child_node.as_executor().process(&response);
                    debug!("Child response = {}", msg.log.join(", "));
                    // let actors = child_node.actors();
                    // let actor = actors.get(0).unwrap();
                    // let result = actor.get_unique_id().await;
                    // warn!("[{}] Child result = {}", child_id, result);
                }
            }
            */
        }
        Ok("".to_string())
        //     //
        //     //            self.follow(op_node).await;

        //     let state = in_msg.state;

        //     // send message to first
        // match self.send_message(node_id, in_msg).await {
        //         Ok(_) => {
        //             info!("Sent msg {}", state)
        //         }
        //         Err(_) => {
        //             error!("Error msg")
        //         }
        //     }

        //     // open up an "end" channel and wait/watch

        //     Ok(())
        // } else {
        //     Err("Node not found".to_string())
        // }
    }

    // async fn follow(&self, op_node: &dyn OperatorNode) {
    //     let node = op_node.node();
    //     // let nodes = &self.dag.nodes;
    //     let pnd = node.id.clone();
    //     info!("Execute {} {} {}", pnd, node.action, node.children.len(),);

    //     for nd in &node.children {
    //         // let cnd = nodes.get(nd);
    //         // println!("Send Message to = {}", nd);
    //         // let _ = self.send_message(nd, Message { state: 12 }).await;

    //         let child_node = self.find(nd);
    //         Box::pin(self.follow(child_node.unwrap())).await;
    //     }
    // }

    pub async fn send_message(&self, node_id: &String, message: Message) -> Result<(), String> {
        //let op_node = self.find(node_id).unwrap();

        //let nd = op_node.node();

        Err("Expecting inner details".to_string())
        // let inner = nd.as_ref().node.as_ref();

        // if inner.is_some() {
        //     let ndinner = inner.unwrap();
        //     let ports = &ndinner.ports;

        //     for (sender, _) in ports {
        //         let sender = Arc::clone(sender);
        //         let sender = sender.lock().await;
        //         sender
        //             .send(message.clone())
        //             .await
        //             .map_err(|e| e.to_string())?;
        //     }
        //     Ok(())
        // } else {
        //     Err("Expecting inner details".to_string())
        // }
    }

    pub fn find(&self, node_id: &String) -> Option<&dyn OperatorNode> {
        let op_node = self.op_nodes.iter().filter(|x| x.name() == node_id).next();
        if op_node.is_none() {
            println!("No OpNode found {}", node_id);
            return None;
        }
        if Some(op_node.unwrap()).is_some() {
            return Some(op_node.unwrap().as_ref());
        } else {
            println!("NONE!");
            return None;
        }
    }
}
