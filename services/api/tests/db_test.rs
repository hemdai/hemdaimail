use sqlx::PgPool;
use std::env;

#[tokio::test]
async fn test_db_connection() {
    dotenv::dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("Connecting to {}", database_url);
    let pool = PgPool::connect(&database_url).await.expect("Failed to connect to DB");
    
    let row: (i32,) = sqlx::query_as("SELECT 1").fetch_one(&pool).await.expect("Failed to query DB");
    assert_eq!(row.0, 1);
}
