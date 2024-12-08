use crate::api::handler::day1::{headers, hello_world};
use crate::api::handler::day2::{
    calculate_ip_add, calculate_ip_sub, calculate_ipv6_add, calculate_ipv6_sub,
};
use crate::api::handler::order::manifest;
use axum::routing::post;
use axum::{routing::get, Router};

mod api;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/-1/seek", get(headers))
        .route("/2/dest", get(calculate_ip_add))
        .route("/2/key", get(calculate_ip_sub))
        .route("/2/v6/dest", get(calculate_ipv6_add))
        .route("/2/v6/key", get(calculate_ipv6_sub))
        .route("/5/manifest", post(manifest));

    Ok(router.into())
}
