use std::sync::{Arc, Mutex};

use tokio::sync::{mpsc, oneshot};
use tracing::{debug, error, info, warn};

use crate::graph::message::Message;

use crate::graph::graph::{AsyncHandleTrait, Graph, Operator, OperatorType};
use crate::operators::shell::process::ShellProcess;

use super::resources::get_instance;

#[derive(Clone, Debug)]
pub(crate) struct Shell {
    command: String,
    args: Vec<String>,
    next: Vec<Arc<Mutex<dyn Operator + 'static>>>,
}

impl From<serde_yaml::Value> for Shell {
    fn from(value: serde_yaml::Value) -> Self {
        let command = value
            .get("command")
            .and_then(|v| v.as_str())
            .unwrap_or("bash")
            .to_string();

        let args = value
            .get("args")
            .and_then(|v| v.as_sequence())
            .map(|seq| {
                seq.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        Shell {
            command,
            args,
            next: Vec::new(),
        }
    }
}

impl Operator for Shell {
    fn _type(&self) -> OperatorType {
        OperatorType::Endpoint
    }

    fn name(&self) -> String {
        "Shell".to_string()
    }

    fn get(&self) -> Option<Arc<dyn AsyncHandleTrait>> {
        None
    }

    fn handle(&self, message: Message) -> Message {
        match message {
            Message::Exit { origin, .. } => {
                info!("Exit message received");
                // if there is an origin Session, then reference it
                let session_id = origin
                    .as_ref()
                    .and_then(|o| o.session.clone())
                    .unwrap_or_default();
                let mut proc_list = get_instance().lock().unwrap();
                let shell_process = proc_list.get_process(session_id.clone());
                if let Some(mut shell_process) = shell_process {
                    debug!("Stopping shell process");
                    shell_process.stop();
                } else {
                    warn!("No shell process found for session id {:?}", session_id);
                }

                return Message::Exit { origin: None };
            }
            // Message::Error { error, .. } => {
            //     info!("Error message received");
            //     // if there is an origin Session, then reference it
            //     let session_id = origin
            //         .as_ref()
            //         .and_then(|o| o.session.clone())
            //         .unwrap_or_default();
            //     let mut proc_list = get_instance().lock().unwrap();
            //     let shell_process = proc_list.get_process(session_id.clone());
            //     if let Some(mut shell_process) = shell_process {
            //         debug!("Stopping shell process");
            //         shell_process.stop();
            //     } else {
            //         warn!("No shell process found for session id {:?}", session_id);
            //     }

            //     return Message::Error { error };
            // }
            Message::JSON {
                message, origin, ..
            } => {
                // if there is an origin Session, then reference it
                let session_id = origin
                    .as_ref()
                    .and_then(|o| o.session.clone())
                    .unwrap_or_default();

                info!("Message for session id {:?}", session_id);
                let mut processes = get_instance().lock().unwrap();

                let process = processes.get_process(session_id.clone());

                let mut shell_process = match process {
                    Some(shell_process) => {
                        debug!("Process already exists, reusing it");
                        shell_process
                    }
                    None => {
                        let (ctrl_tx, ctrl_rx) = oneshot::channel::<String>();

                        let origin = origin.unwrap();

                        if origin.mpsc_respond_to.is_none() {
                            warn!("No mpsc_respond_to channel found");
                            let respond_to = origin.respond_to;

                            let (tx, mut rx) = mpsc::channel::<Message>(10);

                            let mut shell_process = ShellProcess::new(tx);

                            let command = self.command.clone();
                            let args = self.args.clone();

                            shell_process.start(&command, &args, ctrl_tx);

                            tokio::spawn(async move {
                                let mut lines = Vec::new();
                                while let Some(message) = rx.recv().await {
                                    debug!("Received message from shell process");
                                    match message {
                                        Message::Standard { message, .. } => {
                                            lines.push(String::from_utf8(message).unwrap());

                                            debug!("Standard message received");
                                        }
                                        Message::JSON { message, .. } => {
                                            let j = serde_json::to_string(&message).unwrap();
                                            debug!("JSON message received");
                                            lines.push(j);
                                        }
                                        _ => {
                                            error!("Unexpected message type");
                                        }
                                    }
                                }
                                info!("Flushing out the lines back to user");
                                let r = respond_to.send(Message::JSON {
                                    message: serde_json::to_value(lines).unwrap(),
                                    origin: None,
                                });
                                if let Err(e) = r {
                                    warn!("Error sending message: {:?}", e);
                                }
                            });

                            let session_id = session_id.clone();
                            tokio::spawn(async move {
                                let _ = ctrl_rx.await;
                                info!("Shell process finished");
                                let mut proc_list = get_instance().lock().unwrap();
                                proc_list.get_process(session_id);
                            });

                            shell_process
                        } else {
                            let respond_to = origin.mpsc_respond_to.unwrap();

                            let mut shell_process = ShellProcess::new(respond_to);

                            let command = self.command.clone();
                            let args = self.args.clone();

                            shell_process.start(&command, &args, ctrl_tx);

                            let session_id = session_id.clone();
                            tokio::spawn(async move {
                                let _ = ctrl_rx.await;
                                info!("Shell process finished");
                                let mut proc_list = get_instance().lock().unwrap();
                                proc_list.get_process(session_id);
                            });

                            shell_process
                        }
                    }
                };

                shell_process.command(
                    message
                        .get("command")
                        .unwrap()
                        .as_str()
                        .unwrap()
                        .as_bytes()
                        .to_vec(),
                );

                processes.add_process(session_id, shell_process);

                info!("Finished processing");
                return Message::Standard {
                    message: "Started".to_string().into_bytes().to_vec(),
                    origin: None,
                };
            }

            _ => panic!("Unexpected message type {}", message),
        }
    }

    fn init(&mut self, _graph: &mut Graph) {
        info!(
            "Initializing shell operator with command: {} {}",
            self.command,
            self.args.join(" ")
        );
    }

    fn control(&mut self, _message: Message) {
        match _message {
            Message::Init { next, .. } => {
                for n in next {
                    self.add_next(n);
                }
            }
            _ => {
                panic!("Unexpected message type for control");
            }
        }
    }
    fn send(&self, message: Message) {
        //panic!("Send not implemented for Shell operator");
        self.next(self.handle(message));
    }

    fn wait(&self) -> Message {
        panic!("Wait not implemented for Shell operator");
    }

    fn get_output_channels(&self) -> &Vec<Arc<Mutex<dyn Operator>>> {
        panic!("Get output channels not implemented for Shell operator");
    }
}

impl Shell {
    fn next(&self, message: Message) {
        let mut counter = 0;
        for n in &self.next {
            if counter == 0 {
                n.lock().unwrap().send(message);
                break;
            } else {
                info!("Not implemented");
            }
            counter = counter + 1;
        }
    }

    fn add_next(&mut self, operator: Arc<Mutex<dyn Operator + 'static>>) {
        self.next.push(operator);
    }
}
