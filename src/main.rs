use actix_web::main;
use dotenv::dotenv;
use env_logger::Env;
use infrastructure::web::run;

pub mod application;
pub mod domain;
pub mod infrastructure;
pub mod presentation;
pub mod schema;

#[main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    run().await
}
