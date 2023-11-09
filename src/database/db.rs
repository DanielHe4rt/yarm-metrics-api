use std::{env, time::Duration};

use scylla::{Session, SessionBuilder};

pub struct Database {
    pub session: Session,
}

impl Database {
    pub async fn new() -> Database {
        let raw_hostname = env::var("DB_HOSTNAMES").unwrap().to_string();
        let username = env::var("DB_USERNAME").unwrap().to_string();
        let password = env::var("DB_PASSWORD").unwrap().to_string();

        let nodes = raw_hostname.split(",").collect::<Vec<&str>>();
    

        let session: Session = SessionBuilder::new()
            .known_nodes(&nodes)
            .connection_timeout(Duration::from_secs(5))
            .user(username, password)
            .build()
            .await
            .expect("Connection Refused. Check your credentials and your IP linked on the ScyllaDB Cloud.");

        return Self { session };
    }

}