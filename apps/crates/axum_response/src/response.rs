//! 接口响应类型

use axum::{Json, response::IntoResponse};
use serde::{Deserialize, Serialize};

use crate::ResponseErr;

/// 响应体封装
pub type Responder<T> = Result<Response<T>, ResponseErr>;

/// 响应结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response<T: Serialize> {
    /// 返回业务码
    code: u16,
    /// 返回信息
    msg: String,

    /// 返回数据
    data: Option<T>,
    /// 数据总数
    #[serde(skip_serializing_if = "Option::is_none")]
    total: Option<usize>,
}

impl<T: Serialize> Default for Response<T> {
    fn default() -> Self {
        Self {
            code: 0,
            msg: "ok".to_string(),
            data: None,
            total: None,
        }
    }
}

impl<T: Serialize> Response<T> {
    /// 返回成功
    pub fn ok() -> Self {
        Self::default()
    }

    /// 设置返回的数据
    pub fn data(data: T) -> Self {
        Self {
            data: Some(data),
            total: None,
            ..Default::default()
        }
    }

    ///  Set return msg
    pub fn with_msg(mut self, msg: &str) -> Self {
        self.msg = msg.to_string();
        self
    }

    /// Add msg information and add new information based on the error code information
    pub fn append_msg(mut self, msg: &str) -> Self {
        self.msg = format!("{}, {}", self.msg, msg);
        self
    }

    /// 获取当前状态
    pub fn status(&self) -> u16 {
        self.code
    }

    /// 返回 JSON 结构
    pub fn to_json<J>(&self) -> Result<J, serde_json::Error>
    where
        J: for<'de> Deserialize<'de> + Serialize,
    {
        let data = serde_json::to_value(self)?;
        let target: J = serde_json::from_value(data)?;
        Ok(target)
    }
}

impl<T: Serialize> Response<Vec<T>> {
    /// 设置返回的数据列表
    pub fn data_list<U>(data: Vec<T>, total: U) -> Self
    where
        U: TryInto<usize> + TryInto<u64>,
    {
        let total_usize = total.try_into().unwrap_or_default(); // 转换为 usize
        Self {
            data: Some(data),
            total: Some(total_usize),
            ..Default::default()
        }
    }
}

/// 打印 Response
impl<T: Serialize> std::fmt::Display for Response<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(Response(code: {}, msg: {}))", self.code, self.msg)
    }
}

impl<T: Serialize + std::fmt::Debug> std::error::Error for Response<T> {}

/// Axum 响应体实现
impl<T: Serialize> IntoResponse for Response<T> {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    struct Book {
        title: String,
        desc: String,
    }

    #[test]
    fn test_ok() {
        let resp = Response::<()>::ok().with_msg("ok");
        println!("resp: {:#?}", resp);

        let resp_str =
            serde_json::to_string_pretty(&resp).expect("Failed to serialize response to JSON");
        println!("resp str: {}", resp_str);
    }

    #[test]
    fn test_data() {
        let book = Book {
            title: "weather".to_string(),
            desc: "This is a book about weather".to_string(),
        };

        let resp = Response::data(book);

        println!("{:#?}", resp);

        assert!(resp.status() == 0);
    }

    #[test]
    fn test_data_list() {
        let books = vec![Book {
            title: "Douqi Continent".to_string(),
            desc: "This is a fantasy novel".to_string(),
        }];
        let total = books.len();

        let resp = Response::data_list(books, total).with_msg("operation successful");
        println!("resp: {:#?}", resp);

        let resp_str =
            serde_json::to_string_pretty(&resp).expect("Failed to serialize response to JSON");
        println!("resp str: {}", resp_str);
    }

    #[test]
    fn test_data_to_json() -> anyhow::Result<()> {
        let book = Book {
            title: "weather".to_string(),
            desc: "This is a book about weather".to_string(),
        };

        #[derive(Debug, Serialize, Deserialize)]
        pub struct TemplateResp {
            data: Book,
        }

        let resp = Response::data(book)
            .with_msg("operation successful")
            .to_json::<TemplateResp>()?;
        println!("resp: {:#?}", resp);

        let resp_str =
            serde_json::to_string_pretty(&resp).expect("Failed to serialize response to JSON");
        println!("resp str: {}", resp_str);

        Ok(())
    }

    #[test]
    fn test_data_list_to_json() -> anyhow::Result<()> {
        let books = vec![Book {
            title: "Douqi Continent".to_string(),
            desc: "This is a fantasy novel".to_string(),
        }];

        #[derive(Debug, Serialize, Deserialize)]
        struct TemplatesResp {
            code: u16,
            msg: String,
            pub data: Vec<Book>,
            pub total: u64,
        }

        let total = books.len();

        let resp = Response::data_list(books, total)
            .with_msg("operation successful")
            .to_json::<TemplatesResp>()?;
        println!("resp: {:#?}", resp);

        let resp_str =
            serde_json::to_string_pretty(&resp).expect("Failed to serialize response to JSON");
        println!("resp str: {}", resp_str);

        Ok(())
    }

    #[test]
    fn test_into_response() {
        let book = Book {
            title: "weather".to_string(),
            desc: "This is a book about weather".to_string(),
        };

        let resp = Response::data(book)
            .with_msg("operation successful")
            .into_response();
        println!("resp: {:#?}", resp);
    }

    #[test]
    fn test_to_string() {
        let book = Book {
            title: "weather".to_string(),
            desc: "This is a book about weather".to_string(),
        };

        let resp = Response::data(book)
            .with_msg("operation successful")
            .to_string();
        println!("resp: {:#?}", resp);
    }
}
