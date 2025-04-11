use crate::graph::message::Message;

use crate::operators::shell::resources::get_instance;
use futures::SinkExt;
use futures::StreamExt;
use futures::channel::mpsc::{self as FuturesMPSC};
use serde_json::json;
use std::io::{BufRead, Write};
use std::process::Stdio;
use std::thread;
use std::thread::sleep;
use std::time::{Duration, Instant};
use tokio::io;
use tokio::io::AsyncWriteExt;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;
use tokio::sync::mpsc;
use tokio::sync::oneshot;
use tokio::task::JoinHandle;
use tracing::{debug, error, info, warn};

#[derive(Debug)]
pub struct ShellProcess {
    child: Option<tokio::process::Child>,
    sender: mpsc::Sender<Message>,
    stdin_sender: Option<mpsc::Sender<String>>,
    handlers: Vec<JoinHandle<()>>,
}

impl ShellProcess {
    pub fn new(sender: mpsc::Sender<Message>) -> Self {
        ShellProcess {
            child: None,
            sender,
            stdin_sender: None,
            handlers: Vec::new(),
        }
    }

    pub fn command(&mut self, cmd: Vec<u8>) {
        let stdin_sender = self.stdin_sender.clone().unwrap();

        tokio::task::spawn(async move {
            // let mut stdin = self.stdin.as_mut().unwrap();
            // stdin.write_all(&cmd).expect("Failed to write to stdin");
            // stdin.flush().expect("Failed to flush stdin");
            let result = stdin_sender.send(String::from_utf8(cmd).unwrap()).await;
            match result {
                Ok(_) => {
                    info!("Command sent successfully");
                }
                Err(e) => {
                    error!("Failed to send command: {}", e);
                }
            }
        });
    }

    pub fn stop(&mut self) {
        warn!("Do some stopping stuff");
    }

