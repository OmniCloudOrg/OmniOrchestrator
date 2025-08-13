//! Build management module for handling build operations.
//!
//! This module provides a REST API for managing builds, including:
//! - Listing builds with pagination
//! - Getting build details
//! - Listing builds for specific applications

// Import and re-export all route modules
pub mod list;
pub mod get;

// Re-export all route functions
pub use list::{list_builds, list_builds_for_app};
pub use get::get_build;