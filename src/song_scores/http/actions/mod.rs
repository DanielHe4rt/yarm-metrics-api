use serde::Deserialize;

pub mod new_submission;


#[derive(Deserialize, Debug, Clone)]
pub struct CreateSongScoreRequest {
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
    played_at: i32,   
}