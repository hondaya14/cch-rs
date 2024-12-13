use crate::api::interface::day9::{MilkUnitType, MilkUnits, SpecifiedUnit};
use axum::extract::rejection::JsonRejection;
use axum::http::{HeaderMap, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::Json;
use leaky_bucket::RateLimiter;
use std::sync::Arc;

pub async fn post_milk(
    headers: HeaderMap,
    units: Result<Json<SpecifiedUnit>, JsonRejection>,
    limiter: Arc<RateLimiter>,
) -> impl IntoResponse {
    let rate_limit_ok = limiter.try_acquire(1);

    let content_type = headers.get("content-type");
    // option checkしながら、application/jsonかどうかを判定してbooleanの変数を持つ
    let is_unit_specified = content_type.map_or(
        false, |value| value == "application/json");

    if !is_unit_specified {
        if rate_limit_ok {
            return Response::builder()
                .status(StatusCode::OK)
                .body("Milk withdrawn\n".to_string())
                .unwrap();
        } else {
            return Response::builder()
                .status(StatusCode::TOO_MANY_REQUESTS)
                .body("No milk available\n".to_string())
                .unwrap();
        }
    }

    let units = match units {
        Ok(valid_units) => valid_units,
        Err(_) => {
            return Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body("".to_string())
                .unwrap();
        }
    };

    if !units.validate() {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body("".to_string())
            .unwrap();
    }

    let milk_units: MilkUnits = if is_unit_specified {
        println!("Debug: units.0 = {:?}", units.0);
        MilkUnits::from(units.0)
    } else {
        // default is liters
        MilkUnits::new(1f32, MilkUnitType::Liters)
    };
    println!("Debug: milk_units = {:?}", milk_units);

    match milk_units.unit_type {
        MilkUnitType::Liters => {
            if is_unit_specified {
                Response::builder()
                    .status(StatusCode::OK)
                    .body(format!("{{\"gallons\":{}}}", milk_units.gallons))
                    .unwrap()
            } else {
                Response::builder()
                    .status(StatusCode::OK)
                    .body("".to_string())
                    .unwrap()
            }
        }
        MilkUnitType::Gallons => {
            Response::builder()
                .status(StatusCode::OK)
                .body(format!("{{\"liters\":{}}}", milk_units.liters))
                .unwrap()
        }
        MilkUnitType::Litres => {
            Response::builder()
                .status(StatusCode::OK)
                .body(format!("{{\"pints\":{}}}", milk_units.pints))
                .unwrap()
        }
        MilkUnitType::Pints => {
            Response::builder()
                .status(StatusCode::OK)
                .body(format!("{{\"litres\":{}}}", milk_units.liters))
                .unwrap()
        }
        _ => {
            Response::builder()
                .status(StatusCode::OK)
                .body("".to_owned())
                .unwrap()
        }
    }
}

pub async fn post_refill(headers: HeaderMap, body: String) -> impl IntoResponse {
    "Refill"
}