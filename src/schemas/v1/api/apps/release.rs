use super::super::super::db::queries as db;
use rocket::http::{ContentType, Status};
use rocket::{post, Data, State};
use std::sync::Arc;

use crate::DatabaseManager;

/// Releases a new version of the target application by uploading an artifact.
/// TODO: @tristanpoland Review if we actually need this or should drop in favor
///       of using the deploy route.
/// 
/// # Arguments
///
/// * `platform_id` - Platform identifier
/// * `app_id` - The ID of the application to release a new version for
/// * `release_version` - The version tag for this release
/// * `content_type` - The content type of the data being uploaded
/// * `data` - The data stream of the artifact being uploaded
/// * `db_manager` - Database manager for accessing platform-specific pools
///
/// # Returns
///
/// * `Status::Ok` - If the artifact is successfully uploaded and added to the build jobs list
/// * `Status::BadRequest` - If there is an error in the upload process
///
/// # Details
///
/// This route handles the release of a new version of an application by:
/// 1. Uploading the provided artifact to the build artifacts list.
/// 2. Adding the artifact to the list of build jobs for the Forge instances to pick up and process.
///
/// The actual implementation of the release process is delegated to the `helpers::release::release`
/// function, as it is quite extensive.
#[post(
    "/platform/<platform_id>/apps/<app_id>/releases/<release_version>/upload",
    format = "multipart/form-data",
    data = "<data>"
)]
pub async fn create_release(
    platform_id: i64,
    app_id: String,
    release_version: String,
    content_type: &ContentType,
    data: Data<'_>,
    db_manager: &State<Arc<DatabaseManager>>,
) -> Result<Status, Status> {
    // We need to modify the helper function to work with platform-specific DBs
    // For now, we'll just pass the helper what it needs, but ideally the helper should be updated to use platform pools

    // Get platform info and pass to helper
    match db::platforms::get_platform_by_id(db_manager.get_main_pool(), platform_id).await {
        Ok(_) => {
            // We found the platform, proceed with release
            super::super::helpers::release::release(app_id, release_version, content_type, data).await
        },
        Err(_) => {
            // Platform not found
            Err(Status::NotFound)
        }
    }
}