use graph::Operator;

//pub(crate) mod actor;
//pub(crate) mod actor_operator;
pub(crate) mod graph;
pub(crate) mod operators;
pub(crate) mod processors;
pub(crate) mod serde;

pub trait OperatorExecutor {
    fn execute(&self);
}

#[derive(Clone)]
pub struct Wrapper;

impl OperatorExecutor for Wrapper {
    fn execute(&self) {
        println!("Executing operator");
    }
}

impl Wrapper {
    fn as_executor(&self) -> Box<dyn OperatorExecutor> {
        let executor: Box<dyn OperatorExecutor> = Box::new(self.clone());

        return executor;
    }
}
