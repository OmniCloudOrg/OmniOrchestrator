//! Cost management module for handling cost tracking and analysis operations.
//!
//! This module provides a REST API for managing cost-related entities, including:
//! - Resource types management
//! - Cost metrics tracking and analysis
//! - Cost projections and forecasting
//! - Budget management
//! - Resource pricing management
//! - Cost allocation tagging

// Import and re-export all route modules
pub mod types;
pub mod resource_types;
pub mod metrics;
pub mod analysis;
pub mod budgets;
pub mod projections;
pub mod pricing;
pub mod allocation_tags;

// Re-export types for easier access
pub use types::*;

// Re-export all route functions
pub use resource_types::{
    list_resource_types, 
    count_resource_types, 
    get_resource_type, 
    create_resource_type, 
    update_resource_type, 
    delete_resource_type
};
pub use metrics::{
    list_cost_metrics, 
    get_cost_metric, 
    create_cost_metric, 
    delete_cost_metric
};
pub use analysis::{
    analyze_costs_by_dimension, 
    analyze_cost_over_time
};
pub use budgets::{
    list_cost_budgets, 
    get_cost_budget, 
    create_cost_budget, 
    update_cost_budget, 
    delete_cost_budget
};
pub use projections::{
    list_cost_projections, 
    get_cost_projection, 
    create_cost_projection, 
    delete_cost_projection
};
pub use pricing::{
    list_resource_pricing, 
    get_resource_pricing, 
    create_resource_pricing, 
    update_resource_pricing, 
    delete_resource_pricing
};
pub use allocation_tags::{
    get_cost_allocation_tags, 
    create_cost_allocation_tag, 
    delete_cost_allocation_tag
};