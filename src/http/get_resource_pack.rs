use std::sync::Arc;

use axum::{extract::{Path, State}, http::{StatusCode, header}, response::{AppendHeaders, IntoResponse}};

use crate::ResourcePackState;

pub async fn get_resource_pack_endpoint(
    Path(hash): Path<String>,
    State(state): State<Arc<ResourcePackState>>,
) -> Result<impl IntoResponse, StatusCode> {
    let resource_pack = state.resource_packs.lock().await.get(&hash).cloned();

    if resource_pack.is_none() {
        return Err(StatusCode::NOT_FOUND);
    }

    let resource_pack = resource_pack.unwrap();

    let headers = AppendHeaders([
        (header::CONTENT_TYPE, "application/zip"),
        (
            header::CONTENT_DISPOSITION,
            "attachment; filename=\"resource_pack.zip\"",
        ),
    ]);

    Ok((headers, resource_pack))
}
