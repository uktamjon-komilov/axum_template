mod ctx;
mod error;
mod log;
mod model;
mod web;

pub use self::error::{Error, Result};
use crate::log::log_request;
use crate::model::location::LocationModelController;
use crate::web::mw_auth::{mw_ctx_resolver, mw_require_auth};
use crate::web::routes_location;
use axum::http::{Method, Uri};
use axum::Json;
use axum::{
    middleware,
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use ctx::Ctx;
use serde_json::json;
use std::net::SocketAddr;
use tower_cookies::CookieManagerLayer;
use uuid::Uuid;

async fn health_check() -> impl IntoResponse {
    Html("<b>Pong!</b>")
}

#[allow(unused)]
#[tokio::main]
async fn main() -> Result<()> {
    let mc: LocationModelController = LocationModelController::new().await?;

    let api_v1: Router = Router::new()
        .nest("/locations", routes_location::routes(mc.clone()))
        .layer(middleware::from_fn(mw_require_auth));

    let routes: Router = Router::new()
        .route("/ping", get(health_check))
        .nest("/api/v1/auth", web::routes_auth::routes())
        .nest("/api/v1", api_v1)
        .layer(middleware::map_response(main_response_mapper))
        .layer(middleware::from_fn_with_state(mc.clone(), mw_ctx_resolver))
        .layer(CookieManagerLayer::new());

    let addr: SocketAddr = SocketAddr::from(([127, 0, 0, 1], 8000));
    println!("->> LISTENING on {addr}\n");

    axum::Server::bind(&addr)
        .serve(routes.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn main_response_mapper(
    ctx: Option<Ctx>,
    uri: Uri,
    req_method: Method,
    res: Response,
) -> Response {
    println!("->> {:12} - main_response_mapper", "RES_MAPPER");

    let uuid = Uuid::new_v4();
    let service_error = res.extensions().get::<Error>();
    let client_status_error = service_error.map(|se| se.client_status_and_error());

    let error_response = client_status_error
        .as_ref()
        .map(|(status_code, client_error)| {
            let client_error_body = json!({
                "error": {
                    "type": client_error.as_ref(),
                    "req_uuid": uuid.to_string()
                }
            });

            println!("    ->> client_error_body: {client_error_body}");

            (*status_code, Json(client_error_body)).into_response()
        });

    let client_error = client_status_error.unzip().1;

    let _ = log_request(uuid, req_method, uri, ctx, service_error, client_error).await;

    println!("    ->> server log line - {uuid} - Error {service_error:?}");

    error_response.unwrap_or(res)
}
