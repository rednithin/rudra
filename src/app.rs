use crate::{routes, state};
use std::net::SocketAddr;
use warp;

fn get_bind_address() -> SocketAddr {
    dotenv::var("BIND_ADDRESS")
        .expect("BIND_ADDRESS is not set")
        .parse()
        .expect("BIND_ADDRESS is invalid")
}

pub async fn start() {
    let state = state::create_state().await;
    let bind_address = get_bind_address();

    let routes = routes::routes(state);

    warp::serve(routes).run(bind_address).await;
}
