use scylla::{Session, cql_to_rust::FromCqlVal};
use uuid::{Uuid, Timestamp, Context};

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
    
    let context = Context::new(rand::random::<u16>());
    let now = Timestamp::now(context);
    let test_uuid = Uuid::new_v1(now, &[1,2,3,4,5,6]);
    
    println!("uuid: {}", test_uuid);

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
            String::from("add song_scores table"),
            String::from(
                "CREATE TABLE IF NOT EXISTS yarg.song_scores (
                    submission_id timeuuid,
                    song_id text,
                    user_id text,
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
                    PRIMARY KEY (submission_id)   
                );",
            ),
        ),
        (
            String::from("add last_submissions table"),
            String::from(
                "CREATE TABLE IF NOT EXISTS yarg.last_submissions (
                    submission_id timeuuid,
                    song_id text,
                    user_id text,
                    song_name text,
                    charter_name text,
                    country_code text,
                    played_at timestamp,
                    PRIMARY KEY (submission_id)
                );",
            ),
        ),

    ];

    let submissions_queries = vec![
        String::from("
            INSERT INTO yarg.song_scores 
                (submission_id, song_id, user_id, difficulty, instrument, stars, score, accuracy_percentage, missed_count, ghost_notes_count, max_combo_count, overdrive_count, speed, played_at, modifiers)
            VALUES
                (09758799-7f57-11ee-8f2f-010203040506, 'fuel', 'danielhe4rt', 'expert+', 'guitar', 5, 1000, 100, 0, 0, 0, 10, 10 , 1699550482 , {'all-taps'})
        "),
        String::from("
            INSERT INTO yarg.song_scores 
                (submission_id, song_id, user_id, difficulty, instrument, stars, score, accuracy_percentage, missed_count, ghost_notes_count, max_combo_count, overdrive_count, speed, played_at, modifiers)
            VALUES
                (20e444e3-7f57-11ee-a46e-010203040506, 'break stuff', 'danielhe4rt', 'expert+', 'guitar', 5, 1000, 100, 0, 0, 0, 10, 10 , 1699560482 , {'all-taps'})
        "),
        String::from("
            INSERT INTO yarg.last_submissions 
                (submission_id, song_id, user_id, song_name, charter_name, country_code, played_at)
            VALUES
                (09758799-7f57-11ee-8f2f-010203040506, 'fuel', 'danielhe4rt', 'Fuel', 'Metallica', 'US', 1699550482)
        "),
        String::from("
            INSERT INTO yarg.last_submissions 
                (submission_id, song_id, user_id, song_name, charter_name, country_code, played_at)
            VALUES
                (20e444e3-7f57-11ee-a46e-010203040506, 'break stuff', 'danielhe4rt', 'Break Stuff', 'Limp Bizkit', 'BR', 169964482)
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
        println!("-> Adding Rows...", );
        database.query(query, []).await.unwrap();
    }

    println!("->............All Good!...........<-");
    println!("------------------------------------");

    Ok(())
}
