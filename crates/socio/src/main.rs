use anyhow::Result;
use dotenvy::dotenv;
use socio::create_app;
use tracing_subscriber;

#[actix_web::main]
async fn main() -> Result<()> {
    // Load environment variables
    dotenv().ok();

    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Create and run the application
    create_app().await?;

    Ok(())
}
