use axum::{
    http::StatusCode, routing::post, Json, Router
};

use std::sync::Arc;
use serde::Deserialize;
use tokio::sync::Semaphore;

mod build;

#[derive(Deserialize)]
struct Response  {
    response: String
}

#[derive(Deserialize)]
struct Repository {
    ssh_url: String,
}

#[derive(Deserialize)]
struct GiteaWebhook {
    repository: Repository,
    after: String,
}

#[derive(Clone)]
struct AppState {
    main_semaphore: Arc<Semaphore>
}

#[axum::debug_handler]
async fn webhook_handler(Json(payload): Json<GiteaWebhook>) -> StatusCode { 
    println!("Repository: {}, Commit: {}", payload.repository.ssh_url, payload.after);

    StatusCode::OK
}

#[tokio::main]
async fn main(){
    let main_semaphore = Arc::new(Semaphore::new(4));
    let app_state = AppState { main_semaphore };

    let app = Router::new()
        .route("/whlisten", post(webhook_handler))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:6780").await.unwrap();
    axum::serve(listener, app).await.unwrap_or_default()
}

