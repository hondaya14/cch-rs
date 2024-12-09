use std::collections::HashMap;
use std::str::FromStr;
use axum::http::{HeaderMap, StatusCode};
use axum::Json;
use axum::response::{IntoResponse, Response};
use crate::api::interface::day9::{MilkUnitType, MilkUnits, SpecifiedUnit};

pub async fn post_milk(headers: HeaderMap, units: Json<SpecifiedUnit>) -> impl IntoResponse {

    // todo: RateLimit
    
    let content_type = headers.get("content-type")?;
    let is_unit_specified  = content_type == "application/json";

    if !units.validate() {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body("")
            .unwrap();
    }

    let milk_units: MilkUnits = if is_unit_specified{
        MilkUnits::from(units)
    } else {
        // default is liters
        MilkUnits::new(1f32, MilkUnitType::Liters)
    };

    match milk_units.unit_type {
        MilkUnitType::Liters => {
            if is_unit_specified {
                Response::builder()
                    .status(StatusCode::OK)
                    .body(Json(HashMap::from([("gallons", milk_units.gallons)])))
                    .unwrap()
            } else {
                Response::builder()
                    .status(StatusCode::OK)
                    .body("")
                    .unwrap()
            }
        }
        MilkUnitType::Gallons => {
            Response::builder()
                .status(StatusCode::OK)
                .body(Json(HashMap::from([("liters", milk_units.liters)])))
                .unwrap()
        }
        MilkUnitType::Litres => {
            Response::builder()
                .status(StatusCode::OK)
                .body(Json(HashMap::from([("pints", milk_units.pints)])))
                .unwrap()
        }
        _ => {}
    }
    
}

pub async fn post_refill(headers: HeaderMap, body: String) -> impl IntoResponse {
    "Refill"
}