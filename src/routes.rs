use warp::{self, Filter};

use crate::handlers;
use crate::state::AppState;

fn with_state(
    state: AppState,
) -> impl Filter<Extract = (AppState,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || state.clone())
}

pub fn routes(
    state: AppState,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path::end()
        .map(handlers::index::index)
        .or(warp::path!("api" / "users")
            .and(warp::get())
            .and(with_state(state.clone()))
            .and_then(handlers::users::fetch_all_users))
}
