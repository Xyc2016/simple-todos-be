use crate::types::BizResp;
use rocket::serde::json::Json;
use serde_json::{json, Value};

pub fn success_ret<T>(data: T) -> Json<BizResp<T>> {
    Json(BizResp {
        code: 0,
        msg: "ok".into(),
        data: Some(data),
    })
}

pub fn failure_ret<T>(msg: String, code: Option<i32>) -> Json<BizResp<T>> {
    Json(BizResp {
        code: code.unwrap_or(-1),
        msg,
        data: None,
    })
}
