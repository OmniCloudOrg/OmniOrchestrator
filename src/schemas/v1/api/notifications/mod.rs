//! Notification management module for handling CRUD operations on notifications.
//! 
//! This module provides functionality to create, read, update, and delete
//! notifications in the system. It includes endpoints for managing both
//! user notifications and role-based notifications.

// Import and re-export all modules
pub mod types;
pub mod user;
pub mod role;
pub mod acknowledge;

// Re-export all route functions
pub use user::{
    list_user_notifications,
    count_unread_user_notifications,
    get_user_notification_by_id,
    create_user_notification,
    mark_user_notification_as_read,
    mark_all_user_notifications_as_read,
    delete_user_notification,
    delete_read_user_notifications,
    get_all_user_notifications_with_count,
};
pub use role::{
    list_role_notifications,
    create_role_notification,
};
pub use acknowledge::acknowledge_notification;