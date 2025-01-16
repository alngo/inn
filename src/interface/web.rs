use std::sync::Arc;

use anyhow::Context;
use axum::{extract::State, routing::get, Router};
use tokio::net;

use crate::application::inn::owner::{create_owner, OwnerRepository};

use super::{
    inn::owner::{self, cli},
    shared::Present,
};

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
    pub async fn new<D: OwnerRepository + Sync + Send + 'static>(
        database: D,
        config: WebServerConfig<'_>,
    ) -> anyhow::Result<Self> {
        let owner_controller = owner::Controller::new(Arc::new(database), cli::Presenter);
        let router = Router::new().nest("/api", owner_routes(owner_controller));
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

fn owner_routes<R, P>(controller: owner::Controller<R, P>) -> Router<()>
where
    R: OwnerRepository + Sync + Send + 'static,
    P: Present<create_owner::Result> + Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/owners", get(|_: State<owner::Controller<R, P>>| async {}))
        .with_state(controller)
}
