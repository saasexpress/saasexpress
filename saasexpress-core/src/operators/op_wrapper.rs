use crate::graph;
use crate::graph::graph::{AsyncHandleTrait, Graph};
use crate::graph::message::Message;
use crate::graph::meta::NodeMeta;
use crate::graph::operator::{
    GraphOperatorContext, Operator, OperatorRef, OperatorRefRead, OperatorRole, OperatorRuntime,
    OperatorRuntimeType, OperatorState, OperatorType,
};
use crate::graph::registry::GraphRegistry;
use fastrace::future::FutureExt;
use fastrace::local::LocalSpan;
use fastrace::prelude::SpanContext;
use fastrace::{Event, Span, trace};
use serde::de;
use std::any::Any;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};
use tracing::{debug, error, info, warn};

#[derive(Debug, Clone)]
pub struct OperatorWrapper {
    id: String,
    name: String,
    //graph_name: String,
    management: Arc<Mutex<dyn Operator + 'static>>,
}

impl OperatorWrapper {
    #[trace]
    pub fn new<T>(graph_name: String, id: String, operator: T) -> Self
    where
        T: Operator + 'static,
    {
        let nm = operator.name();

        Self {
            id,
            //graph_name,
            name: format!("Wrapper({})", String::clone(&nm)),
            management: Arc::new(Mutex::new(operator)),
        }
    }
}

impl Drop for OperatorWrapper {
    fn drop(&mut self) {
        debug!("DROP OperatorWrapper: {}", self.name);
    }
}

impl Operator for OperatorWrapper {
    fn _type(&self) -> OperatorType {
        //self.management.lock().unwrap()._type()
        OperatorType::Filter
    }
    fn name(&self) -> String {
        //self.management.lock().unwrap().name()
        "huh".to_string()
    }

    fn init(&mut self, graph: &mut Graph, node_meta: &NodeMeta) {
        panic!("init never gets called on OperatorWrapper");
        // self.management.lock().unwrap().init(graph, node_meta);
    }

    fn control(&mut self, _message: Message) {
        match _message {
            // Message::Init2 { id, next } => {
            //     let mut hdl = self.management.lock().unwrap();

            //     hdl.control(Message::Init2 { id, next });
            // }
            // Message::Init {
            //     id,
            //     next,
            //     start,
            //     end,
            //     op_runtime,
            // } => {
            //     let mut hdl = self.management.lock().unwrap();

            //     hdl.control(Message::Init {
            //         id,
            //         next,
            //         start,
            //         end,
            //         op_runtime: Arc::clone(&op_runtime),
            //     });
            // }
            Message::Control { command, origin } => {
                // {
                //     self.replace_runtime();
                // }
                let mut hdl = self.management.lock().unwrap();
                hdl.control(Message::Control { command, origin });
            }
            _ => {
                panic!("Unexpected message type for control");
            }
        }
    }

    fn new_runtime(
        &self,
        graph_operator_context: GraphOperatorContext,
    ) -> Arc<dyn OperatorRuntime> {
        let mgmt = self.management.lock().unwrap();

        Arc::new(OperatorWrapperRuntime {
            name: format!("RuntimeWrapper({})", self.name.clone()),
            upstream_runtime: mgmt.new_runtime(graph_operator_context.clone()),
        })
    }
}

#[derive(Debug, Clone)]
pub struct OperatorWrapperRuntime {
    name: String,
    upstream_runtime: Arc<dyn OperatorRuntime + 'static>,
    //next_nodes: Vec<OperatorRole>,
}

impl OperatorRuntime for OperatorWrapperRuntime {
    fn _type(&self) -> OperatorType {
        let hdl = self.upstream_runtime.as_ref();
        hdl._type()
    }
    fn name(&self) -> String {
        return self.name.clone();
    }
    fn state(&self) -> OperatorState {
        let hdl = self.upstream_runtime.as_ref();
        hdl.state()
    }

