use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Request data for creating a new resource type.
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateResourceTypeRequest {
    /// Name of the resource type
    pub name: String,
    /// Category of the resource
    pub category: String,
    /// Unit of measurement (e.g., 'vCPU-hour', 'GB-month')
    pub unit_of_measurement: String,
    /// Optional description of the resource type
    pub description: Option<String>,
}

/// Request data for updating a resource type.
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateResourceTypeRequest {
    /// New name for the resource type
    pub name: Option<String>,
    /// New category for the resource
    pub category: Option<String>,
    /// New unit of measurement
    pub unit_of_measurement: Option<String>,
    /// New description of the resource type
    pub description: Option<String>,
}

/// Request data for creating a new cost metric.
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCostMetricRequest {
    /// Resource type ID
    pub resource_type_id: i32,
    /// Provider ID
    pub provider_id: Option<i64>,
    /// Region ID
    pub region_id: Option<i64>,
    /// Application ID
    pub app_id: Option<i64>,
    /// Worker ID
    pub worker_id: Option<i64>,
    /// Organization ID
    pub org_id: Option<i64>,
    /// Start time of the usage period
    pub start_time: DateTime<Utc>,
    /// End time of the usage period
    pub end_time: DateTime<Utc>,
    /// Amount of resource used
    pub usage_quantity: f64,
    /// Cost per unit
    pub unit_cost: f64,
    /// Currency code (e.g., 'USD')
    pub currency: String,
    /// Total cost for this usage
    pub total_cost: f64,
    /// Discount percentage applied
    pub discount_percentage: Option<f64>,
    /// Reason for the discount
    pub discount_reason: Option<String>,
    /// Billing period (e.g., '2025-05')
    pub billing_period: Option<String>,
}

/// Request data for filtering cost metrics.
#[derive(Debug, Serialize, Deserialize)]
pub struct CostMetricFilter {
    /// Filter by resource type ID
    pub resource_type_id: Option<i32>,
    /// Filter by provider ID
    pub provider_id: Option<i64>,
    /// Filter by application ID
    pub app_id: Option<i64>,
    /// Filter by start date
    pub start_date: Option<DateTime<Utc>>,
    /// Filter by end date
    pub end_date: Option<DateTime<Utc>>,
    /// Filter by billing period
    pub billing_period: Option<String>,
}

/// Request data for creating a new cost budget.
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCostBudgetRequest {
    /// Organization ID
    pub org_id: i64,
    /// Application ID (optional)
    pub app_id: Option<i64>,
    /// Budget name
    pub budget_name: String,
    /// Budget amount
    pub budget_amount: f64,
    /// Currency code (e.g., 'USD')
    pub currency: String,
    /// Budget period type
    pub budget_period: String,
    /// Start date of the budget period
    pub period_start: DateTime<Utc>,
    /// End date of the budget period
    pub period_end: DateTime<Utc>,
    /// Alert threshold percentage
    pub alert_threshold_percentage: f64,
    /// Contacts to alert when threshold is reached (JSON)
    pub alert_contacts: String,
}

/// Request data for updating a cost budget.
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateCostBudgetRequest {
    /// New budget name
    pub budget_name: Option<String>,
    /// New budget amount
    pub budget_amount: Option<f64>,
    /// New alert threshold percentage
    pub alert_threshold_percentage: Option<f64>,
    /// New contacts to alert when threshold is reached (JSON)
    pub alert_contacts: Option<String>,
    /// Whether the budget is active
    pub is_active: Option<bool>,
}

/// Request data for creating a new cost projection.
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCostProjectionRequest {
    /// Organization ID
    pub org_id: i64,
    /// Application ID (optional)
    pub app_id: Option<i64>,
    /// Projection period type (e.g., 'monthly', 'quarterly')
    pub projection_period: String,
    /// Start date of the projection period
    pub start_date: DateTime<Utc>,
    /// End date of the projection period
    pub end_date: DateTime<Utc>,
    /// Projected cost amount
    pub projected_cost: f64,
    /// Currency code (e.g., 'USD')
    pub currency: String,
    /// Projection model used (e.g., 'linear', 'average_30d')
    pub projection_model: String,
    /// Confidence level of the projection
    pub confidence_level: Option<f64>,
    /// Additional metadata about the projection (JSON)
    pub metadata: Option<String>,
}

/// Request data for creating a new resource pricing entry.
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateResourcePricingRequest {
    /// Resource type ID
    pub resource_type_id: i32,
    /// Provider ID
    pub provider_id: i64,
    /// Region ID (optional)
    pub region_id: Option<i64>,
    /// Tier name (e.g., 'standard', 'premium')
    pub tier_name: String,
    /// Price per unit
    pub unit_price: f64,
    /// Currency code (e.g., 'USD')
    pub currency: String,
    /// When this pricing becomes effective
    pub effective_from: DateTime<Utc>,
    /// When this pricing expires (optional)
    pub effective_to: Option<DateTime<Utc>>,
    /// Pricing model (e.g., 'on-demand', 'reserved')
    pub pricing_model: String,
    /// Commitment period (e.g., '1-year', '3-year')
    pub commitment_period: Option<String>,
    /// Volume discount tiers (JSON)
    pub volume_discount_tiers: Option<String>,
}

/// Request data for updating a resource pricing entry.
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateResourcePricingRequest {
    /// New price per unit
    pub unit_price: Option<f64>,
    /// New expiration date
    pub effective_to: Option<DateTime<Utc>>,
    /// New volume discount tiers (JSON)
    pub volume_discount_tiers: Option<String>,
}

/// Request data for creating a new cost allocation tag.
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCostAllocationTagRequest {
    /// Tag key
    pub tag_key: String,
    /// Tag value
    pub tag_value: String,
    /// Resource ID
    pub resource_id: i64,
    /// Resource type
    pub resource_type: String,
}

/// Request data for aggregate cost analysis by dimension.
#[derive(Debug, Serialize, Deserialize)]
pub struct CostAnalysisByDimensionRequest {
    /// Dimension to group by
    pub dimension: String,
    /// Start date for analysis
    pub start_date: DateTime<Utc>,
    /// End date for analysis
    pub end_date: DateTime<Utc>,
    /// Maximum number of results to return
    pub limit: i64,
}

/// Request data for cost analysis over time.
#[derive(Debug, Serialize, Deserialize)]
pub struct CostOverTimeRequest {
    /// Application ID to analyze
    pub app_id: i64,
    /// Time interval ('day', 'week', 'month')
    pub interval: String,
    /// Start date for analysis
    pub start_date: DateTime<Utc>,
    /// End date for analysis
    pub end_date: DateTime<Utc>,
}