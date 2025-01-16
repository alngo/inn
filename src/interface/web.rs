use std::sync::Arc;

use anyhow::Context;
use axum::{routing::get, Router};
use tokio::net;

use crate::application::inn::owner::OwnerRepository;

use super::{
    inn::{
        owner::present::cli,
        service::{InnService, Service},
    },
    shared::Present,
};

mod handlers;
mod responses;

struct AppState<IS: InnService<P: Present>> {
    service: Arc<IS>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WebServerConfig<'a> {
    pub port: &'a str,
}

pub struct WebServer {
    router: axum::Router,
    listener: net::TcpListener,
}

impl WebServer {
    pub async fn new<D: OwnerRepository + Sync + Send + 'static>(
        database: Arc<D>,
        config: WebServerConfig<'_>,
    ) -> anyhow::Result<Self> {
        let service = Service::new(database, cli::Presenter);
        let router = Router::new()
            .route("/api", get(|| async { "Hello, World!" }))
            .with_state(service);
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

fn api_route() -> Router<Service<D, P>> {
    Router::new().route("/owners", post(create_author::<BS>))
}
