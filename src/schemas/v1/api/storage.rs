use std::sync::Arc;
use crate::DatabaseManager;
use crate::schemas::v1::db::queries::storage;
use rocket::http::Status;
use rocket::serde::json::{json, Json, Value};
use rocket::{get, State};
use crate::schemas::v1::db::queries::{self as db};

/// Query parameters for storage class listing
#[derive(FromForm, Default, Debug)]
pub struct StorageClassQuery {
    pub storage_type: Option<String>,
    pub volume_binding_mode: Option<String>,
    pub allow_volume_expansion: Option<bool>,
}

/// Query parameters for storage volume listing
#[derive(FromForm, Default, Debug)]
pub struct StorageVolumeQuery {
    pub app_id: Option<i64>,
    pub storage_class_id: Option<i64>,
    pub status: Option<String>,
    pub node_id: Option<i64>,
    pub persistence_level: Option<String>,
    pub write_concern: Option<String>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

/// List all storage classes with optional filtering
#[get("/platform/<platform_id>/storage/classes?<query..>")]
pub async fn list_storage_classes(
    platform_id: i64,
    query: StorageClassQuery,
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
    
    let filter = storage::StorageClassFilter {
        storage_type: query.storage_type,
        volume_binding_mode: query.volume_binding_mode,
        allow_volume_expansion: query.allow_volume_expansion,
    };
    
    match storage::list_storage_classes(&pool, filter).await {
        Ok(storage_classes) => Ok(Json(json!({
            "storage_classes": storage_classes
        }))),
        Err(_) => Err((
            Status::InternalServerError,
            Json(json!({
                "error": "Database error",
                "message": "Failed to list storage classes"
            }))
        )),
    }
}

/// Get a specific storage class by ID
#[get("/platform/<platform_id>/storage/classes/<id>")]
pub async fn get_storage_class(
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
    
    match storage::get_storage_class_by_id(&pool, id).await {
        Ok(Some(storage_class)) => Ok(Json(json!({
            "storage_class": storage_class
        }))),
        Ok(None) => Err((
            Status::NotFound,
            Json(json!({
                "error": "Not found",
                "message": format!("Storage class with ID {} does not exist", id)
            }))
        )),
        Err(_) => Err((
            Status::InternalServerError,
            Json(json!({
                "error": "Database error",
                "message": "Failed to get storage class"
            }))
        )),
    }
}

/// List storage volumes with comprehensive filtering
#[get("/platform/<platform_id>/storage/volumes?<query..>")]
pub async fn list_storage_volumes(
    platform_id: i64,
    query: StorageVolumeQuery,
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
    
    let page = query.page.unwrap_or(0);
    let per_page = query.per_page.unwrap_or(10);
    
    let filter = storage::StorageVolumeFilter {
        app_id: query.app_id,
        storage_class_id: query.storage_class_id,
        status: query.status,
        node_id: query.node_id,
        persistence_level: query.persistence_level,
        write_concern: query.write_concern,
    };
    
    let storage_volumes = match storage::list_storage_volumes(&pool, filter.clone(), page, per_page).await {
        Ok(volumes) => volumes,
        Err(_) => {
            return Err((
                Status::InternalServerError,
                Json(json!({
                    "error": "Database error",
                    "message": "Failed to list storage volumes"
                }))
            ));
        }
    };
    
    let total_count = match storage::count_storage_volumes_with_filter(&pool, &filter).await {
        Ok(count) => count,
        Err(_) => {
            return Err((
                Status::InternalServerError,
                Json(json!({
                    "error": "Database error",
                    "message": "Failed to count storage volumes"
                }))
            ));
        }
    };
    
    let total_pages = (total_count as f64 / per_page as f64).ceil() as i64;
    
    Ok(Json(json!({
        "storage_volumes": storage_volumes,
        "pagination": {
            "page": page,
            "per_page": per_page,
            "total_count": total_count,
            "total_pages": total_pages
        }
    })))
}

/// Get volumes by storage class
#[get("/platform/<platform_id>/storage/classes/<id>/volumes?<page>&<per_page>")]
pub async fn get_volumes_by_storage_class(
    platform_id: i64,
    id: i64,
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
    
    // First check if storage class exists
    match storage::get_storage_class_by_id(&pool, id).await {
        Ok(Some(_)) => {},
        Ok(None) => {
            return Err((
                Status::NotFound,
                Json(json!({
                    "error": "Not found",
                    "message": format!("Storage class with ID {} does not exist", id)
                }))
            ));
        },
        Err(_) => {
            return Err((
                Status::InternalServerError,
                Json(json!({
                    "error": "Database error",
                    "message": "Failed to verify storage class existence"
                }))
            ));
        }
    };
    
    let page = page.unwrap_or(0);
    let per_page = per_page.unwrap_or(10);
    
    let volumes = match storage::get_volumes_by_storage_class(&pool, id, page, per_page).await {
        Ok(volumes) => volumes,
        Err(_) => {
            return Err((
                Status::InternalServerError,
                Json(json!({
                    "error": "Database error",
                    "message": "Failed to fetch volumes by storage class"
                }))
            ));
        }
    };
    
    let filter = storage::StorageVolumeFilter {
        storage_class_id: Some(id),
        ..Default::default()
    };
    
    let total_count = match storage::count_storage_volumes_with_filter(&pool, &filter).await {
        Ok(count) => count,
        Err(_) => {
            return Err((
                Status::InternalServerError,
                Json(json!({
                    "error": "Database error",
                    "message": "Failed to count volumes"
                }))
            ));
        }
    };
    
    let total_pages = (total_count as f64 / per_page as f64).ceil() as i64;
    
    Ok(Json(json!({
        "volumes": volumes,
        "pagination": {
            "page": page,
            "per_page": per_page,
            "total_count": total_count,
            "total_pages": total_pages
        }
    })))
}

/// Get QoS policies
#[get("/platform/<platform_id>/storage/qos-policies")]
pub async fn list_qos_policies(
    platform_id: i64,
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
    
    match storage::list_storage_qos_policies(&pool).await {
        Ok(policies) => Ok(Json(json!({
            "qos_policies": policies
        }))),
        Err(_) => Err((
            Status::InternalServerError,
            Json(json!({
                "error": "Database error",
                "message": "Failed to fetch QoS policies"
            }))
        )),
    }
}

/// List volumes by write concern level
#[get("/platform/<platform_id>/storage/write-concerns/<write_concern>/volumes?<page>&<per_page>")]
pub async fn list_volumes_by_write_concern(
    platform_id: i64,
    write_concern: String,
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
    
    let page = page.unwrap_or(0);
    let per_page = per_page.unwrap_or(10);
    
    let volumes = match storage::get_volumes_by_write_concern(&pool, write_concern.clone(), page, per_page).await {
        Ok(volumes) => volumes,
        Err(_) => {
            return Err((
                Status::InternalServerError,
                Json(json!({
                    "error": "Database error",
                    "message": "Failed to fetch volumes by write concern"
                }))
            ));
        }
    };
    
    let filter = storage::StorageVolumeFilter {
        write_concern: Some(write_concern),
        ..Default::default()
    };
    
    let total_count = match storage::count_storage_volumes_with_filter(&pool, &filter).await {
        Ok(count) => count,
        Err(_) => {
            return Err((
                Status::InternalServerError,
                Json(json!({
                    "error": "Database error",
                    "message": "Failed to count volumes"
                }))
            ));
        }
    };
    
    let total_pages = (total_count as f64 / per_page as f64).ceil() as i64;
    
    Ok(Json(json!({
        "volumes": volumes,
        "pagination": {
            "page": page,
            "per_page": per_page,
            "total_count": total_count,
            "total_pages": total_pages
        }
    })))
}

/// List volumes by persistence level
#[get("/platform/<platform_id>/storage/persistence-levels/<persistence_level>/volumes?<page>&<per_page>")]
pub async fn list_volumes_by_persistence_level(
    platform_id: i64,
    persistence_level: String,
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
    
    let page = page.unwrap_or(0);
    let per_page = per_page.unwrap_or(10);
    
    let volumes = match storage::get_volumes_by_persistence_level(&pool, persistence_level.clone(), page, per_page).await {
        Ok(volumes) => volumes,
        Err(_) => {
            return Err((
                Status::InternalServerError,
                Json(json!({
                    "error": "Database error",
                    "message": "Failed to fetch volumes by persistence level"
                }))
            ));
        }
    };
    
    let filter = storage::StorageVolumeFilter {
        persistence_level: Some(persistence_level),
        ..Default::default()
    };
    
    let total_count = match storage::count_storage_volumes_with_filter(&pool, &filter).await {
        Ok(count) => count,
        Err(_) => {
            return Err((
                Status::InternalServerError,
                Json(json!({
                    "error": "Database error",
                    "message": "Failed to count volumes"
                }))
            ));
        }
    };
    
    let total_pages = (total_count as f64 / per_page as f64).ceil() as i64;
    
    Ok(Json(json!({
        "volumes": volumes,
        "pagination": {
            "page": page,
            "per_page": per_page,
            "total_count": total_count,
            "total_pages": total_pages
        }
    })))
}

/// Get storage volumes for a specific region, grouped by region, with pagination
#[get("/platform/<platform_id>/storage/regions/<region_id>/volumes?<page>&<per_page>")]
pub async fn get_volumes_for_region_route(
    platform_id: i64,
    region_id: i64,
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
    
    let page = page.unwrap_or(0);
    let per_page = per_page.unwrap_or(10);

    let region_volumes = match storage::get_volumes_for_region(&pool, region_id, page, per_page).await {
        Ok(volumes) => volumes,
        Err(_) => {
            return Err((
                Status::InternalServerError,
                Json(json!({
                    "error": "Database error",
                    "message": "Failed to fetch volumes for region"
                }))
            ));
        }
    };

    let total_count = match storage::count_volumes_for_region(&pool, region_id).await {
        Ok(count) => count,
        Err(_) => {
            return Err((
                Status::InternalServerError,
                Json(json!({
                    "error": "Database error",
                    "message": "Failed to count volumes for region"
                }))
            ));
        }
    };

    let total_pages = (total_count as f64 / per_page as f64).ceil() as i64;

    Ok(Json(json!({
        "region": region_volumes.region,
        "volumes": region_volumes.volumes,
        "pagination": {
            "page": page,
            "per_page": per_page,
            "total_count": total_count,
            "total_pages": total_pages
        }
    })))
}

/// Get storage volumes for a specific provider, with pagination
#[get("/platform/<platform_id>/storage/providers/<provider_id>/volumes?<page>&<per_page>")]
pub async fn get_storage_volumes_for_provider(
    platform_id: i64,
    provider_id: i64,
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
    
    let page = page.unwrap_or(0);
    let per_page = per_page.unwrap_or(10);

    let volumes = match storage::get_volumes_for_provider(&pool, provider_id, page, per_page).await {
        Ok(volumes) => volumes,
        Err(_) => {
            return Err((
                Status::InternalServerError,
                Json(json!({
                    "error": "Database error",
                    "message": "Failed to fetch volumes for provider"
                }))
            ));
        }
    };

    let total_count = match storage::count_volumes_for_provider(&pool, provider_id).await {
        Ok(count) => count,
        Err(_) => {
            return Err((
                Status::InternalServerError,
                Json(json!({
                    "error": "Database error",
                    "message": "Failed to count volumes for provider"
                }))
            ));
        }
    };

    let total_pages = (total_count as f64 / per_page as f64).ceil() as i64;

    Ok(Json(json!({
        "provider_id": provider_id,
        "volumes": volumes,
        "pagination": {
            "page": page,
            "per_page": per_page,
            "total_count": total_count,
            "total_pages": total_pages
        }
    })))
}