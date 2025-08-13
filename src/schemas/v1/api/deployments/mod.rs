//! Deployment management module for handling deployment operations.
//!
//! This module provides a REST API for managing deployments, including:
//! - Listing deployments with pagination
//! - Getting deployment details
//! - Creating and managing deployments
//! - Updating deployment status
//! - Deleting deployments

// Import and re-export all modules
pub mod types;
pub mod list;
pub mod get;
pub mod create;
pub mod update;
pub mod delete;

// Re-export all route functions
pub use list::{list_deployments, count_deployments, list_app_deployments};
pub use get::get_deployment;
pub use create::create_deployment;
pub use update::update_deployment_status;
pub use delete::delete_deployment;