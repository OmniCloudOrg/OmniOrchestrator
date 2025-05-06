use super::super::tables::{
    Provider,
    Region,
    StorageClass,
    StorageVolume,
    StorageSnapshot,
    StorageQosPolicy,
    StorageMigration,
};
use anyhow::Context;
use sqlx::{MySql, Pool};
use sqlx::Row;

/// Storage class query filters
#[derive(Default, Debug)]
pub struct StorageClassFilter {
    pub storage_type: Option<String>,
    pub volume_binding_mode: Option<String>,
    pub allow_volume_expansion: Option<bool>,
}

/// Storage volume query filters
#[derive(Default, Debug, Clone)]
pub struct StorageVolumeFilter {
    pub app_id: Option<i64>,
    pub storage_class_id: Option<i64>,
    pub status: Option<String>,
    pub node_id: Option<i64>,
    pub persistence_level: Option<String>,
    pub write_concern: Option<String>,
}

/// Retrieves all storage classes with optional filtering
pub async fn list_storage_classes(
    pool: &Pool<MySql>,
    filter: StorageClassFilter,
) -> anyhow::Result<Vec<StorageClass>> {
    let mut query_builder = sqlx::QueryBuilder::new("SELECT * FROM storage_classes WHERE 1=1");
    
    if let Some(storage_type) = filter.storage_type {
        query_builder.push(" AND storage_type = ");
        query_builder.push_bind(storage_type);
    }
    
    if let Some(binding_mode) = filter.volume_binding_mode {
        query_builder.push(" AND volume_binding_mode = ");
        query_builder.push_bind(binding_mode);
    }
    
    if let Some(allow_expansion) = filter.allow_volume_expansion {
        query_builder.push(" AND allow_volume_expansion = ");
        query_builder.push_bind(allow_expansion);
    }
    
    let query = query_builder.build_query_as::<StorageClass>();
    let storage_classes = query
        .fetch_all(pool)
        .await
        .context("Failed to fetch storage classes")?;
    
    Ok(storage_classes)
}

/// Retrieves a single storage class by ID
pub async fn get_storage_class_by_id(
    pool: &Pool<MySql>,
    id: i64,
) -> anyhow::Result<Option<StorageClass>> {
    let storage_class = sqlx::query_as::<_, StorageClass>(
        "SELECT * FROM storage_classes WHERE id = ?"
    )
    .bind(id)
    .fetch_optional(pool)
    .await
    .context("Failed to fetch storage class")?;
    
    Ok(storage_class)
}

/// Retrieves a paginated list of storage volumes with filtering
pub async fn list_storage_volumes(
    pool: &Pool<MySql>,
    filter: StorageVolumeFilter,
    page: i64,
    per_page: i64,
) -> anyhow::Result<Vec<StorageVolume>> {
    let offset = page * per_page;
    
    let mut query_builder = sqlx::QueryBuilder::new("SELECT * FROM storage_volumes WHERE 1=1");
    
    if let Some(app_id) = filter.app_id {
        query_builder.push(" AND app_id = ");
        query_builder.push_bind(app_id);
    }
    
    if let Some(storage_class_id) = filter.storage_class_id {
        query_builder.push(" AND storage_class_id = ");
        query_builder.push_bind(storage_class_id);
    }
    
    if let Some(status) = &filter.status {
        query_builder.push(" AND status = ");
        query_builder.push_bind(status);
    }
    
    if let Some(node_id) = filter.node_id {
        query_builder.push(" AND node_id = ");
        query_builder.push_bind(node_id);
    }
    
    if let Some(persistence_level) = &filter.persistence_level {
        query_builder.push(" AND persistence_level = ");
        query_builder.push_bind(persistence_level);
    }
    
    if let Some(write_concern) = &filter.write_concern {
        query_builder.push(" AND write_concern = ");
        query_builder.push_bind(write_concern);
    }
    
    query_builder.push(" LIMIT ");
    query_builder.push_bind(per_page);
    query_builder.push(" OFFSET ");
    query_builder.push_bind(offset);
    
    let query = query_builder.build_query_as::<StorageVolume>();
    let storage_volumes = query
        .fetch_all(pool)
        .await
        .context("Failed to fetch storage volumes")?;
    
    Ok(storage_volumes)
}

