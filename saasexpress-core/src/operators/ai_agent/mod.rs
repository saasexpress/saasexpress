use crate::graph::operator::{OperatorRuntime, OperatorRuntimeType, OperatorType};
use crate::graph::operator_types::ai_agent::AIAgentOperator;
use crate::graph::operator_types::ai_tool::AIToolOperator;
use crate::graph::{
    message::{Message, OriginMessage},
    operator::{OperatorRef, OperatorRole},
};
use futures::channel::oneshot;
use futures::channel::oneshot::Canceled;
use model::AiAgentModel;
use serde::Deserialize;
use serde_json::{Value, json};
use std::{collections::HashMap, sync::Arc};
use tracing::error;
use tracing::info;

//use AiAgentModel;
mod model;

fn default_empty() -> String {
    "".to_string()
}

#[derive(Deserialize, Debug)]
//#[serde(deny_unknown_fields)]
struct ThisModel {
    #[serde(default = "default_empty")]
    name: String,
}

#[derive(Debug)]
pub(super) struct AIAgentV1;

impl From<serde_yaml::Value> for AIAgentV1 {
    fn from(_value: serde_yaml::Value) -> Self {
        AIAgentV1 {}
    }
}
impl AIAgentOperator for AIAgentV1 {
    fn process(
        &self,
        origin: Option<OriginMessage>,
        user_prompt: String,
        next: Vec<OperatorRole>,
        tools: HashMap<String, OperatorRuntimeType>,
    ) {
        info!("AIAgentV1 ChatGPT process");

        let tools = tools.clone();

        tokio::spawn(async move {
            // (1) if existing conversation, retrieve it
            //let storage = callout(next.clone(), "storage".to_string(), json!({})).await;
            // storage will be added to the conversation history

            // (2) gather up any special prompts
            let prompts_result = callout(next.clone(), "prompt".to_string(), json!({})).await;

            // (4) wait for response
            if prompts_result.is_err() {
                error!("Error getting Prompts: {:?}", prompts_result.err());
                return;
            }
            let prompts_result = prompts_result.unwrap();

            let system_prompt = match prompts_result {
                Message::JSON { message, .. } => message,
                Message::Standard { message, .. } => serde_json::from_slice(&message).unwrap(),
                _ => {
                    error!("Unexpected Prompts result type: {:?}", prompts_result);
                    return;
                }
            };
            let system_prompt = system_prompt
                .get("content")
                .unwrap()
                .as_str()
                .unwrap()
                .to_string();

            let mut response = json!({});

            let mut history = Vec::new();

            let user_prompt = user_prompt.clone();

            loop {
                // (2) prepare the llm message which includes the tool schemas()
                let llm_request = prepare_llm_request(
                    system_prompt.clone(),
                    history.clone(),
                    user_prompt.clone(),
                    tools.clone(),
                )
                .await;

                info!("{}", serde_yaml::to_string(&llm_request).unwrap());
                // (3) send to operator (role = llm)
                let llm_result = callout(next.clone(), "llm".to_string(), json!(llm_request)).await;

                // (4) wait for response
                if llm_result.is_err() {
                    error!("Error calling LLM: {:?}", llm_result.err());
                    return;
                }
                let llm_result = llm_result.unwrap();

                let message = match llm_result {
                    Message::JSON { message, .. } => message,
                    Message::Standard { message, .. } => serde_json::from_slice(&message).unwrap(),
                    _ => {
                        error!("Unexpected LLM result type: {:?}", llm_result);
                        return;
                    }
                };

                info!("LLM result: {}", serde_yaml::to_string(&message).unwrap());

                // (5) determine the next function (role = tool) and call it
                //let a = next_move(_message, HashMap::new()).await;
                //aa().await;
                let ai_model: AiAgentModel = serde_json::from_value(message.clone()).unwrap();

                if ai_model.choices[0].finish_reason == "tool_calls" {
                    let cho = ai_model.choices.get(0).unwrap();
                    let tool_calls = &cho.message.tool_calls;

                    for tool_call in tool_calls.into_iter() {
                        info!("Tool call: {:?}", tool_call);
                        let func_name = &tool_call.function.name;
                        let func_args = &tool_call.function.arguments;
                        let func_args = serde_json::from_str(func_args)
                            .expect("Failed to parse function arguments");

                        let tool_result = callout_tool(func_name, func_args, tools.clone()).await;
                        if tool_result.is_err() {
                            error!("Error calling Tool: {:?}", tool_result.err());
                            return;
                        }
                        let tool_result = tool_result.unwrap();

                        response = match tool_result {
                            Message::JSON { message, .. } => message,
                            Message::Standard { message, .. } => {
                                serde_json::from_slice(&message).unwrap()
                            }
                            Message::Tuple { message_2, .. } => match *message_2 {
                                Message::JSON { message, .. } => message,
                                Message::Standard { message, .. } => {
                                    serde_json::from_slice(&message).unwrap()
                                }
                                _ => {
                                    error!("Unexpected Tool result type");
                                    return;
                                }
                            },
                            _ => {
                                error!("Unexpected Tool result type: {:?}", tool_result);
                                return;
                            }
                        };
                    }

                    let content = serde_json::to_value(&response.get("response").unwrap()).unwrap();

                    let cho = ai_model.choices.get(0).unwrap();

                    history.push(serde_json::to_value(&cho.message).unwrap());

                    let tool_calls = &cho.message.tool_calls;

                    for tool_call in tool_calls.into_iter() {
                        history.push(json!({"role": "tool", "tool_call_id": tool_call.id, "content": content }));
                    }

                //                      # append model's function call message
                // messages.append({                               # append result message
                //     "role": "tool",
                //     "tool_call_id": tool_call.id,
                //     "content": str(result)
                // })

                //user_prompt = response
                } else {
                    info!("I think finished");
                    let cho = ai_model.choices.get(0).unwrap();
                    let model_message = &cho.message;
                    response = json!({"response": &model_message.content});
                    break;
                }
            }
            // (6) pass conversation to operator (role = storage)

            // (7) return response
            let role = OperatorRole::default();
            let next = next
                .iter()
                .filter(|o| o.role == role)
                .next()
                .map(|n| n.operator.clone())
                .unwrap();

            next_send(
                Message::Standard {
                    message: serde_json::to_vec(&response).unwrap(),
                    origin,
                },
                next,
            )
            .await;
        });
    }
}

