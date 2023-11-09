use scylla::{FromRow, ValueList, cql_to_rust::FromRowError, QueryResult, IntoTypedRows};
use serde::Serialize;

#[derive(Debug, FromRow, ValueList, Clone, Serialize)]
pub struct SongScore {
    score_id: String,
    song_id: String,
    user_id: String,
    difficulty: String,
    instrument: String,
    stars: i32,
    score: i32,
    accuracy_percentage: f32,
    missed_count: i32,
    ghost_notes_count: i32,
    max_combo_count: i32,
    overdrive_count: i32,
    speed: i32,
    played_at: i64,
    modifiers: Vec<String>,
}


impl SongScore {
    pub fn fromRow(row: QueryResult) -> Result<SongScore, FromRowError> {
        row.rows.unwrap().into_typed::<SongScore>().next().unwrap()
    }
}