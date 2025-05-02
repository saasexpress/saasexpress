use cel_interpreter::objects::Key::String;
use cel_interpreter::Value;
use serde_json::Value as JsonValue;
use tracing::warn;

pub fn cel_value_to_json(value: &Value) -> JsonValue {
    match value {
        Value::Bool(b) => JsonValue::Bool(*b),
        Value::Int(i) => JsonValue::Number((*i).into()),
        Value::UInt(u) => JsonValue::Number((*u).into()),
        Value::String(s) => JsonValue::String(s.to_string()),
        // Value::Bytes(b) => {
        //     // Convert bytes to base64 string for JSON
        //     let base64 = base64::encode(b);
        //     JsonValue::String(base64)
        // }
        Value::List(items) => {
            let json_items = items.iter().map(|item| cel_value_to_json(item)).collect();
            JsonValue::Array(json_items)
        }
        Value::Map(map) => {
            let mut json_map = serde_json::Map::new();

            for (k, v) in map.map.iter() {
                if let String(key) = k {
                    json_map.insert(key.to_string(), cel_value_to_json(v));
                } else {
                    // JSON only supports string keys
                    let key_str = format!("{:?}", k);
                    json_map.insert(key_str, cel_value_to_json(v));
                }
            }
            JsonValue::Object(json_map)
        }
        Value::Null => JsonValue::Null,
        // Handle other CEL value types appropriately
        _ => {
            warn!("Unsupported CEL value type: {:?}", value);
            JsonValue::Null
        }
    }
}
