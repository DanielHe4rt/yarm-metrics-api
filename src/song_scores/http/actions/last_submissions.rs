use std::sync::Arc;

use axum::{Json, Extension};
use scylla::Session;
use serde_json::{json, Value};

pub async fn last_submissions(
    Extension(db): Extension<Arc<Session>>
) -> Json<Value> {
    Json(json!({ "data": "not found" }))
}
