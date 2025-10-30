//! 错误处理
#![allow(unused)]
use axum::{
    body::Body,
    http::{Response, StatusCode},
};
use log::error;

use axum_response::ResponseErr;
use err_code::{Error, ErrorMsg};

pub(crate) fn create_error_response(err: ErrorMsg) -> Response<Body> {
    let err: ResponseErr = err.into();
    let data = serde_json::to_string(&err)
        .map_err(|err| {
            error!("转换为JSON字符串失败, error: {err:#?}");
            Error::JsonSerialization(err.to_string())
        })
        .unwrap_or_default();

    let mut resp = Response::new(Body::from(data));
    *resp.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;

    resp
}