/// Counts storage volumes with the same filtering options
pub async fn count_storage_volumes_with_filter(
    pool: &Pool<MySql>,
    filter: &StorageVolumeFilter,
) -> anyhow::Result<i64> {
    let mut query_builder = sqlx::QueryBuilder::new("SELECT COUNT(*) FROM storage_volumes WHERE 1=1");
    
    if let Some(app_id) = filter.app_id {
        query_builder.push(" AND app_id = ");
        query_builder.push_bind(app_id);
    }
    
    if let Some(storage_class_id) = filter.storage_class_id {
        query_builder.push(" AND storage_class_id = ");
        query_builder.push_bind(storage_class_id);
    }
    
    if let Some(status) = &filter.status {
        query_builder.push(" AND status = ");
        query_builder.push_bind(status);
    }
    
    if let Some(node_id) = filter.node_id {
        query_builder.push(" AND node_id = ");
        query_builder.push_bind(node_id);
    }
    
    if let Some(persistence_level) = &filter.persistence_level {
        query_builder.push(" AND persistence_level = ");
        query_builder.push_bind(persistence_level);
    }
    
    if let Some(write_concern) = &filter.write_concern {
        query_builder.push(" AND write_concern = ");
        query_builder.push_bind(write_concern);
    }
    
    let query = query_builder.build_query_as::<(i64,)>();
    let (count,) = query
        .fetch_one(pool)
        .await
        .context("Failed to count storage volumes")?;
    
    Ok(count)
}

/// Get volumes by storage class
pub async fn get_volumes_by_storage_class(
    pool: &Pool<MySql>,
    storage_class_id: i64,
    page: i64,
    per_page: i64,
) -> anyhow::Result<Vec<StorageVolume>> {
    let offset = page * per_page;
    let query = "SELECT * FROM storage_volumes WHERE storage_class_id = ? LIMIT ? OFFSET ?";
    
    let volumes = sqlx::query_as::<_, StorageVolume>(query)
        .bind(storage_class_id)
        .bind(per_page)
        .bind(offset)
        .fetch_all(pool)
        .await
        .context("Failed to fetch volumes by storage class")?;
    
    Ok(volumes)
}

/// Get QoS policies
pub async fn list_storage_qos_policies(
    pool: &Pool<MySql>,
) -> anyhow::Result<Vec<StorageQosPolicy>> {
    let policies = sqlx::query_as::<_, StorageQosPolicy>(
        "SELECT * FROM storage_qos_policies"
    )
    .fetch_all(pool)
    .await
    .context("Failed to fetch storage QoS policies")?;
    
    Ok(policies)
}

/// Get storage with specified write concern
pub async fn get_volumes_by_write_concern(
    pool: &Pool<MySql>,
    write_concern: String,
    page: i64,
    per_page: i64,
) -> anyhow::Result<Vec<StorageVolume>> {
    let offset = page * per_page;
    let query = "SELECT * FROM storage_volumes WHERE write_concern = ? LIMIT ? OFFSET ?";
    
    let volumes = sqlx::query_as::<_, StorageVolume>(query)
        .bind(write_concern)
        .bind(per_page)
        .bind(offset)
        .fetch_all(pool)
        .await
        .context("Failed to fetch volumes by write concern")?;
    
    Ok(volumes)
}

/// Get volumes with specific persistence level
pub async fn get_volumes_by_persistence_level(
    pool: &Pool<MySql>,
    persistence_level: String,
    page: i64, 
    per_page: i64,
) -> anyhow::Result<Vec<StorageVolume>> {
    let offset = page * per_page;
    let query = "SELECT * FROM storage_volumes WHERE persistence_level = ? LIMIT ? OFFSET ?";
    
    let volumes = sqlx::query_as::<_, StorageVolume>(query)
        .bind(persistence_level)
        .bind(per_page)
        .bind(offset)
        .fetch_all(pool)
        .await
        .context("Failed to fetch volumes by persistence level")?;
    
    Ok(volumes)
}

/// Struct to represent a Region with its storage volumes
#[derive(Debug, serde::Serialize)]
pub struct RegionVolumes {
    pub region: Region,
    pub volumes: Vec<StorageVolume>
}

