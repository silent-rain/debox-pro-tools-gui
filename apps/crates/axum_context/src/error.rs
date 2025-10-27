//! 错误码
#![allow(unused)]
use axum::{
    body::Body,
    http::{Response, StatusCode},
};
use serde::{Serialize, Serializer};
use serde_json::json;
use tracing::error;

#[derive(Debug, PartialEq, thiserror::Error)]
#[repr(u16)]
pub enum Error {
    #[error("internal server error")]
    InternalServer,
}

impl Error {
    /// 返回错误码
    pub fn code(&self) -> u16 {
        unsafe {
            let ptr = self as *const Error as *const u16;
            ptr.read_volatile()
        }
    }
    /// 返回错误码信息
    pub fn msg(&self) -> String {
        self.to_string()
    }
}

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

impl From<Error> for Response<Body> {
    fn from(err: Error) -> Self {
        let err_msg = ErrorMsg {
            code: StatusCode::BAD_REQUEST.into(),
            msg: err.msg(),
        };

        let data = serde_json::to_string(&err_msg).unwrap_or_else(|e| {
            error!("转换为JSON字符串失败, error: {:#?}", e);

            json!({
                "code": Error::InternalServer.code(),
                "msg": Error::InternalServer.msg()
            })
            .to_string()
        });

        let mut resp = Response::new(Body::from(data));
        *resp.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;

        resp
    }
}

#[derive(Debug, Serialize)]
struct ErrorMsg {
    code: u16,
    msg: String,
}
