use super::super::super::super::auth::User;
use super::super::super::db::queries as db;
use super::types::{CreateCostBudgetRequest, UpdateCostBudgetRequest};
use rocket::http::Status;
use rocket::serde::json::{json, Json, Value};
use rocket::{delete, get, post, put, State};
use std::sync::Arc;
use crate::DatabaseManager;

use libomni::types::db::v1 as types;
use types::cost::CostBudget;

/// List all cost budgets with pagination support.
#[get("/platform/<platform_id>/cost_budgets?<page>&<per_page>")]
pub async fn list_cost_budgets(
    platform_id: i64,
    page: Option<i64>,
    per_page: Option<i64>,
    db_manager: &State<Arc<DatabaseManager>>,
) -> Result<Json<Value>, (Status, Json<Value>)> {
    // Get platform information
    let platform = match db::platforms::get_platform_by_id(db_manager.get_main_pool(), platform_id).await {
        Ok(platform) => platform,
        Err(_) => {
            return Err((
                Status::NotFound,
                Json(json!({
                    "error": "Platform not found",
                    "message": format!("Platform with ID {} does not exist", platform_id)
                }))
            ));
        }
    };

    // Get platform-specific database pool
    let pool = match db_manager.get_platform_pool(&platform.name, platform_id).await {
        Ok(pool) => pool,
        Err(_) => {
            return Err((
                Status::InternalServerError,
                Json(json!({
                    "error": "Database error",
                    "message": "Failed to connect to platform database"
                }))
            ));
        }
    };

    match (page, per_page) {
        (Some(p), Some(pp)) => {
            let cost_budgets = match db::cost::list_cost_budgets(&pool, p, pp).await {
                Ok(budgets) => budgets,
                Err(_) => {
                    return Err((
                        Status::InternalServerError,
                        Json(json!({
                            "error": "Database error",
                            "message": "Failed to retrieve cost budgets"
                        }))
                    ));
                }
            };
            
            let total_count = match db::cost::count_cost_budgets(&pool).await {
                Ok(count) => count,
                Err(_) => {
                    return Err((
                        Status::InternalServerError,
                        Json(json!({
                            "error": "Database error",
                            "message": "Failed to count cost budgets"
                        }))
                    ));
                }
            };
            
            let total_pages = (total_count as f64 / pp as f64).ceil() as i64;

            let response = json!({
                "cost_budgets": cost_budgets,
                "pagination": {
                    "page": p,
                    "per_page": pp,
                    "total_count": total_count,
                    "total_pages": total_pages
                }
            });

            Ok(Json(response))
        }
        _ => Err((
            Status::BadRequest,
            Json(json!({
                "error": "Missing pagination parameters",
                "message": "Please provide both 'page' and 'per_page' parameters"
            }))
        ))
    }
}

/// Get a specific cost budget by ID.
#[get("/platform/<platform_id>/cost_budgets/<id>")]
pub async fn get_cost_budget(
    platform_id: i64,
    id: i64,
    db_manager: &State<Arc<DatabaseManager>>,
) -> Result<Json<CostBudget>, (Status, Json<Value>)> {
    // Get platform information
    let platform = match db::platforms::get_platform_by_id(db_manager.get_main_pool(), platform_id).await {
        Ok(platform) => platform,
        Err(_) => {
            return Err((
                Status::NotFound,
                Json(json!({
                    "error": "Platform not found",
                    "message": format!("Platform with ID {} does not exist", platform_id)
                }))
            ));
        }
    };

    // Get platform-specific database pool
    let pool = match db_manager.get_platform_pool(&platform.name, platform_id).await {
        Ok(pool) => pool,
        Err(_) => {
            return Err((
                Status::InternalServerError,
                Json(json!({
                    "error": "Database error",
                    "message": "Failed to connect to platform database"
                }))
            ));
        }
    };

    match db::cost::get_cost_budget_by_id(&pool, id).await {
        Ok(budget) => Ok(Json(budget)),
        Err(_) => Err((
            Status::NotFound,
            Json(json!({
                "error": "Cost budget not found",
                "message": format!("Cost budget with ID {} could not be found", id)
            }))
        )),
    }
}

