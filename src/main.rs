use std::{collections::HashMap, env, sync::Arc};

use axum::{
    Router,
    routing::{get, post},
};
use tokio::sync::Mutex;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

use crate::http::{generate_resource_pack::resource_pack_endpoint, get_resource_pack::get_resource_pack_endpoint};

pub mod bingo_board_image;
pub mod resource_pack;

pub mod http;

#[derive(Default)]
pub struct ResourcePackState {
    pub resource_packs: Mutex<HashMap<String, Vec<u8>>>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::DEBUG.into())
                .from_env_lossy(),
        )
        .init();

    let app = Router::new()
        .route("/generate", post(resource_pack_endpoint))
        .route("/resource-pack/{hash}", get(get_resource_pack_endpoint))
        .with_state(Arc::new(ResourcePackState::default()));

    let listener = tokio::net::TcpListener::bind(format!(
        "0.0.0.0:{}",
        env::var("LISTEN_PORT").unwrap_or("3000".to_string())
    ))
    .await
    .unwrap();

    axum::serve(listener, app).await.unwrap();
}