    fn send(&self, _message: Message) {
        let runtime: &(dyn OperatorRuntime + 'static) = self.upstream_runtime.as_ref();
        let message = runtime.handle(_message);

        if let Message::Error { error, origin } = message {
            error!("Error in operator {}: {}", self.name, error);
            runtime.send(Message::Error { error, origin });
        } else {
            // this has to be after the handle() to avoid deadlock
            runtime.send(message);
        }
    }

    fn get(&self) -> Option<Arc<dyn AsyncHandleTrait>> {
        let async_handle = self.upstream_runtime.as_ref().get();

        if async_handle.is_some() {
            debug!("Async handle found {}", self.name);
            Some(Arc::new(self.to_owned()))
        } else {
            None
        }
    }

    fn handle(&self, _message: Message) -> Message {
        debug!("Handle {}", self.name);

        let nm = format!("op_wrapper_handle_in ({})", self.name);

        match _message.get_span() {
            Some(parent) => {
                let child_span = Span::enter_with_parent(nm, parent);
                child_span.set_local_parent();
                self.middleware(_message)
            }
            None => {
                error!("No span found {} for message {}", self.name, _message);
                let span = Span::root(
                    format!("op_wrapper (no span) ({:?})", _message),
                    SpanContext::random(),
                );
                let _guard = span.set_local_parent();
                self.middleware(_message.with_span(span))
            }
        }
    }
}

impl Drop for OperatorWrapperRuntime {
    fn drop(&mut self) {
        //error!("Dropping OperatorWrapper: {}", self.name);
        // Optionally, you can add cleanup logic here if needed
    }
}

impl OperatorWrapperRuntime {
    fn middleware(&self, _message: Message) -> Message {
        let mut message = _message;

        // If there is an error, then skip forther processing
        if let Message::Error { .. } = message {
            return message;
        }

        if message.get_span().is_some() {
            let nm = format!("middleware ({})", self.name);

            // Two scenarios here:
            // 1. Use the same span as the inbound message
            let child_span = Span::enter_with_parent(nm, message.get_span().unwrap());
            child_span.set_local_parent();

            // 2. Create a new child span (NOT WORKING YET)
            //message.get_span().unwrap().set_local_parent();

            message = message.with_span(child_span);
        } else {
            warn!("No span found {} for message {}", self.name, message);
        }

        let hdl = self.upstream_runtime.as_ref();

        let start_time = std::time::Instant::now();
        debug!("Middleware {:?} {:?}", message, start_time);
        debug!("Handle Type = {:?}", hdl._type());
        let result = match hdl._type() {
            OperatorType::Filter2 { operator } => {
                debug!("Filter2 operator");
                operator.handle(message)
            }
            _ => hdl.handle(message),
        };

        debug!("Result from middleware {:?}", result);
        let elapsed_time = start_time.elapsed();

        tracing::debug!(
            "Middleware execution time for operator {}: {:?}",
            self.name,
            elapsed_time
        );

        LocalSpan::add_event(Event::new(format!(
            "Middleware execution time: {:?}",
            elapsed_time
        )));

        result
    }

    async fn async_middleware(&self, _message: Message) -> Message {
        let hdl = self
            .upstream_runtime
            .as_ref()
            .get()
            .expect("Missing async handle");

        let nm = format!("middleware ({})", self.name);
        let child_span = Span::enter_with_parent(nm, _message.get_span().unwrap());
        child_span.set_local_parent();

        let start_time = std::time::Instant::now();
        let result = hdl.async_handle(_message.with_span(child_span)).await;
        let elapsed_time = start_time.elapsed();

        tracing::debug!(
            "Middleware execution time for operator {}: {:?}",
            self.name,
            elapsed_time
        );

        LocalSpan::add_event(Event::new(format!(
            "Middleware execution time: {:?}",
            elapsed_time
        )));

        result
    }
}

impl AsyncHandleTrait for OperatorWrapperRuntime {
    #[must_use]
    #[allow(
        elided_named_lifetimes,
        clippy::type_complexity,
        clippy::type_repetition_in_bounds
    )]
    fn async_handle<'life0, 'async_trait>(
        &'life0 self,
        message: Message,
    ) -> ::core::pin::Pin<
        Box<dyn ::core::future::Future<Output = Message> + ::core::marker::Send + 'async_trait>,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        //let hdl = self.handle.lock().unwrap().get().unwrap();

        let nm = format!("op_actor_handler (async) ({})", self.name);
        let parent_span = message.get_span();
        let span = match parent_span {
            Some(span) => Span::enter_with_parent(nm, span),
            None => {
                error!("No span found {}", nm);
                Span::root(nm, SpanContext::random())
            }
        };
        let _guard = span.set_local_parent();

        Box::pin(
            async move {
                match message {
                    _ => self.async_middleware(message).await,
                }
            }
            .in_span(span),
        )
    }

    #[must_use]
    #[allow(
        elided_named_lifetimes,
        clippy::type_complexity,
        clippy::type_repetition_in_bounds
    )]
    fn async_handle_ptr<'life0, 'async_trait>(
        &'life0 self,
        _message: Arc<Message>,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = Arc<Message>> + ::core::marker::Send + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async move { Arc::new(Message::NoOp {}) })
    }
}
