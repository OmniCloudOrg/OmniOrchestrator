//! Application management module for handling CRUD operations on applications.
//!
//! This module provides a REST API for managing applications, including:
//! - Listing applications
//! - Creating new applications
//! - Updating existing applications
//! - Getting application details and statistics
//! - Starting and stopping applications
//! - Scaling applications
//! - Deleting applications
//! - Releasing new versions of applications

// Import and re-export all route modules
pub mod types;
pub mod list;
pub mod get;
pub mod create;
pub mod update;
pub mod control;
pub mod delete;
pub mod release;
pub mod instances;

// Re-export types for easier access
pub use types::*;

// Re-export all route functions
pub use list::{list_apps, count_apps};
pub use get::{get_app, get_app_with_instances, get_app_stats};
pub use create::create_app;
pub use update::update_app;
pub use control::{start_app, stop_app, scale_app};
pub use delete::delete_app;
pub use release::create_release;
pub use instances::list_instances;

