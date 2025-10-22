//! Http Server
use std::{
    collections::HashMap,
    net::{IpAddr, Ipv4Addr, SocketAddr},
};

use axum::{Json, Router, extract::Query, response::IntoResponse, routing::get};
use axum_streams::StreamBodyAs;
use colored::Colorize;
use futures::{
    Stream,
    stream::{self},
};
use log::{error, info};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use tokio_stream::StreamExt;

use crate::middleware::cors_layer;

#[derive(Debug, Clone, Deserialize, Serialize)]
struct MyTestStructure {
    some_test_field: String,
}

fn source_test_stream() -> impl Stream<Item = MyTestStructure> {
    // Simulating a stream with a plain vector and throttling to show how it works
    stream::iter(vec![
        MyTestStructure {
            some_test_field: "test1".to_string()
        };
        100
    ])
    .throttle(std::time::Duration::from_secs(1))
}

async fn test_json_array_stream() -> impl IntoResponse {
    StreamBodyAs::json_array(source_test_stream())
}

async fn test_json_nl_stream() -> impl IntoResponse {
    StreamBodyAs::json_nl(source_test_stream())
}

fn source_text_stream() -> impl Stream<Item = String> {
    // Simulating a stream with a plain vector and throttling to show how it works
    stream::iter(vec![
        "苟利国家生死以，岂因祸福避趋之？".to_string();
        1000
    ])
    .throttle(std::time::Duration::from_secs(1))
}

async fn test_text_stream() -> impl IntoResponse {
    StreamBodyAs::text(source_text_stream())
}

async fn hello(Query(params): Query<HashMap<String, String>>) -> Json<Value> {
    error!("================={:#?}", params);
    Json(json!({"code":0, "data" : format!("hi, {}", params.get("name").map_or("noi", |v| v))}))
}

/// Http 服务
pub struct HttpServer {}

impl HttpServer {
    /// 服务
    pub async fn run() -> anyhow::Result<()> {
        // 创建路由
        let app = Router::new()
            .route("/test_json_array_stream", get(test_json_array_stream))
            .route("/test_json_nl_stream", get(test_json_nl_stream))
            .route("/test_text_stream", get(test_text_stream))
            .route("/health", get(|| async { "ok" }))
            .route("/hello", get(hello))
            .layer(cors_layer());

        // 启动服务器
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 3000);
        let listener = tokio::net::TcpListener::bind(addr).await?;

        info!(
            "listening on {}",
            listener.local_addr()?.to_string().yellow()
        );
        // Run the server
        axum::serve(listener, app).await?;

        Ok(())
    }
}
