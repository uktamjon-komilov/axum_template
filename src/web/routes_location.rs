use crate::{
    ctx::Ctx,
    model::location::{Location, LocationBatchSave, LocationModelController, LocationSave},
    Result,
};
use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};

pub fn routes(mc: LocationModelController) -> Router {
    Router::new()
        .route("/trajectory/", get(list_user_locations_handler))
        .route("/save/", post(save_location_handler))
        .route("/save/batch/", post(save_locations_batch_handler))
        .with_state(mc)
}

async fn save_location_handler(
    ctx: Ctx,
    State(mc): State<LocationModelController>,
    Json(payload): Json<LocationSave>,
) -> Result<Json<Location>> {
    let location = mc.save_location(payload, ctx.user_id()).await.unwrap();

    Ok(Json(location))
}

async fn list_user_locations_handler(
    ctx: Ctx,
    State(mc): State<LocationModelController>,
) -> Result<Json<Vec<Location>>> {
    let locations = mc.list_user_locations(ctx.user_id()).await.unwrap();

    Ok(Json(locations))
}

async fn save_locations_batch_handler(
    ctx: Ctx,
    State(mc): State<LocationModelController>,
    Json(payload): Json<LocationBatchSave>,
) -> Result<Json<()>> {
    mc.save_batch_locations(payload, ctx.user_id())
        .await
        .unwrap();

    Ok(Json(()))
}
