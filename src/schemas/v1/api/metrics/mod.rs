//! Metrics management module for handling metrics operations.
//!
//! This module provides a REST API for managing metrics, including:
//! - Getting metrics by instance ID
//! - Getting all metrics for a platform

// Import and re-export all modules
pub mod get;

// Re-export all route functions
pub use get::{get_metrics_by_app_id, get_metrics};