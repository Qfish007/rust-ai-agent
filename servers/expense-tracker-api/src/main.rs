mod auth;
mod error;
mod handlers;
mod models;
mod store;

use axum::Router;
use axum::middleware;
use axum::routing::get;
use tower_http::trace::DefaultOnRequest;
use tower_http::trace::DefaultOnResponse;
use tower_http::trace::TraceLayer;
use tracing::Level;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let db = store::seed_db();

    // Everything under /expenses requires the API key.
    // /health stays open so you can check the process is alive without auth.
    let expenses_routes = Router::new()
        .route(
            "/",
            get(handlers::list_expenses)
            .post(handlers::create_expense),
        )
        .route(
            "/{id}",
            get(handlers::get_expense)
                .put(handlers::update_expense)
                .delete(handlers::delete_expense),
        )
        .route("/summary", get(handlers::summary))
        .route_layer(middleware::from_fn(auth::require_api_key));

    let app = Router::new()
        .route("/health", get(|| async { "ok" }))
        .nest("/expenses", expenses_routes)
        .with_state(db)
        .layer(TraceLayer::new_for_http()
            .on_request(DefaultOnRequest::new().level(Level::INFO))
            .on_response(DefaultOnResponse::new().level(Level::INFO))); // RUST_LOG=tower_http=debug cargo run

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("failed to bind port 3000");

    tracing::info!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.expect("server error");
}
