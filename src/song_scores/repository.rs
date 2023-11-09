use std::fmt::format;
use std::sync::Arc;
use chrono::Duration;
use scylla::Session;
use scylla::frame::value::Timestamp;
use scylla::prepared_statement::PreparedStatement;
use super::http::actions::CreateSongScoreRequest;
use scylla::batch::Batch;
use scylla::query::Query;

pub const INSERT_SONG_SCORE: &str = "
    INSERT INTO yarg.song_scores 
        (score_id, song_id, user_id, difficulty, instrument, stars, score, accuracy_percentage, missed_count, ghost_notes_count, max_combo_count, overdrive_count, speed, played_at, modifiers)
        VALUES
        (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ? ,? , ?);
";

pub struct SongScoreRepository{
    db: Arc<Session>
}

impl SongScoreRepository {
    pub fn new(db: Arc<Session>) -> Self {
        Self { db }
    }

    pub async fn store(&self, request: CreateSongScoreRequest) -> Result<(), String> {
        let prepared_query: PreparedStatement = self.db.prepare(INSERT_SONG_SCORE).await.unwrap();
        let _ = self.db.execute(&prepared_query, request.to_database()).await.unwrap();

        
        Ok(())
    }
}