// backup/mod.rs
//
// Main module file for the OmniCloud backup system

mod model;
mod coordinator;
mod iso;
mod validation;
mod export;
mod recovery;

pub use model::Backup;
pub use coordinator::BackupCoordinator;
pub use validation::BackupValidator;
pub use export::BackupExporter;
pub use iso::IsoManager;

// Re-export types used throughout the module
pub use coordinator::BackupJobStatus;