use axum::routing::get;
use axum::{AddExtensionLayer, Json, Router};
use serde::Serialize;

use warden::config::Config;
use warden::state::State;

#[tokio::main]
async fn main() {
    let config = Config::new().expect("Failed to load values from config sources.");
    println!("{:#?}", config);

    let config = State::new(config);

    let app = Router::new()
        .route("/health", get(health_check))
        .layer(AddExtensionLayer::new(config.clone()));

    println!("Starting server. Listening on {}", config.bind_address());
    axum::Server::bind(&config.bind_address())
        .serve(app.into_make_service())
        .await
        .expect("Server failed to start.");
}

#[derive(Serialize)]
pub struct HealthStatus {
    pub message: String,
}

pub async fn health_check() -> Json<HealthStatus> {
    Json::from(HealthStatus {
        message: "healthy".to_string(),
    })
}
