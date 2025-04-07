// backup/coordinator/mod.rs
//
// Main module file for the OmniCloud backup coordinator system

mod types;
mod coordinator;
mod system_core;
mod director;
mod orchestrator;
mod network;
mod app_definitions;
mod volume_data;
mod manifest;

// Re-export the types and main coordinator
pub use types::BackupJobStatus;
pub use coordinator::BackupCoordinator;

// Internal crate-level exports
pub(crate) use system_core::backup_system_core;
pub(crate) use director::backup_director;
pub(crate) use orchestrator::backup_orchestrator;
pub(crate) use network::backup_network_config;
pub(crate) use app_definitions::backup_app_definitions;
pub(crate) use volume_data::backup_volume_data;
pub(crate) use manifest::create_backup_manifest;