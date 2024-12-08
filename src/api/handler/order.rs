use crate::api::interface::order::parse_orders;
use axum::http::{HeaderMap, Response, StatusCode};
use axum::response::IntoResponse;
use cargo_manifest::Manifest;
use shuttle_runtime::__internals::serde_json;
use std::str::FromStr;

pub async fn manifest(headers: HeaderMap, body: String) -> impl IntoResponse {
    // task 4
    const APPLICATION_TOML: &str = "application/toml";
    const APPLICATION_YAML: &str = "application/yaml";
    const APPLICATION_JSON: &str = "application/json";
    let allowed_content_types = vec![APPLICATION_TOML, APPLICATION_YAML, APPLICATION_JSON];

    let content_type = headers
        .get("content-type")
        .and_then(|value| value.to_str().ok())
        .unwrap();

    if !allowed_content_types.contains(&content_type) {
        return Response::builder()
            .status(StatusCode::UNSUPPORTED_MEDIA_TYPE)
            .body("Unsupported media type".to_string())
            .unwrap();
    }

    let manifest: Result<Manifest, String> = match content_type {
        APPLICATION_TOML => {
            let parsed_toml = Manifest::from_str(body.as_str());
            if parsed_toml.is_ok() {
                Ok(parsed_toml.unwrap())
            } else {
                Err("parse toml failed".to_string())
            }
        }
        APPLICATION_YAML => {
            let parsed: Result<Manifest, serde_yaml::Error> = serde_yaml::from_str(body.as_str());
            if parsed.is_ok() {
                Ok(parsed.unwrap())
            } else {
                Err("parse yaml failed".to_string())
            }
        }
        APPLICATION_JSON => {
            let parsed: Result<Manifest, serde_json::Error> = serde_json::from_str(body.as_str());
            if parsed.is_ok() {
                Ok(parsed.unwrap())
            } else {
                Err("parse yaml failed".to_string())
            }
        }
        _ => Err("Unsupported media type".to_string()),
    };

    match manifest {
        Ok(manifest) => {
            let keywords = manifest
                .package
                .as_ref()
                .and_then(|p| p.keywords.clone())
                .and_then(|k| k.as_local());

            let magic_word = String::from("Christmas 2024");
            if keywords.map_or(true, |keyword| !keyword.contains(&magic_word)) {
                return Response::builder()
                    .status(StatusCode::BAD_REQUEST)
                    .body("Magic keyword not provided".to_string())
                    .unwrap();
            }

            let order_v = manifest
                .package
                .as_ref()
                .and_then(|p| p.metadata.as_ref())
                .and_then(|m| m.get("orders"));
            let orders = parse_orders(order_v);
            if orders.len() == 0 {
                return Response::builder()
                    .status(StatusCode::NO_CONTENT)
                    .body("".to_string())
                    .unwrap();
            }

            let body = orders
                .iter()
                .map(|order| order.to_string())
                .collect::<Vec<String>>()
                .join("\n");
            Response::builder()
                .status(StatusCode::OK)
                .body(body)
                .unwrap()
        }
        Err(e) => {
            println!("Error: {:?}", e);
            let body = "Invalid manifest".to_string();
            return Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(body)
                .unwrap();
        }
    }
}