/// Create a new cost budget.
#[post("/platform/<platform_id>/cost_budgets", format = "json", data = "<request>")]
pub async fn create_cost_budget(
    platform_id: i64,
    request: Json<CreateCostBudgetRequest>,
    db_manager: &State<Arc<DatabaseManager>>,
    user: User,
) -> Result<Json<CostBudget>, (Status, Json<Value>)> {
    // Get platform information
    let platform = match db::platforms::get_platform_by_id(db_manager.get_main_pool(), platform_id).await {
        Ok(platform) => platform,
        Err(_) => {
            return Err((
                Status::NotFound,
                Json(json!({
                    "error": "Platform not found",
                    "message": format!("Platform with ID {} does not exist", platform_id)
                }))
            ));
        }
    };

    // Get platform-specific database pool
    let pool = match db_manager.get_platform_pool(&platform.name, platform_id).await {
        Ok(pool) => pool,
        Err(_) => {
            return Err((
                Status::InternalServerError,
                Json(json!({
                    "error": "Database error",
                    "message": "Failed to connect to platform database"
                }))
            ));
        }
    };

    let user_id = user.id;

    //TODO: Validate user permissions here later

    match db::cost::create_cost_budget(
        &pool,
        request.org_id,
        request.app_id,
        &request.budget_name,
        request.budget_amount,
        &request.currency,
        &request.budget_period,
        request.period_start,
        request.period_end,
        request.alert_threshold_percentage,
        &request.alert_contacts,
        user_id,
    ).await {
        Ok(budget) => Ok(Json(budget)),
        Err(e) => Err((
            Status::InternalServerError,
            Json(json!({
                "error": "Failed to create cost budget",
                "message": format!("{}", e)
            }))
        )),
    }
}

/// Update an existing cost budget.
#[put("/platform/<platform_id>/cost_budgets/<id>", format = "json", data = "<request>")]
pub async fn update_cost_budget(
    platform_id: i64,
    id: i64,
    request: Json<UpdateCostBudgetRequest>,
    db_manager: &State<Arc<DatabaseManager>>,
) -> Result<Json<CostBudget>, (Status, Json<Value>)> {
    // Get platform information
    let platform = match db::platforms::get_platform_by_id(db_manager.get_main_pool(), platform_id).await {
        Ok(platform) => platform,
        Err(_) => {
            return Err((
                Status::NotFound,
                Json(json!({
                    "error": "Platform not found",
                    "message": format!("Platform with ID {} does not exist", platform_id)
                }))
            ));
        }
    };

    // Get platform-specific database pool
    let pool = match db_manager.get_platform_pool(&platform.name, platform_id).await {
        Ok(pool) => pool,
        Err(_) => {
            return Err((
                Status::InternalServerError,
                Json(json!({
                    "error": "Database error",
                    "message": "Failed to connect to platform database"
                }))
            ));
        }
    };

    match db::cost::update_cost_budget(
        &pool,
        id,
        request.budget_name.as_deref(),
        request.budget_amount,
        request.alert_threshold_percentage,
        request.alert_contacts.as_deref(),
        request.is_active,
    ).await {
        Ok(budget) => Ok(Json(budget)),
        Err(e) => Err((
            Status::InternalServerError,
            Json(json!({
                "error": "Failed to update cost budget",
                "message": format!("{}", e)
            }))
        )),
    }
}

/// Delete a cost budget.
#[delete("/platform/<platform_id>/cost_budgets/<id>")]
pub async fn delete_cost_budget(
    platform_id: i64,
    id: i64,
    db_manager: &State<Arc<DatabaseManager>>,
) -> Result<Json<Value>, (Status, Json<Value>)> {
    // Get platform information
    let platform = match db::platforms::get_platform_by_id(db_manager.get_main_pool(), platform_id).await {
        Ok(platform) => platform,
        Err(_) => {
            return Err((
                Status::NotFound,
                Json(json!({
                    "error": "Platform not found",
                    "message": format!("Platform with ID {} does not exist", platform_id)
                }))
            ));
        }
    };

    // Get platform-specific database pool
    let pool = match db_manager.get_platform_pool(&platform.name, platform_id).await {
        Ok(pool) => pool,
        Err(_) => {
            return Err((
                Status::InternalServerError,
                Json(json!({
                    "error": "Database error",
                    "message": "Failed to connect to platform database"
                }))
            ));
        }
    };

    match db::cost::delete_cost_budget(&pool, id).await {
        Ok(_) => Ok(Json(json!({ "status": "deleted" }))),
        Err(e) => Err((
            Status::InternalServerError,
            Json(json!({
                "error": "Failed to delete cost budget",
                "message": format!("{}", e)
            }))
        )),
    }
}