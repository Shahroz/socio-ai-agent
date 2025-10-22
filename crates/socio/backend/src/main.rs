//! Main entry point for the Socio backend server.
//!
//! This binary starts the Actix-web server with all necessary middleware,
//! routes, and application state. It follows the project's coding standards
//! and integrates with the agentloop and llm crates.

use anyhow::Result;
use dotenvy::dotenv;
use tracing_subscriber;
use socio_backend::routes::app_setup::create_app;

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
