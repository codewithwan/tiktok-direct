use serde_json::Value;

pub fn fill(target: &mut Option<String>, value: Option<String>) {
    if target.is_none() {
        *target = value;
    }
}

pub fn string_field(value: Option<&Value>, key: &str) -> Option<String> {
    value?.get(key).and_then(|v| match v {
        Value::String(text) => Some(text.clone()),
        Value::Number(number) => Some(number.to_string()),
        _ => None,
    })
}

pub fn u64_field(value: &Value, key: &str) -> Option<u64> {
    value.get(key).and_then(|v| match v {
        Value::Number(number) => number.as_u64(),
        Value::String(text) => text.parse().ok(),
        _ => None,
    })
}
