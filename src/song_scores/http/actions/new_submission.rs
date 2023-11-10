use std::sync::Arc;

use axum::{Extension, Json};
use scylla::Session;
use serde_json::{json, Value};
use uuid::{Context, Timestamp, Uuid};

use crate::song_scores::repository::SongScoreRepository;

use super::CreateSongScoreRequest;

pub async fn new_submission(
    Extension(db): Extension<Arc<Session>>,
    Json(mut payload): Json<CreateSongScoreRequest>,
) -> Json<Value> {
    let context = Context::new(rand::random::<u16>());
    let now = Timestamp::now(context);
    let timeuuid = Uuid::new_v1(now, &[1,2,3,4,5,6]);

    payload.submission_id = timeuuid.to_string();
    
    let repository = SongScoreRepository::new(db);

    let _ = repository.store(payload).await;

    Json(json!({ "data": "ok"}))
}
