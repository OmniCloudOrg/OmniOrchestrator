use crate::schemas::v1::db::queries::storage;
use rocket::http::Status;
use rocket::serde::json::{json, Json, Value};
use rocket::{get, State};
use sqlx::MySql;

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
#[get("/storage/classes?<query..>")]
pub async fn list_storage_classes(
    pool: &State<sqlx::Pool<MySql>>,
    query: StorageClassQuery,
) -> Json<Value> {
    let filter = storage::StorageClassFilter {
        storage_type: query.storage_type,
        volume_binding_mode: query.volume_binding_mode,
        allow_volume_expansion: query.allow_volume_expansion,
    };
    
    let storage_classes = storage::list_storage_classes(pool, filter)
        .await
        .expect("Failed to list storage classes");
    
    Json(json!({
        "storage_classes": storage_classes
    }))
}

/// Get a specific storage class by ID
#[get("/storage/classes/<id>")]
pub async fn get_storage_class(
    pool: &State<sqlx::Pool<MySql>>,
    id: i64,
) -> Result<Json<Value>, Status> {
    let storage_class = storage::get_storage_class_by_id(pool, id)
        .await
        .expect("Database error")
        .ok_or(Status::NotFound)?;
    
    Ok(Json(json!({
        "storage_class": storage_class
    })))
}

/// List storage volumes with comprehensive filtering
#[get("/storage/volumes?<query..>")]
pub async fn list_storage_volumes(
    pool: &State<sqlx::Pool<MySql>>,
    query: StorageVolumeQuery,
) -> Json<Value> {
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
    
    let storage_volumes = storage::list_storage_volumes(pool, filter.clone(), page, per_page)
        .await
        .expect("Failed to list storage volumes");
    
    let total_count = storage::count_storage_volumes_with_filter(pool, &filter)
        .await
        .expect("Failed to get total count of storage volumes");
    
    let total_pages = (total_count as f64 / per_page as f64).ceil() as i64;
    
    Json(json!({
        "storage_volumes": storage_volumes,
        "pagination": {
            "page": page,
            "per_page": per_page,
            "total_count": total_count,
            "total_pages": total_pages
        }
    }))
}

/// Get volumes by storage class
#[get("/storage/classes/<id>/volumes?<page>&<per_page>")]
pub async fn get_volumes_by_storage_class(
    pool: &State<sqlx::Pool<MySql>>,
    id: i64,
    page: Option<i64>,
    per_page: Option<i64>,
) -> Result<Json<Value>, Status> {
    // First check if storage class exists
    storage::get_storage_class_by_id(pool, id)
        .await
        .expect("Database error")
        .ok_or(Status::NotFound)?;
    
    let page = page.unwrap_or(0);
    let per_page = per_page.unwrap_or(10);
    
    let volumes = storage::get_volumes_by_storage_class(pool, id, page, per_page)
        .await
        .expect("Failed to fetch volumes by storage class");
    
    let filter = storage::StorageVolumeFilter {
        storage_class_id: Some(id),
        ..Default::default()
    };
    
    let total_count = storage::count_storage_volumes_with_filter(pool, &filter)
        .await
        .expect("Failed to count volumes");
    
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
#[get("/storage/qos-policies")]
pub async fn list_qos_policies(
    pool: &State<sqlx::Pool<MySql>>,
) -> Json<Value> {
    let policies = storage::list_storage_qos_policies(pool)
        .await
        .expect("Failed to fetch QoS policies");
    
    Json(json!({
        "qos_policies": policies
    }))
}

/// List volumes by write concern level
#[get("/storage/write-concerns/<write_concern>/volumes?<page>&<per_page>")]
pub async fn list_volumes_by_write_concern(
    pool: &State<sqlx::Pool<MySql>>,
    write_concern: String,
    page: Option<i64>,
    per_page: Option<i64>,
) -> Json<Value> {
    let page = page.unwrap_or(0);
    let per_page = per_page.unwrap_or(10);
    
    let volumes = storage::get_volumes_by_write_concern(pool, write_concern.clone(), page, per_page)
        .await
        .expect("Failed to fetch volumes by write concern");
    
    let filter = storage::StorageVolumeFilter {
        write_concern: Some(write_concern),
        ..Default::default()
    };
    
    let total_count = storage::count_storage_volumes_with_filter(pool, &filter)
        .await
        .expect("Failed to count volumes");
    
    let total_pages = (total_count as f64 / per_page as f64).ceil() as i64;
    
    Json(json!({
        "volumes": volumes,
        "pagination": {
            "page": page,
            "per_page": per_page,
            "total_count": total_count,
            "total_pages": total_pages
        }
    }))
}

/// List volumes by persistence level
#[get("/storage/persistence-levels/<persistence_level>/volumes?<page>&<per_page>")]
pub async fn list_volumes_by_persistence_level(
    pool: &State<sqlx::Pool<MySql>>,
    persistence_level: String,
    page: Option<i64>,
    per_page: Option<i64>,
) -> Json<Value> {
    let page = page.unwrap_or(0);
    let per_page = per_page.unwrap_or(10);
    
    let volumes = storage::get_volumes_by_persistence_level(pool, persistence_level.clone(), page, per_page)
        .await
        .expect("Failed to fetch volumes by persistence level");
    
    let filter = storage::StorageVolumeFilter {
        persistence_level: Some(persistence_level),
        ..Default::default()
    };
    
    let total_count = storage::count_storage_volumes_with_filter(pool, &filter)
        .await
        .expect("Failed to count volumes");
    
    let total_pages = (total_count as f64 / per_page as f64).ceil() as i64;
    
    Json(json!({
        "volumes": volumes,
        "pagination": {
            "page": page,
            "per_page": per_page,
            "total_count": total_count,
            "total_pages": total_pages
        }
    }))
}

/// Get storage volumes for a specific region, grouped by region, with pagination
#[get("/storage/regions/<region_id>/volumes?<page>&<per_page>")]
pub async fn get_volumes_for_region_route(
    pool: &State<sqlx::Pool<MySql>>,
    region_id: i64,
    page: Option<i64>,
    per_page: Option<i64>,
) -> Result<Json<Value>, Status> {
    let page = page.unwrap_or(0);
    let per_page = per_page.unwrap_or(10);

    let region_volumes = storage::get_volumes_for_region(pool, region_id, page, per_page)
        .await
        .map_err(|_| Status::InternalServerError)?;

    let total_count = storage::count_volumes_for_region(pool, region_id)
        .await
        .map_err(|_| Status::InternalServerError)?;

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
#[get("/storage/providers/<provider_id>/volumes?<page>&<per_page>")]
pub async fn get_storage_volumes_for_provider(
    pool: &State<sqlx::Pool<MySql>>,
    provider_id: i64,
    page: Option<i64>,
    per_page: Option<i64>,
) -> Result<Json<Value>, Status> {
    let page = page.unwrap_or(0);
    let per_page = per_page.unwrap_or(10);

    let volumes = storage::get_volumes_for_provider(pool, provider_id, page, per_page)
        .await
        .map_err(|_| Status::InternalServerError)?;

    let total_count = storage::count_volumes_for_provider(pool, provider_id)
        .await
        .map_err(|_| Status::InternalServerError)?;

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