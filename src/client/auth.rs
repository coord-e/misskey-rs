use crate::api::ApiRequest;
use crate::error::Result;

use serde_json::value::{self, Value};

pub fn to_json_with_api_key<T: ApiRequest>(data: T, api_key: &str) -> Result<Value> {
    let mut value = value::to_value(data)?;

    let obj = value.as_object_mut().expect("ApiRequest must be an object");
    if obj
        .insert("i".to_string(), Value::String(api_key.to_string()))
        .is_some()
    {
        panic!("ApiRequest must not have 'i' key");
    }

    Ok(value)
}
