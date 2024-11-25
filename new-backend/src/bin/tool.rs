use financrr::app::App;
use financrr::utils::env::load_env_file;
use loco_rs::cli;
use migration::Migrator;

#[tokio::main]
async fn main() -> loco_rs::Result<()> {
    load_env_file();

    cli::main::<App, Migrator>().await
}
