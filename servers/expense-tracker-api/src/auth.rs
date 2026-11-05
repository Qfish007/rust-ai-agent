use axum::extract::Request;
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::Response;

/// Header clients must send: `x-api-key: dev-secret-key`
const API_KEY_HEADER: &str = "x-api-key";

/// Hardcoded on purpose for the tutorial. In a real service this would
/// come from an environment variable / secrets manager, and you'd likely
/// support multiple keys (one per caller) instead of a single shared one.
const EXPECTED_API_KEY: &str = "dev-secret-key";

/// Runs before every route it's attached to. No key, or the wrong key,
/// short-circuits the request with 401 before the handler ever runs.
pub async fn require_api_key(req: Request, next: Next) -> Result<Response, StatusCode> {
    let provided = req
        .headers()
        .get(API_KEY_HEADER)
        .and_then(|value| value.to_str().ok());

    match provided {
        Some(key) if key == EXPECTED_API_KEY => Ok(next.run(req).await),
        _ => Err(StatusCode::UNAUTHORIZED),
    }
}
