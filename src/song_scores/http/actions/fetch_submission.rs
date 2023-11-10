use std::sync::Arc;

use axum::{extract::Path, Json, Extension};
use scylla::Session;
use serde_json::{json, Value};

use crate::song_scores::repository::SongScoreRepository;

pub async fn fetch_submission(
    Path(score_id): Path<String>,
    Extension(db): Extension<Arc<Session>>
) -> Json<Value> {

    println!("score_id: {}", score_id);
    let repository = SongScoreRepository::new(db);
    let result = repository.find_by_score_id(score_id).await;
    println!("result: {:?}", result);

    match result {
        Ok(result) => Json(json!({ "data": result })),
        Err(_) => Json(json!({ "data": "not found" }))
    }
}
