use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::Duration;

use fastrace::local::LocalSpan;
use fastrace::prelude::SpanContext;
use fastrace::{Span, trace};
use tokio::sync::mpsc;
use tokio::task::spawn_blocking;
use tracing::{debug, error, info, info_span, instrument, warn};

use crate::graph::graph::{AsyncHandleTrait, Graph, OperatorType};

use crate::graph::message::Message;

use crate::graph::graph::Operator;

use super::op_actor::OpActor;

use fastrace::future::FutureExt;
use tracing::Instrument;

#[derive(Debug)]
pub(crate) struct OperatorActorHandle {
    sender: mpsc::Sender<Message>,
    name: String,
    _nodes: Vec<Arc<Mutex<dyn Operator + 'static>>>,
}

impl OperatorActorHandle {
    //#[instrument[name = "op-actor-handle", skip_all]]
    #[trace]
    pub fn new<T>(operator: T) -> Self
    where
        T: Operator + 'static,
    {
        let nm = operator.name();
        let name = operator.name().clone();
        let (sender, receiver) = mpsc::channel(8);

        let mut actor = OpActor::new(name, receiver, operator);

        //let cx = Context::current();

        // let tracer = global::tracer("saaasexpress_acotr_trace");
        // tracer.in_span("doing_actor_work", |_cx| {
        //     // Your application logic here...
        //     //sleep(Duration::from_secs(100));
        //     info!("doing work");
        // });

        // let span_name = "operator";

        // let tracer = global::tracer("graph");
        // let span = tracer
        //     .span_builder(String::from(span_name))
        //     .with_kind(SpanKind::Client)
        //     .start(&tracer);
        // let cx = Context::current_with_span(span);

        //let the_span = info_span!("Operator-Actor", name = nm.clone());
        // let trace_ctx = Context::current();

        // let tracer = global::tracer("graph");
        // let mut span = tracer
        //     .span_builder("oteldemo.ActorHandle")
        //     .with_kind(SpanKind::Producer)
        //     .start_with_context(&tracer, &trace_ctx);

        // let cx = Context::current_with_span(span);
        //.with_context(cx)
        //let span = LocalSpan::enter_with_local_parent("op_actor_handle");
        //let root = Span::root("op_actor_handle", SpanContext::random());

        //let parent = SpanContext::random();
        //let span = Span::root("root", parent);

        let future = async move {
            //let _span = LocalSpan::enter_with_local_parent("a span");
            // let root = Span::root("op_actor", SpanContext::random());
            // let _g = root.set_local_parent();

            //let child = Span::enter_with_local_parent("child");
            actor.run().await;
        };

        tokio::spawn(future);

        Self {
            name: String::clone(&nm),
            sender,
            _nodes: Vec::new(),
        }
    }
}

impl Operator for OperatorActorHandle {
    fn _type(&self) -> OperatorType {
        OperatorType::Endpoint
    }
    fn name(&self) -> String {
        return self.name.clone();
    }

    fn get(&self) -> Option<Arc<dyn AsyncHandleTrait>> {
        None
    }

    fn handle(&self, _message: Message) -> Message {
        return _message;
    }

    fn init(&mut self, _: &mut Graph) {
        warn!("Not implemented");
    }

    fn control(&mut self, _message: Message) {
        match _message {
            Message::Init { .. } => match self.sender.try_send(_message) {
                Ok(_) => debug!("Message sent to {}", self.name),
                Err(e) => panic!("Failed to send: {}", e),
            },
            _ => {
                panic!("Unexpected message type for control");
            }
        }
    }

    fn send(&self, _message: Message) {
        match _message {
            Message::Init { .. } => {
                error!("Unexpected message type {}", _message);
            }
            _ => match self.sender.try_send(_message) {
                Ok(_) => debug!("Message sent to {}", self.name),
                Err(e) => {
                    error!("Failed to send: {}", e)
                }
            },
        }
    }
    fn wait(&self) -> Message {
        panic!("Not implemented");
    }
    fn get_output_channels(&self) -> &Vec<Arc<Mutex<dyn Operator>>> {
        self._nodes.as_ref()
    }
}