async fn next_send(message: Message, next: Arc<dyn OperatorRuntime + 'static>) {
    next.send(message);
}

async fn prepare_llm_request(
    system_prompt: String,
    mut history: Vec<Value>,
    user_prompt: String,
    tools: HashMap<String, OperatorRuntimeType>,
) -> Value {
    let tool_schemas = tools
        .iter()
        .map(|(_name, tool)| {
            let tool = tool._type();
            let tool = match tool {
                OperatorType::AITool { tool } => tool,
                _ => panic!("Invalid operator type"),
            };
            let schema = tool.get_schema().unwrap();
            json!({
                "type": "function",
                "function": schema
            })
        })
        .collect::<Vec<_>>();

    let mut messages = Vec::new();
    messages.append(
        json!([
        {
            "role": "system",
            "content": system_prompt
        }])
        .as_array_mut()
        .unwrap(),
    );
    messages.append(
        json!([
        {
            "role": "user",
            "content": user_prompt
        }])
        .as_array_mut()
        .unwrap(),
    );
    messages.append(&mut history);

    json!({
      "messages": messages,
      "model": "gpt-4.1",
      "tools": tool_schemas,
      "tool_choice": "auto"
    })
}

fn do_callout(message: Value, next: &OperatorRuntimeType) -> oneshot::Receiver<Message> {
    let (tx, rx) = oneshot::channel::<Message>();

    info!("Sending message to next operator: {:?}", message);

    let message = Message::Standard {
        message: serde_json::to_vec(&message).unwrap(),
        origin: Some(OriginMessage::new(Some(tx))),
    };

    next.send(message);

    rx
}

async fn callout(
    next_list: Vec<OperatorRole>,
    role: String,
    json: Value,
) -> Result<Message, Canceled> {
    let next = next_list
        .iter()
        .filter(|o| o.role == role)
        .next()
        .map(|n| n.operator.clone());

    if next.is_none() {
        error!("No matching operator found for role {}", role);
        return Err(Canceled);
    }
    let next = next.unwrap().clone();

    let llm_result = do_callout(json, &next).await;
    llm_result
}

async fn callout_tool(
    name: &str,
    json: Value,
    tools: HashMap<String, OperatorRuntimeType>,
) -> Result<Message, Canceled> {
    let next = tools
        .iter()
        .filter(|o| o.0.to_string() == name)
        .next()
        .map(|n| n.clone());

    if next.is_none() {
        error!("No matching operator found for role {}", name);
        return Err(Canceled);
    }
    let next = next.unwrap().clone().1;

    do_callout(json, next).await
}
