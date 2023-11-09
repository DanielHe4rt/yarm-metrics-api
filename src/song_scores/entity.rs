

/*
table: song_scores 
* song_id: string (41)
* season_id: uuid
* user_id: uuid
* difficulty: string
* instrument: string
* stars: int
* score: int (10)
* accuracy_percentage: float 
* missed_count: int
* ghost_notes_count: int
* max_combo_count: int (8)
* overdrive_count: int(4)
* speed: int (4)
* played_at: timestamp

*/

use scylla::{FromRow, ValueList, frame::value::Timestamp};
use uuid::Uuid;

#[derive(Debug, FromRow, ValueList, Clone)]
pub struct SongScore {
    song_id: String,
    season_id: Uuid,
    user_id: uuid::Uuid,
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
    played_at: Timestamp,   
}