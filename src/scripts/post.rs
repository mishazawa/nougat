use std::sync::{Arc, Mutex};

use activitypub_federation::{config::FederationConfig, fetch::webfinger::webfinger_resolve_actor};
use nougat::{database::Database, objects::User, utils::config};
use tracing::log::LevelFilter;
#[tokio::main]
async fn main() {
    env_logger::builder()
        .filter_level(LevelFilter::Warn)
        .filter_module("activitypub_federation", LevelFilter::Debug)
        .format_timestamp(None)
        .init();

    let c = config();
    let local_user = User::new(&c.hostname, &c.handle_name).expect("User created");

    let database = Arc::new(Database {
        users: Mutex::new(vec![local_user]),
    });

    let config = FederationConfig::builder()
        .domain("mastodon.social")
        .app_data(database)
        .build()
        .await
        .expect("Config created");

    let data = config.to_request_data();
    let user: User = webfinger_resolve_actor("nougat69@mastodon.social", &data)
        .await
        .expect("retrieved user data");

    println!("{:?}", user);
}
