use super::entity::SongScore;
use super::http::actions::CreateSongScoreRequest;
use anyhow::Error;
use scylla::prepared_statement::PreparedStatement;
use scylla::{IntoTypedRows, Session};
use std::sync::Arc;

pub const INSERT_SONG_SCORE: &str = "
    INSERT INTO yarg.song_scores 
        (score_id, song_id, user_id, difficulty, instrument, stars, score, accuracy_percentage, missed_count, ghost_notes_count, max_combo_count, overdrive_count, speed, played_at, modifiers)
        VALUES
        (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ? ,? , ?);
";

pub const SELECT_SONG_SCORE_BY_SCORE_ID: &str = "
    SELECT score_id,
        song_id,
        user_id,
        difficulty,
        instrument,
        stars,
        score,
        accuracy_percentage,
        missed_count,
        ghost_notes_count,
        max_combo_count,
        overdrive_count,
        speed,
        played_at,
        modifiers 
    FROM 
        yarg.song_scores 
    WHERE 
        score_id = ?
";

pub struct SongScoreRepository {
    db: Arc<Session>,
}

impl SongScoreRepository {
    pub fn new(db: Arc<Session>) -> Self {
        Self { db }
    }

    pub async fn store(&self, request: CreateSongScoreRequest) -> Result<(), String> {
        let prepared_query: PreparedStatement = self.db.prepare(INSERT_SONG_SCORE).await.unwrap();
        let _ = self
            .db
            .execute(&prepared_query, request.to_database())
            .await
            .unwrap();

        Ok(())
    }

    pub(crate) async fn find_by_score_id(
        &self,
        score_id: String,
    ) -> Result<SongScore, anyhow::Error> {
        let prepared_query: PreparedStatement = self
            .db
            .prepare(SELECT_SONG_SCORE_BY_SCORE_ID)
            .await
            .unwrap();

        let row = self.db.execute(&prepared_query, (score_id,)).await.unwrap();
        let row = SongScore::fromRow(row);

        match row {
            Ok(row) => Ok(row),
            Err(row) => Err(Error::from(row)),
        }
    }
}
