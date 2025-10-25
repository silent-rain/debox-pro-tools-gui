//! 服务
use std::{
    net::{IpAddr, SocketAddr},
    sync::Arc,
};

use axum::{Extension, Router};
use colored::Colorize;
use listenfd::ListenFd;
use tokio::net::TcpListener;

use app_state::mobile::AppState;
use config::AppConfig;
use database::Mdb;
use inject::InjectProvider;

use crate::router;

/// Http 服务
pub struct HttpServer {}

impl HttpServer {
    /// 运行服务
    pub async fn run(
        app_config: AppConfig,
        _db_pool: Mdb,
        inject_provider: Arc<InjectProvider>,
        _app_state: Arc<AppState>,
    ) -> anyhow::Result<()> {
        // Build our application by creating our router.
        let app = Router::new()
            .nest("/api/v1", router::register()) // API 服务
            .fallback(router::fallback) // 用于处理与路由器路由不匹配的任何请求
            .layer(Extension(app_config.clone())) // 全局配置文件
            .layer(Extension(inject_provider)); // 依赖注入

        // Run our application as a hyper server
        let mut listenfd = ListenFd::from_env();
        let listener = match listenfd.take_tcp_listener(0)? {
            // if we are given a tcp listener on listen fd 0, we use that one
            Some(listener) => {
                listener.set_nonblocking(true)?;
                TcpListener::from_std(listener)?
            }
            // otherwise fall back to local listening
            None => {
                let ip_addr: IpAddr = app_config.server.base.address.parse()?;
                let addr = SocketAddr::new(ip_addr, app_config.server.base.port);
                TcpListener::bind(addr).await?
            }
        };

        println!(
            "listening on {}",
            listener.local_addr()?.to_string().yellow()
        );
        // Run the server with graceful shutdown
        axum::serve(
            listener,
            app.into_make_service_with_connect_info::<SocketAddr>(),
        )
        .with_graceful_shutdown(router::shutdown_signal())
        .await?;

        Ok(())
    }
}
