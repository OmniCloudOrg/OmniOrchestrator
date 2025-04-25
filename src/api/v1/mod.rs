use rocket::routes;

pub mod apps;
pub mod alerts;
pub mod audit_log;
pub mod builds;
pub mod deploy;
pub mod helpers;
pub mod instances;
pub mod metadata;
pub mod permissions;
pub mod regions;
pub mod users;
pub mod platforms;
pub mod workers;
pub mod control;
pub mod metrics;
pub mod providers;
pub mod storage;
//pub mod deployments;

use control::*;
use apps::*;
use alerts::*;
use users::*;
use builds::*;
use deploy::*;
use workers::*;
use regions::*;
use metadata::*;
use audit_log::*;
use platforms::*;
use instances::*;
use permissions::*;
use metrics::*;
use providers::*;
use storage::*;
// use platforms::*;
// use deployments::*;

pub fn routes() -> Vec<rocket::Route> {
    routes![
        // apps
        get_app,
        count_apps,
        list_apps,
        create_app,
        release,
        delete_app,
        scale_app,
        start_app,
        stop_app,
        get_app_stats,
        update_app,
        get_app_with_instances,
        // alerts
        list_alerts,
        // instances
        list_instances,
        list_instances_by_region,
        count_instances,
        get_instance,
        // deploy
        deploy_permissions,
        // users
        handle_create_user,
        handle_login,
        // permissions
        list_permission,
        get_permission_by_id,
        create_permission,
        delete_permission,
        // update_permission,
        // metadata
        get_meta_value,
        set_meta_value,
        // audit log
        create_audit_log,
        list_audit_logs,
        //Builds
        list_builds,
        list_builds_for_app,
        get_build,
        list_regions,
        list_providers,
        get_provider_instances,
        get_provider_audit_logs_paginated,
        list_provider_regions,
        // regions
        // get_region,
        // delete_region,
        // create_region,
        // update_region
        // deployments
        // list_deployments

        init_platform,
        check_platform_status,
        bootstrap_host,
        configure_network,
        setup_monitoring,
        setup_backups,
        
        // workers
        list_workers,
        get_worker_by_id,

        // Metrics
        get_metrics,
        get_metrics_by_app_id,

        // Storage
        list_storage_volumes_paginated,
        count_storage_volumes,

        // CLI
        control::backup::get_backup,
        control::backup::list_backups,
        control::backup::create_backup,
        control::backup::list_backups_by_app_id,
        ]
}
