use std::sync::Arc;

use axum::{Extension, Json};
use scylla::Session;
use serde_json::{json, Value};

use crate::song_scores::repository::SongScoreRepository;

use super::CreateSongScoreRequest;

pub async fn new_submission(
    Extension(db): Extension<Arc<Session>>,
    Json(payload): Json<CreateSongScoreRequest>,
) -> Json<Value> {
    let repository = SongScoreRepository::new(db);

    let _ = repository.store(payload).await;

    Json(json!({ "data": "ok"}))
}
