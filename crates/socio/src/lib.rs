//! Socio AI Agent - Main Library
//!
//! This is the main library crate for the Socio AI Agent application.
//! It provides the core functionality for social media content generation
//! and posting through AI agent tools.

pub use socio_backend;

// Re-export key types and functions for external use
pub use socio_backend::types::app_state::AppState;
pub use socio_backend::types::error::SocioError;
