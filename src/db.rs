use rusqlite::{Connection, Result, Error};
use dotenv::dotenv;
use std::path::Path;
use std::env;

pub fn init_database() -> Result<(), Error> {
    dotenv().ok();

    let database_path = env::var("DATABASE_PATH").expect("DATABASE_URL must be set");
    
    
    if !Path::new(&database_path).exists() {
        let conn = Connection::open(&database_path)?;

        conn.execute(
            "CREATE TABLE hardware (
                id INTEGER PRIMARY KEY,
                gpu TEXT NOT NULL,
                ram TEXT NOT NULL,
                internet_speed TEXT NOT NULL
            )",
            [],
        )?;

        println!("Database created at {}", &database_path);
        println!("Table hardware created");
    } else {
        println!("Database already exists at {}", &database_path);
    }

    Ok(())
}