use std::sync::Arc;

use scylla::Session;

use crate::database::db::Database;

use super::http::actions::CreateSongScoreRequest;



pub struct SongScoreRepository{
    db: Arc<Session>
}

impl SongScoreRepository {
    pub fn new(db: Arc<Session>) -> Self {
        Self { db }
    }

    pub async fn store(&self, request: CreateSongScoreRequest) -> Result<(), String> {
        
        
        Ok(())
    }
}