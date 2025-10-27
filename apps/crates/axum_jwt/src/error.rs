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

    #[error("Request header does not contain the Authorization field")]
    HeadersNotAuthorization,
    #[error("Authorization header is missing the Bearer prefix")]
    HeadersNotAuthorizationBearer,

    /// 令牌已经过期
    #[error("Token has expired")]
    CheckExp,
    /// Audience 验证错误
    #[error("Audience verification failed")]
    CheckAud,
    /// Issued At 时间验证错误
    #[error("Issued At time verification failed")]
    CheckIat,
    /// Not Before 时间验证错误
    #[error("Not Before time verification failed")]
    CheckNbf,
    /// Subject 验证错误
    #[error("Subject verification failed")]
    CheckSub,
    /// 发行人验证错误
    #[error("Issuer verification failed")]
    CheckIss,
    /// JWT 错误
    #[error(transparent)]
    JsonWebToken(#[from] jsonwebtoken::errors::Error),
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
            code: StatusCode::UNAUTHORIZED.into(),
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
