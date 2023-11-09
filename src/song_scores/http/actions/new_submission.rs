use std::{sync::Arc, net::IpAddr};

use axum::{Extension, Json};
use scylla::{Session, IntoTypedRows};
use serde_json::{Value, json};

use crate::song_scores::repository::SongScoreRepository;

use super::CreateSongScoreRequest;



pub async fn new_submission(
    Extension(db): Extension<Arc<Session>>,
    Json(payload): Json<CreateSongScoreRequest>
) -> Json<Value> {
    let repository = SongScoreRepository::new(db);
    

    let _ = repository.store(payload).await;

    Json(json!({ "data": "ok"}))
}