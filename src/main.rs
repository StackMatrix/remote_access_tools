mod app;
mod bootstrap;
mod untils;

use tokio;

#[tokio::main]
async fn main() {
    bootstrap::app_logic::run().await
        .map_err(|e| println!("Error: {:#?}", e))
        .ok();
}