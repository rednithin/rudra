use dotenv;
use sqlx::SqlitePool;

#[derive(Clone)]
pub struct AppState {
    pub pool: SqlitePool,
}

pub async fn create_state() -> AppState {
    let db_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL is not set.");

    let pool = SqlitePool::new(&db_url)
        .await
        .expect("Couldn't connect to database.");

    AppState { pool }
}
