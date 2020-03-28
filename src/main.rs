use dotenv::dotenv;

mod app;
mod handlers;
mod routes;
mod state;

#[tokio::main(core_threads = 24)]
async fn main() {
    dotenv().unwrap();

    app::start().await;
}
