use axum::{Router, routing::get};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "rust_api=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();

    tracing::debug!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app()).await.unwrap();
}

fn app() -> Router {
    Router::new()
        .route("/", get(home))
        .layer(TraceLayer::new_for_http())
}

async fn home() -> &'static str {
    "Hello, World!"
}