    pub fn start(&mut self, command: &str, args: &[String], ctrl_tx: oneshot::Sender<String>) {
        //let mut sender = self.sender.to_owned();

        let mut child = Command::new(command)
            .args(args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("Failed to start shell process");

        let (stdin_tx, mut stdin_rx) = mpsc::channel::<String>(8);

        info!("PID = {}", child.id().unwrap());

        //let (tx, mut rx) = mpsc::channel::<String>(10);

        // batch up responses
        // let hdl = tokio::spawn(async move {
        //     while let Some(line) = rx.recv().await {
        //         info!("Got a line - sending to operator - {}", line);
        //         let result = sender
        //             .send(Message::Standard {
        //                 message: line.into_bytes(),
        //                 origin: None,
        //             })
        //             .await;
        //         match result {
        //             Ok(_) => {}
        //             Err(e) => {
        //                 error!("Error sending line to channel: {}", e);
        //             }
        //         };
        //     }
        //     /*
        //     loop {
        //         info!("Length? {}", rx);
        //         match rx.recv().await.unwrap() {
        //             Ok(Some(line)) => {
        //                 let result = sender
        //                     .send(Message::Standard {
        //                         message: line.into_bytes(),
        //                         origin: None,
        //                     })
        //                     .await;
        //                 match result {
        //                     Ok(_) => {}
        //                     Err(e) => {
        //                         error!("Error sending line to channel: {}", e);
        //                     }
        //                 };
        //             }
        //             Ok(None) => {
        //                 warn!("No lines available");
        //                 break;
        //             }

        //             Err(_err) => {
        //                 // channel is empty
        //                 //warn!("Warn receiving line {}", err);
        //                 //block_wait = true;
        //             }
        //         }

        //         // Small sleep to avoid busy-waiting
        //         thread::sleep(Duration::from_millis(300));
        //     }
        //     */
        //     warn!("EXITING RX RCV! NO!");
        // });
        // self.handlers.push(hdl);

        //let stdout = child.stdout.unwrap();
        let stderr = child
            .stderr
            .take()
            .expect("child did not have a handle to stderr");

        let stdout = child
            .stdout
            .take()
            .expect("child did not have a handle to stdout");

        let mut out_reader = BufReader::new(stdout).lines();
        let mut err_reader = BufReader::new(stderr).lines();

        // Spawn a thread to read lines from stderr
        //let tx1 = tx.clone();
        let sender = self.sender.clone();
        let hdl = tokio::spawn(async move {
            info!("Waiting for error lines");
            loop {
                for line in err_reader.next_line().await {
                    match line {
                        Some(line) => {
                            info!("[ERR]: {}", line);

                            let result = sender
                                .send(Message::Standard {
                                    message: serde_json::to_vec(
                                        &json!({"type":"err", "line":line}),
                                    )
                                    .unwrap(),
                                    origin: None,
                                })
                                .await;
                            //sender.close().await.unwrap();

                            match result {
                                Ok(_) => {}
                                Err(e) => {
                                    error!("Error sending line to channel: {}", e);
                                }
                            };
                        }
                        None => {
                            error!("No lines from stderr");
                            return;
                        }
                    }
                }
            }
        });
        self.handlers.push(hdl);

        // Spawn a thread to read lines from stdout
        // let mut tx2 = tx.clone();
        let sender = self.sender.clone();
        let hdl = tokio::spawn(async move {
            loop {
                for line in out_reader.next_line().await {
                    match line {
                        Some(line) => {
                            info!("[OUT]: {}, {}", sender.is_closed(), line);

                            let result = sender
                                .send(Message::Standard {
                                    message: serde_json::to_vec(
                                        &json!({"type":"out", "line":line}),
                                    )
                                    .unwrap(),
                                    origin: None,
                                })
                                .await;
                            //sender.close().await.unwrap();

                            match result {
                                Ok(_) => {}
                                Err(e) => {
                                    error!("Error sending line to channel: {}", e);
                                }
                            };

                            // match tx.send(line).await {
                            //     Ok(_) => {}
                            //     Err(e) => {
                            //         error!("Error sending line to channel: {}", e);
                            //     }
                            // }
                        }
                        None => {
                            error!("No lines from stdout");
                            return;
                        }
                    }
                }
            }
        });
        self.handlers.push(hdl);

        // pass commands to process stdin
        //let mut stdin = child.stdin.unwrap();

        let mut stdin = child
            .stdin
            .take()
            .expect("child did not have a handle to stdin");

        //let mut tok_stdin = io::stdin(stdin);

        let hdl = tokio::spawn(async move {
            loop {
                if let Some(line) = stdin_rx.recv().await {
                    warn!("Got a line - sending to stdin - {}", line);
                    stdin
                        .write(format!("{}\n", line).as_bytes())
                        .await
                        .expect("could not write to stdin");

                    // if let Err(e) = stdin.write_all(format!("{}\n", line).as_bytes()) {
                    //     error!("Error writing to stdin: {}", e);
                    // }
                } else {
                    warn!("Error receiving line - stdin_rx closed");
                    break;
                }
            }
        });
        self.handlers.push(hdl);

        warn!("Total handlers = {}", self.handlers.len());

        let sender = self.sender.clone();

        let hdl = tokio::spawn(async move {
            // Wait for the child process to exit

            let _ = child.wait().await;
            info!("Child process exited");
            shutdown(sender);
            ctrl_tx.send("exit".to_string()).unwrap();
        });

        self.handlers.push(hdl);

        self.stdin_sender = Some(stdin_tx);
    }
}

fn shutdown(sender: mpsc::Sender<Message>) {
    tokio::spawn(async move {
        if sender.is_closed() {
            info!("Sender is already closed");
        } else {
            info!("Sender is open");
            sender
                .clone()
                .send(Message::Standard {
                    message: serde_json::to_vec(&json!({"type":"exit"})).unwrap(),
                    origin: None,
                })
                .await
                .unwrap();
        }
    });
}

async fn batching(
    mut rx: FuturesMPSC::Receiver<String>,
    forward_to: &mut FuturesMPSC::Sender<Message>,
) {
    // Process lines with batching
    let mut buffer = Vec::new();
    let mut last_send = Instant::now();
    let batch_interval = Duration::from_millis(100);

    //let mut block_wait = true;

    // while let Some(line) = rx.next().await {
    //     info!("Got a line - sending to buffer - {}", line);
    //     buffer.push(line);
    //     process_batch(&mut buffer, forward_to).await;
    // }
    let mut block_wait = false;
    loop {
        // Check for new lines without blocking
        if block_wait {
            info!("Block waiting for new output..");

            match rx.next().await {
                Some(line) => {
                    info!("Got a line - sending to buffer - {}", line);
                    buffer.push(line);
                    block_wait = false;
                }
                None => {
                    // Channel is closed, process any remaining lines and exit
                    process_batch(&mut buffer, forward_to).await;
                    info!("Exiting batch processing loop");
                    break;
                }
            }
        } else {
            match rx.try_next() {
                Ok(Some(line)) => buffer.push(line),
                Ok(None) => {
                    // No lines available, but continue to check time
                    warn!("No lines available");
                    //block_wait = true;
                }
                // Err(TryRecvError::Empty) => {
                //     // No lines available, but continue to check time
                //     // println!("No lines available");
                // }
                // Err(TryRecvError::Disconnected) => {
                //     // Channel is closed, process any remaining lines and exit
                //     process_batch(&mut buffer, forward_to).await;
                //     break;
                // }
                Err(err) => {
                    warn!("Warn receiving line {}", err);
                    //block_wait = true;
                } // Err(TryRecvError::) => {
                  //     // No lines available, but continue to check time
                  // }
                  // Err(TryRecvError::Disconnected) => {
                  //     // Channel is closed, process any remaining lines and exit
                  //     process_batch(&mut buffer, &forward_to).await;
                  //     break;
                  // }
            }

            let now = Instant::now();
            if now.duration_since(last_send) >= batch_interval {
                // Process the batch even if no new lines were added
                process_batch(&mut buffer, forward_to).await;
                // process_batch(&mut buffer, respond_to);
                last_send = now;
            }

            // Small sleep to avoid busy-waiting
            thread::sleep(Duration::from_millis(100));
        }
    }
}

async fn process_batch(buffer: &mut Vec<String>, forward_to: &mut FuturesMPSC::Sender<Message>) {
    if !buffer.is_empty() {
        info!("Processing batch of {} lines", buffer.len());

        // for line in buffer.iter() {
        //     println!("{}", line);
        // }
        forward_to
            .send(Message::Standard {
                message: buffer.join("\n").to_string().into_bytes(),
                origin: None,
            })
            .await
            .unwrap();

        buffer.clear();
    } else {
        warn!("Processing empty batch (100ms elapsed with no new data)");
    }
}
