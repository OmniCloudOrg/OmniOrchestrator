//! Permission management module for handling permission operations.
//!
//! This module provides a REST API for managing permissions, including:
//! - Listing all permissions
//! - Getting permission details by ID
//! - Creating new permissions
//! - Deleting permissions

// Import and re-export all modules
pub mod list;
pub mod get;
pub mod create;
pub mod delete;

// Re-export all route functions
pub use list::list_permission;
pub use get::get_permission_by_id;
pub use create::create_permission;
pub use delete::delete_permission;