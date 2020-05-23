use dotenv::dotenv;
use pretty_env_logger;

mod app;
mod handlers;
mod routes;
mod state;

#[tokio::main(core_threads = 24)]
async fn main() {
    dotenv().unwrap();
    pretty_env_logger::init();

    app::start().await;
}