/// Retrieves storage volumes for a specific region grouped by region with pagination
pub async fn get_volumes_for_region(
    pool: &Pool<MySql>,
    region_id: i64,
    page: i64,
    per_page: i64,
) -> anyhow::Result<RegionVolumes> {
    // First, get the region
    let region = sqlx::query_as::<_, Region>("SELECT * FROM regions WHERE id = ?")
        .bind(region_id)
        .fetch_one(pool)
        .await
        .context("Failed to fetch region")?;
    
    // Calculate offset
    let offset = page * per_page;
    
    // Get paginated volumes for this region
    let volumes = sqlx::query_as::<_, StorageVolume>(
        r#"
        SELECT
            v.*
        FROM 
            storage_volumes v
        INNER JOIN 
            workers w ON v.node_id = w.id
        WHERE 
            w.region_id = ?
        ORDER BY 
            v.id
        LIMIT ? OFFSET ?
        "#
    )
    .bind(region_id)
    .bind(per_page)
    .bind(offset)
    .fetch_all(pool)
    .await
    .context("Failed to fetch volumes for region")?;
    
    Ok(RegionVolumes {
        region,
        volumes
    })
}

/// Counts the total number of storage volumes for a specific region
pub async fn count_volumes_for_region(
    pool: &Pool<MySql>,
    region_id: i64,
) -> anyhow::Result<i64> {
    // Get the total count of volumes for this region
    let (total_volumes,) = sqlx::query_as::<_, (i64,)>(
        r#"
        SELECT
            COUNT(v.id)
        FROM 
            storage_volumes v
        INNER JOIN 
            workers w ON v.node_id = w.id
        INNER JOIN 
            nodes n ON w.id = n.worker_id
        WHERE 
            w.region_id = ?
        "#
    )
    .bind(region_id)
    .fetch_one(pool)
    .await
    .context("Failed to count volumes for region")?;
    
    Ok(total_volumes)
}

#[derive(serde::Serialize)]
pub struct ProviderRegionVolumes {
    pub provider: Provider,
    pub regions: Vec<RegionVolumes>
}

/// Retrieves storage volumes for a specific provider grouped by region with pagination
pub async fn get_volumes_for_provider(
    pool: &Pool<MySql>,
    provider_id: i64,
    page: i64,
    per_page: i64,
) -> anyhow::Result<ProviderRegionVolumes> {
    // First, get the provider
    let provider = sqlx::query_as::<_, Provider>("SELECT * FROM providers WHERE id = ?")
        .bind(provider_id)
        .fetch_one(pool)
        .await
        .context("Failed to fetch provider")?;
    
    // Get all regions for this provider
    let regions = sqlx::query_as::<_, Region>(
        "SELECT * FROM regions WHERE provider = ? ORDER BY name"
    )
    .bind(provider_id)
    .fetch_all(pool)
    .await
    .context("Failed to fetch regions for provider")?;
    
    let mut region_volumes = Vec::new();
    
    // Calculate offset
    let offset = page * per_page;
    
    // For each region, get paginated volumes
    for region in regions {
        // Get paginated volumes for this region
        let volumes = sqlx::query_as::<_, StorageVolume>(
            r#"
            SELECT
                v.*
            FROM 
                storage_volumes v
            INNER JOIN 
                workers w ON v.node_id = w.id
            INNER JOIN
                regions r ON w.region_id = r.id
            WHERE 
                r.provider = ?
                AND r.id = ?
            ORDER BY 
                v.id
            LIMIT ? OFFSET ?
            "#
        )
        .bind(provider_id)
        .bind(region.id)
        .bind(per_page)
        .bind(offset)
        .fetch_all(pool)
        .await
        .context(format!("Failed to fetch volumes for region {}", region.id))?;
        
        // Only add regions with volumes
        if !volumes.is_empty() {
            region_volumes.push(RegionVolumes {
                region,
                volumes
            });
        }
    }
    
    Ok(ProviderRegionVolumes {
        provider,
        regions: region_volumes
    })
}

/// Counts the total number of storage volumes for a specific provider
pub async fn count_volumes_for_provider(
    pool: &Pool<MySql>,
    provider_id: i64,
) -> anyhow::Result<i64> {
    // Get the total count of volumes for this provider
    let (total_volumes,) = sqlx::query_as::<_, (i64,)>(
        r#"
        SELECT
            COUNT(v.id)
        FROM 
            storage_volumes v
        INNER JOIN 
            workers w ON v.node_id = w.id
        INNER JOIN 
            regions r ON w.region_id = r.id
        WHERE 
            r.provider = ?
        "#
    )
    .bind(provider_id)
    .fetch_one(pool)
    .await
    .context("Failed to count volumes for provider")?;
    
    Ok(total_volumes)
}