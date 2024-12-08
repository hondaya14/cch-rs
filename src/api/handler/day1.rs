use axum::http::{header, Response, StatusCode};
use axum::response::IntoResponse;

pub async fn hello_world() -> &'static str {
    "Hello, bird!"
}

pub async fn headers() -> impl IntoResponse {
    let mut response = Response::new("Hello, world!".to_string());
    response.headers_mut().insert(
        header::LOCATION,
        "https://www.youtube.com/watch?v=9Gc4QTqslN4"
            .parse()
            .unwrap(),
    );
    *response.status_mut() = StatusCode::FOUND;
    response
}
