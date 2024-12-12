use crate::api::handler::day1::{headers, hello_world};
use crate::api::handler::day2::{
    calculate_ip_add, calculate_ip_sub, calculate_ipv6_add, calculate_ipv6_sub,
};
use crate::api::handler::day9::{post_milk, post_refill};
use crate::api::handler::order::manifest;
use axum::routing::post;
use axum::{routing::get, Router};
use leaky_bucket::RateLimiter;
use std::sync::Arc;
use std::time::Duration;

mod api;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let rate_limiter = Arc::new(
        RateLimiter::builder()
            .max(5)
            .initial(5)
            .interval(Duration::from_secs(1))
            .build()
    );

    let router = Router::new()
        .route("/", get(hello_world))
        .route("/-1/seek", get(headers))
        .route("/2/dest", get(calculate_ip_add))
        .route("/2/key", get(calculate_ip_sub))
        .route("/2/v6/dest", get(calculate_ipv6_add))
        .route("/2/v6/key", get(calculate_ipv6_sub))
        .route("/5/manifest", post(manifest))
        .route("/9/milk", post({
            let limiter = rate_limiter.clone();
            move |headers, body| post_milk(headers, body, limiter)
        }))
        .route("/9/refill", post(post_refill));

    Ok(router.into())
}
