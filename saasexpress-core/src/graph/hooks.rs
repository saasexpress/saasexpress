/*
   Hooks are a way of configuring behavior across multiple Graphs
   without having to modify each Graph individually.
   They can be used to add common functionality, such as access control.
*/

use crate::graph::graph::Graph;
use std::fmt::Debug;

pub trait GraphHook: Send + Sync + Debug {
    /// Called when the graph is initialized.
    fn on_init(&self, graph: &mut Graph);

    /// Called when the graph is finalized.
    fn on_finalize(&self, graph: &mut Graph);

    /// Called when a graph is called
    fn on_call(&self, graph: &mut Graph, message: &str);
}
