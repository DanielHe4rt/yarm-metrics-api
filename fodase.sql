INSERT INTO yarg.song_scores 
    (song_id, user_id, difficulty, instrument, stars,score,accuracy_percentage,missed_count,ghost_notes_count,max_combo_count,overdrive_count,speed,played_at)
    VALUES
    (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ? ,?);