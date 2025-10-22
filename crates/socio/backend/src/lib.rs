//! Main library module for the Socio backend.
//!
//! This module provides the core functionality for the Socio AI Agent backend,
//! including API routes, agent tools, and business logic.
//! Follows the project's one-item-per-file coding standards.

pub mod agent_tools;
pub mod routes;
pub mod services;
pub mod types;
pub mod openapi;

// Re-export commonly used types
pub use types::app_state::AppState;
pub use types::error::SocioError;
