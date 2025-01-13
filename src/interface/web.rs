use anyhow::Context;
use axum::{routing::get, Router};
use tokio::net;

mod handlers;
mod responses;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WebServerConfig<'a> {
    pub port: &'a str,
}

pub struct WebServer {
    router: axum::Router,
    listener: net::TcpListener,
}

impl WebServer {
    pub async fn new(config: WebServerConfig<'_>) -> anyhow::Result<Self> {
        let router = Router::new().route("/api", get(|| async { "Hello, World!" }));
        let listener = net::TcpListener::bind(format!("0.0.0.0:{}", config.port))
            .await
            .with_context(|| format!("Failed to bind to port {}", config.port))
            .unwrap();
        Ok(Self { router, listener })
    }

    pub async fn run(self) -> anyhow::Result<()> {
        axum::serve(self.listener, self.router)
            .await
            .with_context(|| "Failed to run server")?;
        Ok(())
    }
}
