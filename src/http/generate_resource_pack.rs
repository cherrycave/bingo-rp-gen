use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode};
use serde::{Deserialize, Serialize};
use sha1_checked::Sha1;

use crate::{
    ResourcePackState, bingo_board_image::generate_bingo_board,
    resource_pack::generate_resource_pack,
};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenerateResourcePackRequest {
    pub materials: Vec<String>,
    pub board_size: u32,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenerateResourcePackResponse {
    pub hash: String,
    pub download_url: String,
}

pub async fn resource_pack_endpoint(
    State(state): State<Arc<ResourcePackState>>,
    Json(request): Json<GenerateResourcePackRequest>,
) -> Result<Json<GenerateResourcePackResponse>, StatusCode> {
    let bingo_board = generate_bingo_board(request.materials, request.board_size);

    let resource_pack = generate_resource_pack(std::path::Path::new("./base_pack/"), &bingo_board);
    if let Err(err) = resource_pack {
        tracing::error!("Failed to generate resource pack: {}", err);
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }
    let resource_pack = resource_pack.unwrap();

    let sha1 = Sha1::try_digest(&resource_pack);
    let sha1 = sha1.hash();
    let hex_hash = format!("{:x}", sha1);

    state.resource_packs.lock().await.insert(hex_hash.clone(), resource_pack);

    Ok(Json(GenerateResourcePackResponse {
        hash: hex_hash.clone(),
        download_url: format!("/resource-pack/{}", hex_hash),
    }))
}
