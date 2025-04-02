use tracing::error;
use tracing::info;

use crate::operators::operator::MessageContext;
use crate::operators::register;
use crate::{
    dag::dag::Graph,
    dag_context::execute::GraphContext,
    operators::operator::{self, Message},
};

pub async fn execute_dag<T>(mut dag: Graph, message: MessageContext<T>) -> Result<String, String> {
    let operator_registry = register();

    // println!(
    //     "Path from A to C: {}",
    //     dag.has_path(&"start".to_string(), &"rp".to_string())
    // );
    // println!(
    //     "Path from C to A: {}",
    //     dag.has_path(&"C".to_string(), &"A".to_string())
    // );

    let op_nodes = dag.init_node_operators(operator_registry);

    let ctx = GraphContext::new(op_nodes);

    let start_node = dag.get_start_node();

    // dag.get_nodes().iter().for_each(|(_, node)| {
    //     println!("node={}", node.get_id());

    //     let actor_handler = node.node.actors.get(0).unwrap();
    //     let fut = async { actor_handler.get_unique_id().await };

    //     async_std::task::block_on(fut);
    // });
    // let op_nodes: Vec<Box<dyn OperatorNode>> = dag.prepare(operator_registry);

    info!("Executing Graph... {}", start_node);
    match ctx.execute(start_node, message).await {
        Ok(result) => {
            info!("Yippy");
            return Ok(result);
        }
        Err(err) => {
            error!("Error {}", err);
            return Err(err);
        }
    }

    // let wait = time::Duration::from_secs(1);
    // thread::sleep(wait);
}
