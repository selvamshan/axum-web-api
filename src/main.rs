use axum::Router;
use tracing::info;
use tracing_subscriber::EnvFilter;

mod config;
mod error;
mod utils;
mod web;

pub use self::error::{Error, Result};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .without_time() //For early local development
        .with_target(false)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let app = Router::new()
        .merge(web::routes_hello::routes())
        .fallback(web::routes_static::serve_dir());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();
    info!("{:<12} - {:?}\n", "LISTENING", listener.local_addr());
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
