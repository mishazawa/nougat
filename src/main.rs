mod server;

use activitypub_federation::config::{FederationConfig, FederationMiddleware};
use std::{
    net::ToSocketAddrs,
    sync::{Arc, Mutex},
};

use axum::{
    Router,
    routing::{get, post},
};
use nougat::utils::config;

use nougat::{database::Database, objects::User, utils::Error};

const BIND_ADDRESS: &str = "localhost:5000";
#[tokio::main]
async fn main() -> Result<(), Error> {
    let config = config();

    let local_user = User::new(&config.hostname, &config.handle_name)?;

    let database = Arc::new(Database {
        users: Mutex::new(vec![local_user]),
    });

    let data = FederationConfig::builder()
        .domain(&config.hostname)
        .app_data(database)
        .build()
        .await?;

    let app = Router::new()
        .route("/user/:name", get(server::http_get_user))
        // .route("/:user/inbox", post(http_post_user_inbox))
        .route("/.well-known/webfinger", get(server::webfinger))
        .layer(FederationMiddleware::new(data.clone()));

    let addr = BIND_ADDRESS
        .to_socket_addrs()?
        .next()
        .expect("Failed to lookup domain name");

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}
