//! Alert management module for handling CRUD operations on alerts.
//! 
//! This module provides functionality to create, read, update, and delete
//! alerts in the system. It includes endpoints for managing alerts
//! associated with applications and organizations.

// Import and re-export all route modules
pub mod types;
pub mod list;
pub mod get;
pub mod create;
pub mod update;
pub mod actions;
pub mod app_alerts;
pub mod org_alerts;
pub mod escalation;
pub mod auto_resolve;
pub mod search;
pub mod bulk;

// Re-export types for easier access
pub use types::*;

// Re-export all route functions
pub use list::list_alerts;
pub use get::get_alert;
pub use create::create_alert;
pub use update::update_alert_status;
pub use actions::{acknowledge_alert, resolve_alert, escalate_alert};
pub use app_alerts::get_app_alerts;
pub use org_alerts::{get_org_active_alerts, get_org_alert_stats};
pub use escalation::get_alerts_needing_escalation;
pub use auto_resolve::auto_resolve_old_alerts;
pub use search::search_alerts;
pub use bulk::bulk_update_alert_status;