mod models;
mod db;
mod api;

use rusqlite::Connection;
use std::sync::{Arc, Mutex};

fn main() {
    // Create a database connection and handle potential errors
    let db_connection = match Connection::open("my_database.db") {
        Ok(conn) => conn,
        Err(err) => {
            eprintln!("Error opening database: {}", err);
            return;
        }
    };

    // Ensure the necessary database tables exist
    if let Err(err) = db::create_tables(&db_connection) {
        eprintln!("Error creating tables: {}", err);
        return;
    }

    // Share the connection across threads using Arc and Mutex
    let shared_connection = Arc::new(Mutex::new(db_connection));

    // Initialize and start the API service
    api::start_api(shared_connection);
}
