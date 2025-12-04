//! 工具函数
use url::Url;

use err_code::Error;

/// 提取URL参数
pub fn extract_url_params(url_str: &str, param: &str) -> Result<Option<String>, Error> {
    let url = Url::parse(url_str)?;

    let query_pairs = url.query_pairs();

    for (key, value) in query_pairs {
        if key == param {
            return Ok(Some(value.to_string()));
        }
    }

    Ok(None)
}
