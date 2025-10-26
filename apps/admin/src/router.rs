//! 路由

use std::time::Duration;

use axum::{Router, extract::DefaultBodyLimit};
use log::warn;
use tokio::signal;
use tower::ServiceBuilder;
use tower_http::{
    ServiceBuilderExt,
    compression::CompressionLayer,
    limit::RequestBodyLimitLayer,
    request_id::MakeRequestUuid,
    timeout::TimeoutLayer,
    trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer},
};

use axum_context::ContextLayer;
use axum_middleware::{cors::cors_layer, empty_wrapper_fn::empty_wrapper_layer};
use service_hub::{
    auth::AuthRouter, debox::DeboxRouter, log::LogRouter, system::SystemRouter, user::UserRouter,
};

/// axum handler for any request that fails to match the router routes.
/// This implementation returns HTTP status code Not Found (404).
pub async fn fallback(uri: axum::http::Uri) -> impl axum::response::IntoResponse {
    warn!("No route {}", uri);
    (
        axum::http::StatusCode::NOT_FOUND,
        format!("No route {}", uri),
    )
}

/// 优雅关机
pub async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}

/// 注册路由
pub fn register() -> Router {
    let my_layers = ServiceBuilder::new()
        .layer(ContextLayer::new()) // 上下文
        .layer(axum::middleware::from_fn(empty_wrapper_layer)); // 空包装

    // 注意中间件加载顺序: Last in, first loading
    let layers = ServiceBuilder::new()
        // make sure to set request ids before the request reaches `TraceLayer`
        .set_x_request_id(MakeRequestUuid)
        // .layer(HandleErrorLayer::new(handle_error)) // 自定义错误类型需要添加该中间件
        .layer(
            // set request_id log requests and responses
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().include_headers(true))
                .on_response(DefaultOnResponse::new().include_headers(true)),
        ) // 高级跟踪/记录
        .layer(cors_layer()) // 为CORS添加标头的中间件
        .layer(CompressionLayer::new()) // 自动压缩响应
        .layer(TimeoutLayer::new(Duration::from_secs(30))) // Timeout requests after 30 seconds
        .layer(my_layers)
        .layer(DefaultBodyLimit::disable()) // Disable the default limit
        .layer(RequestBodyLimitLayer::new(250 * 1024 * 1024)) //250mb, 限制了传入请求的大小，防止试图通过大量请求压垮服务器的攻击
        // propagate the header to the response before the response reaches `TraceLayer`
        .propagate_x_request_id();

    Router::new()
        .merge(AuthRouter::register()) // 用户认证
        .merge(UserRouter::register()) // 用户管理
        .merge(DeboxRouter::register()) // Debox管理
        .merge(SystemRouter::register()) // 系统管理
        .merge(LogRouter::register()) // 日志管理
        .layer(layers)
}
