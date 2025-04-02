use std::collections::HashSet;
use std::sync::Arc;
use std::thread;

use log::{debug, error, info, warn};
use tokio::{io, task};

use tokio::sync::mpsc::{self, Receiver, Sender};
use tokio::sync::Mutex;

use crate::dag::dag::Graph;
use crate::dag_context::execute::GraphContext;
use crate::operators::operator::Message;
use crate::operators::{
    operator::{OperatorExecutor, OperatorNode},
    registry::Registry,
};

use super::actor_handle::MyActorHandle;

pub struct Wrapper {
    pub child_nodes: Vec<String>,
    pub child_nodes_2: Vec<Box<dyn OperatorExecutor>>,
}

pub trait OperatorBinding {
    fn init_node_operators(self, registry: Registry) -> Vec<Box<dyn OperatorNode>>;

    // fn prepare(self, op_nodes: Vec<Box<dyn OperatorNode>>) -> Vec<Box<dyn OperatorNode>>;
    // fn prepare_nodes(&self, op_node: Arc<&dyn OperatorNode>, child_nodes: Wrapper);
    // fn spawn_work(&self, a: String);
}

impl Graph {
    pub fn init_node_operators(&mut self, registry: Registry) -> Vec<Box<dyn OperatorNode + Send>> {
        let mut op_nodes: Vec<Box<dyn OperatorNode + Send>> = Vec::new();

        for node in self.get_nodes().values_mut() {
            let mut op_node = node.init_operator(&registry);

            //op_node.setup_actors(GraphContext::new(op_nodes));

            op_nodes.push(op_node);
        }

        return op_nodes;
    }

    // fn prepare(self, op_nodes: Vec<Box<dyn OperatorNode>>) -> Vec<Box<dyn OperatorNode>> {
    //     // Channels need to be created MPSC
    // }

    /*
    fn prepare(self, op_nodes: Vec<Box<dyn OperatorNode>>) -> Vec<Box<dyn OperatorNode>> {
        // create a "transfer" broadcast channel on a node that
        // the
        // create a broadcast channel and give it to each parent node
        // the parent will subscribe to that channel and
        // create a broadcast channel for each node
        // that the message_handler will send a message to when it has received
        // a message

        //self.spawn_work("joe".to_string());

        // listen for messages on the node ports
        for node in &op_nodes {
            let nd = node.as_ref();
            let mut child_nodes: Vec<Box<(dyn OperatorExecutor)>> = Vec::new();

            let chldren = nd.node().children();

            for child in nd.node().children().iter() {
                info!("Child {}", child);
                let node_id = &child;
                let op_node = op_nodes.iter().filter(|x| x.name() == *node_id).next();

                let nd = op_node.unwrap().as_ref();
                let executor = nd.as_executor();

                child_nodes.push(executor);
            }

            let childs = Wrapper {
                child_nodes: Vec::new(),
                child_nodes_2: child_nodes,
            };

            self.prepare_nodes(Arc::new(node.as_ref()), childs);
        }
        return op_nodes;
    }

    fn prepare_nodes(&self, op_node: Arc<&dyn OperatorNode>, child_nodes: Wrapper) {
        //let op_node_cloned = Arc::clone(&op_node);
        //let executor = op_node_cloned.as_executor();
        let node = op_node.node();
        let pnd = node.id.clone();
        println!("Prepare {} {}", pnd, node.action);
        let chl = &node.node.ports[0];

        let receiver = Arc::clone(&chl.1);
        // op_node.prepare();

        let executor = op_node.as_executor();

        info!("prepare");

        tokio::spawn(async move {
            info!("prepare inside thread");
            message_handler(pnd.to_string(), child_nodes, receiver, executor).await;
        });

        // // Spawn a new task to listen for messages on the node's receiver port
        // for (_, receiver) in &node.node.ports {
        //     let receiver = Arc::clone(receiver);
        //     let pnd_clone = pnd.clone();
        //     let op_node_clone = Arc::clone(&op_node);

        //     // self.message_handler(pnd_clone, receiver, op_node_clone)
        //     //     .await;
        //     let message = Message { state: 3 };

        //     //let executor = op_node.as_executor().process(&message);

        //     task::spawn(async move { self.message_handler(pnd_clone, receiver, executor) });
        //     // if let Some(o    bj_b) = op_node
        //     //     .as_any()
        //     //     .downcast_ref::<Arc<dyn OperatorExecutor + Send + Sync>>()
        //     // {
        //     //     task::spawn(async move {});
        //     // } else {
        //     //     error!("Failed to cast to OperatorExecutor");
        //     // }
        // }
    }

    fn spawn_work(&self, a: String) {
        tokio::spawn(async move {
            warn!("Hello {}", a);
        });
    }
    */
}

// fn find(op_nodes: Vec<Box<&dyn OperatorNode>>, node_id: &String) -> &dyn OperatorNode {
//     let op_node = op_nodes.iter().filter(|x| x.name() == node_id).next();
//     if op_node.is_none() {
//         println!("No OpNode found {}", node_id);
//         return None;
//     }
//     if Some(op_node.unwrap()).is_some() {
//         return Some(op_node.unwrap().as_ref());
//     } else {
//         println!("NONE!");
//         return None;
//     }
// }

pub async fn message_handler(
    pnd: String,
    child_nodes: Wrapper,
    receiver: Arc<Mutex<Receiver<Message>>>,
    executor: Box<dyn OperatorExecutor + Send + Sync>,
) {
    println!("Spawned to receive {}", pnd);
    let mut receiver = receiver.lock().await;
    while let Some(message) = receiver.recv().await {
        println!("Received message on node {}: {}", pnd, message.state);
        // Process the message here
        //op_node_clone.speak();

        // let mut msg = executor.process(&message);

        // msg.state = msg.state + 1;

        // if child_nodes.child_nodes_2.len() == 0 {
        //     warn!("End {}", pnd);
        // }
        // for child in child_nodes.child_nodes_2.iter() {
        //     debug!("Child {}", msg.state);
        //     let chl = &child.node().node.ports[0];
        //     let sender = &chl.0;
        //     let sender = Arc::clone(&sender);
        //     let sender = sender.lock().await;
        //     if let Ok(()) = sender.send(msg.clone()).await.map_err(|e| e.to_string()) {
        //         info!("Send ok")
        //     }
        // }

        // if let Ok(result) = executor.process(&message) {
        //     debug!("process() looks ok! {}", result.is_some());

        //     let msg = result.unwrap();

        //     for child in child_nodes.child_nodes_2.iter() {
        //         debug!("Child {}", msg.state);
        //         let chl = &child.node().node.ports[0];
        //         let sender = &chl.0;
        //         let sender = Arc::clone(&sender);
        //         loop {
        //             let sender = sender.lock().await;
        //             sender.send(msg.clone()).await.map_err(|e| e.to_string());
        //         }
        //     }
        // } else {
        //     error!("process() error");
        // }
    }
}
