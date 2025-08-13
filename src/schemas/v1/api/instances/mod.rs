//! Instance management module for handling instance operations.
//!
//! This module provides a REST API for managing instances, including:
//! - Listing instances by region with pagination
//! - Getting instance details by ID
//! - Counting total instances

// Import and re-export all modules
pub mod list;
pub mod get;
pub mod count;

// Re-export all route functions
pub use list::list_instances_by_region;
pub use get::get_instance;
pub use count::count_instances;