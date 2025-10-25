//! 跨域中间件

use std::time::Duration;

use axum::http::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE, ORIGIN};
use tower_http::cors::{Any, CorsLayer};

// 跨域中间件包装
pub fn cors_layer() -> CorsLayer {
    CorsLayer::new()
        .allow_headers(vec![ORIGIN, AUTHORIZATION, ACCEPT, CONTENT_TYPE])
        .allow_methods(Any)
        .allow_headers(Any)
        .allow_origin(Any)
        .max_age(Duration::from_secs(60) * 10)
}
