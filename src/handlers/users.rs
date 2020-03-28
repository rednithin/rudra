use crate::state::AppState;

use warp;

pub async fn fetch_all_users(state: AppState) -> Result<impl warp::Reply, warp::Rejection> {
    #[derive(serde::Serialize, Debug)]
    struct User {
        id: i32,
        name: String,
    }

    let row = sqlx::query_as!(User, "SELECT * FROM Users")
        .fetch_all(&state.pool)
        .await
        .unwrap();

    Ok(warp::reply::json(&row))
}
