use std::result::Result;
use sqlx::{sqlite::SqliteQueryResult, Sqlite, SqlitePool, migrate::MigrateDatabase};

async fn create_schema(db_url: &str) -> Result<SqliteQueryResult, sqlx::Error> {
    let pool = SqlitePool::connect(db_url).await?;
    let query = "CREATE TABLE IF NOT EXISTS settings (
        id INTEGER PRIMARY KEY,
        name TEXT NOT NULL
    )";
    let result = sqlx::query(&query).execute(&pool).await;
    pool.close().await;
    result
    
}

#[async_std::main]
async fn main() {
    let db_url = String::from("sqlite://sqlite.db");
    if !Sqlite::database_exists(&db_url).await.unwrap_or(false) {
        Sqlite::create_database(&db_url).await.unwrap();
        match create_schema(&db_url).await {
            Ok(_) => println!("Schema created successfully"),
            Err(e) => panic!("Error creating schema: {}", e),
        }
    }
    let instances = SqlitePool::connect(&db_url).await.unwrap();
    let query = "INSERT INTO settings (name) VALUES ($1)";
    let result = sqlx::query(&query).bind("test").execute(&instances).await;
    instances.close().await;
    println!("{:?}", result);
    println!("Hello, world!");
}
