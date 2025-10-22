//! Http Server
use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    sync::Arc,
};

use axum::{Extension, Router};
use colored::Colorize;
use database::Mdb;
use inject::InjectProvider;
use log::info;

use crate::config::AppConfig;
use crate::router::axum_router;

/// Http 服务
pub struct HttpServer {}

impl HttpServer {
    /// 服务
    pub async fn run(
        app_config: AppConfig,
        _db_pool: Mdb,
        inject_provider: Arc<InjectProvider>,
    ) -> anyhow::Result<()> {
        // Build our application by creating our router.
        let app = Router::new()
            .nest("/api/v1", axum_router::register()) // API 服务
            .fallback(axum_router::fallback) // 用于处理与路由器路由不匹配的任何请求
            .layer(Extension(app_config)) // 全局配置文件
            .layer(Extension(inject_provider)); // 依赖注入

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
