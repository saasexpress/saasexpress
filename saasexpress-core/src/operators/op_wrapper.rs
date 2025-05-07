use std::sync::{Arc, Mutex};

use fastrace::local::LocalSpan;
use fastrace::prelude::SpanContext;
use fastrace::{Event, Span, trace};
use tracing::{debug, error, info, warn};

use crate::graph::graph::{AsyncHandleTrait, Operator};
use crate::graph::graph::{Graph, OperatorType};
use crate::graph::message::Message;
use crate::graph::meta::NodeMeta;
use fastrace::future::FutureExt;

#[derive(Clone, Debug)]
pub struct OperatorWrapper {
    name: String,
    handle: Arc<Mutex<Box<dyn Operator + 'static>>>,
    _nodes: Vec<Arc<Mutex<dyn Operator + 'static>>>,
}

impl OperatorWrapper {
    #[trace]
    pub fn new<T>(operator: T) -> Self
    where
        T: Operator + 'static,
    {
        let nm = operator.name();

        Self {
            name: String::clone(&nm),
            handle: Arc::new(Mutex::new(Box::new(operator))),
            _nodes: Vec::new(),
        }
    }

    fn middleware(&self, _message: Message) -> Message {
        let mut message = _message;

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

        let hdl = self.handle.lock().unwrap();

        let start_time = std::time::Instant::now();
        debug!("Middleware {:?} {:?}", message, start_time);
        let result = hdl.handle(message);
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
        let hdl = self.handle.lock().unwrap().get().unwrap();

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

impl Operator for OperatorWrapper {
    fn _type(&self) -> OperatorType {
        OperatorType::Endpoint
    }
    fn name(&self) -> String {
        return self.name.clone();
    }

    fn get(&self) -> Option<Arc<dyn AsyncHandleTrait>> {
        let hdl = self.handle.lock().unwrap();

        if hdl.get().is_some() {
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

    fn init(&mut self, _: &mut Graph, node_meta: &NodeMeta) {
        info!("Not implemented");
    }

    fn finalize(&mut self) {
        self.handle.lock().unwrap().finalize();
    }

    fn control(&mut self, _message: Message) {
        match _message {
            Message::Init {
                id,
                next,
                start,
                end,
            } => {
                let mut hdl = self.handle.lock().unwrap();

                hdl.control(Message::Init {
                    id,
                    next,
                    start,
                    end,
                });
            }
            Message::Control { command, origin } => {
                let mut hdl = self.handle.lock().unwrap();

                info!("Control command: {:?}", command);
                hdl.control(Message::Control { command, origin });
            }
            _ => {
                panic!("Unexpected message type for control");
            }
        }
    }

    fn send(&self, _message: Message) {
        let message = self.handle(_message);

        // this has to be after the handle() to avoid deadlock
        let hdl = self.handle.lock().unwrap();

        hdl.send(message);
    }

    fn wait(&self) -> Message {
        panic!("Not implemented");
    }

    fn get_output_channels(&self) -> &Vec<Arc<Mutex<dyn Operator>>> {
        self._nodes.as_ref()
    }
}

impl AsyncHandleTrait for OperatorWrapper {
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
