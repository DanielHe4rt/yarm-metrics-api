use chrono::Duration;
use scylla::frame::value::Timestamp;
use serde::Deserialize;

pub mod new_submission;
pub mod fetch_submission;


#[derive(Deserialize, Debug, Clone)]
pub struct CreateSongScoreRequest {
    pub song_id: String,
    pub user_id: String,
    pub score_id: String,
    pub difficulty: String,
    pub instrument: String,
    pub stars: i32,
    pub score: i32,
    pub accuracy_percentage: f32,
    pub missed_count: i32,
    pub ghost_notes_count: i32,
    pub max_combo_count: i32,
    pub overdrive_count: i32,
    pub speed: i32,
    pub played_at: i32,   
    pub modifiers: Vec<String>
}

impl CreateSongScoreRequest {
    pub fn to_database(self) -> (String, String, String, String, String, i32, i32, f32, i32, i32, i32, i32, i32, Timestamp, Vec<String>) {
        (
            self.score_id,
            self.song_id,
            self.user_id,
            self.difficulty,
            self.instrument,
            self.stars,
            self.score,
            self.accuracy_percentage,
            self.missed_count,
            self.ghost_notes_count,
            self.max_combo_count,
            self.overdrive_count,
            self.speed,
            Timestamp(Duration::seconds(self.played_at as i64)),
            self.modifiers
        )
    }
}