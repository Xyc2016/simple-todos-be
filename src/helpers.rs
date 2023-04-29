use serde_json::{json, Value};

pub fn success_ret(data: Value) -> Value {
    json!({
        "msg": "ok",
        "code": 0,
        "data": data,
    })
}

pub fn fail_ret(msg: String, code: Option<i32>) -> Value {
    json!({
        "msg": msg,
        "code": code.unwrap_or(-1),
        "data": null,
    })
}
