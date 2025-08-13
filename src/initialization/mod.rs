//! Initialization utilities for OmniOrchestrator
//!
//! This module provides setup routines for logging, database connections, ClickHouse integration,
//! schema initialization, and authentication configuration. Each function is designed to be called
//! during application startup to ensure all subsystems are properly initialized and ready for use.
//!
//! # Functions
//! - `setup_logging`: Initializes the logger with colored output and info-level filtering.
//! - `setup_database`: Connects to the deployment database, registers platforms, and pre-initializes pools.
//! - `setup_clickhouse`: Establishes a connection to ClickHouse and validates connectivity.
//! - `setup_schema`: Loads and initializes the ClickHouse schema from SQL files.
//! - `create_auth_config`: Constructs the authentication config from environment variables.

pub mod launch_server;
pub mod setup_logging;
pub mod setup_database;
pub mod setup_clickhouse;
pub mod setup_schema;
pub mod create_auth_config;
pub mod start_peer_discovery;
pub mod setup_cluster_management;
pub mod start_leader_election;

pub use launch_server::launch_server;
pub use setup_logging::setup_logging;
pub use setup_database::setup_database;
pub use setup_clickhouse::setup_clickhouse;
pub use setup_schema::setup_schema;
pub use create_auth_config::create_auth_config;
pub use start_peer_discovery::start_peer_discovery;
pub use setup_cluster_management::setup_cluster_management;
pub use start_leader_election::start_leader_election;