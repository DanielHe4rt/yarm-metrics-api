use scylla::Session;

pub async fn migrate_database(database: &Session) -> Result<(), anyhow::Error> {
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

    let ddl_queries = vec![
        (
            String::from("keyspace"),
            String::from(
                "DROP KEYSPACE IF EXISTS yarg;",
            ),
        ),
        (
            String::from("keyspace"),
            String::from(
                "CREATE KEYSPACE IF NOT EXISTS yarg WITH REPLICATION = { 'class' : 'SimpleStrategy', 'replication_factor' : 3 };",
            ),
        ),
        (
            String::from("drop song_scores table"),
            String::from(
                "DROP TABLE IF EXISTS yarg.song_scores;",
            ),
        ),
        (
            String::from("add song_scores table"),
            String::from(
                "CREATE TABLE IF NOT EXISTS yarg.song_scores (
                    song_id text,
                    user_id text,
                    score_id text,
                    modifiers frozen<set<text>>,
                    score int,
                    difficulty text,
                    instrument text,
                    stars int,
                    accuracy_percentage float,
                    missed_count int,
                    ghost_notes_count int,
                    max_combo_count int,
                    overdrive_count int,
                    speed int,
                    played_at timestamp,
                    PRIMARY KEY (score_id)   
                );",
            ),
        ),

    ];

    let submissions_queries = vec![
        String::from("
            INSERT INTO yarg.song_scores 
                (score_id, song_id, user_id, difficulty, instrument, stars, score, accuracy_percentage, missed_count, ghost_notes_count, max_combo_count, overdrive_count, speed, played_at, modifiers)
            VALUES
                ('1', 'fuel', 'danielhe4rt', 'expert+', 'guitar', 5, 1000, 100, 0, 0, 0, 10, 10 , 1699550482 , {'all-taps'})
        ")
    ];

    // Materialized view pra essa porra

    println!("-----------------------------------");
    println!("->.......Verifying Database.......<-");

    for (operation, query) in ddl_queries {
        println!("-> Verifying: {}", operation);
        database.query(query.trim(), []).await.unwrap();
    } 

    for query in submissions_queries {
        database.query(query, []).await?;
    }

    println!("->............All Good!...........<-");
    println!("------------------------------------");

    Ok(())
}
