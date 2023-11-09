mod database;
mod song_scores;

use axum::routing::{get, post};
use axum::Extension;
use axum::Router;
use database::db::Database;
use database::migrate::migrate_database;
use song_scores::http::actions::fetch_submission::fetch_submission;
use song_scores::http::actions::new_submission::new_submission;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let _ = dotenv::dotenv();

    let db = Arc::new(Database::new().await.session);

    let _ = migrate_database(&db).await;

    // create a scylladb connection

    let app = Router::new()
        .route("/songs/:score_id", get(fetch_submission))
        .route("/songs/metrics", post(new_submission))
        .layer(Extension(db));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
