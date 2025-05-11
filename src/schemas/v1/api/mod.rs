//-----------------------------------------------------------------------------
// OmniOrchestrator V1 API route registration module
//-----------------------------------------------------------------------------
// Authors: Tristan Poland, Maxine DeAndreade, Caznix
//
// This module contains the route registration for the V1 API of OmniOrchestrator.
// It includes all the necessary routes for various functionalities such as
// apps, alerts, notifications, instances, users, permissions, metadata,
// audit logs, builds, regions, providers, workers, metrics, storage,
// cost, deployments, and logging.
//
// Each module corresponds to a specific functionality and contains the
// implementation of the routes related to that functionality.
// The `routes` function aggregates all the routes from the individual
// modules and returns them as a vector of Rocket routes.
// The routes are defined using the Rocket framework, which is a web
// framework for Rust.
//
// The modules are organized in a way that allows for easy navigation
// and understanding of the different functionalities provided by
// OmniOrchestrator. Each module is responsible for a specific area of
// functionality, and the routes defined within that module are related
// to that functionality.
//
// The `routes` function is the entry point for registering all the
// routes in the V1 API. It uses the `routes!` macro from Rocket to
// define the routes and their corresponding handlers. The routes
// are then returned as a vector, which can be used to register the
// routes with the Rocket application.
//-----------------------------------------------------------------------------

use rocket::routes;

pub mod alerts;
pub mod apps;
pub mod audit_log;
pub mod builds;
pub mod control;
pub mod cost;
pub mod deploy;
pub mod helpers;
pub mod instances;
pub mod metadata;
pub mod metrics;
pub mod notifications;
pub mod permissions;
// pub mod platforms;
pub mod deployments;
pub mod index;
pub mod logging;
pub mod providers;
pub mod regions;
pub mod storage;
pub mod users;
pub mod workers;

pub fn routes() -> Vec<rocket::Route> {
    routes![
        // apps
        apps::release,
        apps::get_app,
        apps::stop_app,
        apps::list_apps,
        apps::start_app,
        apps::scale_app,
        apps::count_apps,
        apps::create_app,
        apps::update_app,
        apps::delete_app,
        apps::get_app_stats,
        apps::list_instances,
        apps::get_app_with_instances,
        // alerts
        alerts::list_alerts,
        alerts::get_alert,
        alerts::create_alert,
        alerts::update_alert_status,
        alerts::acknowledge_alert,
        alerts::resolve_alert,
        alerts::escalate_alert,
        alerts::get_app_alerts,
        alerts::get_org_active_alerts,
        alerts::get_org_alert_stats,
        alerts::get_alerts_needing_escalation,
        alerts::auto_resolve_old_alerts,
        alerts::search_alerts,
        alerts::bulk_update_alert_status,
        // Notifications
        notifications::list_user_notifications,
        notifications::count_unread_user_notifications,
        notifications::get_user_notification_by_id,
        notifications::create_user_notification,
        notifications::mark_user_notification_as_read,
        notifications::mark_all_user_notifications_as_read,
        notifications::delete_user_notification,
        notifications::delete_read_user_notifications,
        notifications::list_role_notifications,
        notifications::create_role_notification,
        notifications::acknowledge_notification,
        notifications::get_all_user_notifications_with_count,
        // Instances
        instances::list_instances_by_region,
        instances::count_instances,
        instances::get_instance,
        // deploy
        deploy::deploy_permissions,
        // Users
        users::handle_register,
        users::handle_login,
        users::update_profile,
        users::get_current_user,
        users::change_password,
        users::logout,
        users::get_user_profile,
        users::list_user_sessions,
        users::invalidate_user_session,
        users::list_users,
        // permissions
        permissions::list_permission,
        permissions::get_permission_by_id,
        permissions::create_permission,
        permissions::delete_permission,
        // update_permission,

        // Metadata
        metadata::get_meta_value,
        metadata::set_meta_value,
        // Audit log
        audit_log::create_audit_log,
        audit_log::list_audit_logs,
        audit_log::list_audit_logs_for_app,
        //Builds
        builds::list_builds,
        builds::list_builds_for_app,
        builds::get_build,
        // Regions
        regions::list_regions,
        regions::list_provider_regions,
        //        regions::get_region,
        //        regions::delete_region,
        //        regions::create_region,
        //        regions::update_region,

        // Providers
        providers::list_providers,
        providers::get_provider_instances,
        providers::get_provider_audit_logs_paginated,
        // TODO: @tristanpoland Migration broke these
        //init_platform,
        //check_platform_status,
        //bootstrap_host,
        //configure_network,
        //setup_monitoring,
        //setup_backups,

        // Workers
        workers::list_workers,
        workers::get_worker_by_id,
        // Metrics
        metrics::get_metrics,
        metrics::get_metrics_by_app_id,
        // Storage
        storage::list_storage_classes,
        storage::get_storage_class,
        storage::list_storage_volumes,
        storage::get_volumes_by_storage_class,
        storage::list_qos_policies,
        storage::list_volumes_by_write_concern,
        storage::list_volumes_by_persistence_level,
        storage::get_volumes_for_region_route,
        storage::get_storage_volumes_for_provider,
        // Cost
        // Resource Type routes
        cost::list_resource_types,
        cost::count_resource_types,
        cost::get_resource_type,
        cost::create_resource_type,
        cost::update_resource_type,
        cost::delete_resource_type,
        // Cost Metric routes
        cost::list_cost_metrics,
        cost::get_cost_metric,
        cost::create_cost_metric,
        cost::delete_cost_metric,
        cost::analyze_costs_by_dimension,
        cost::analyze_cost_over_time,
        // Cost Budget routes
        cost::list_cost_budgets,
        cost::get_cost_budget,
        cost::create_cost_budget,
        cost::update_cost_budget,
        cost::delete_cost_budget,
        // Cost Projection routes
        cost::list_cost_projections,
        cost::get_cost_projection,
        cost::create_cost_projection,
        cost::delete_cost_projection,
        // Resource Pricing routes
        cost::list_resource_pricing,
        cost::get_resource_pricing,
        cost::create_resource_pricing,
        cost::update_resource_pricing,
        cost::delete_resource_pricing,
        // Cost Allocation Tag routes
        cost::get_cost_allocation_tags,
        cost::create_cost_allocation_tag,
        cost::delete_cost_allocation_tag,
        // CLI
        // control::backup::get_backup,
        // control::backup::list_backups,
        // control::backup::create_backup,
        // control::backup::list_backups_by_app_id,
        deployments::list_deployments,
        deployments::count_deployments,
        deployments::get_deployment,
        deployments::list_app_deployments,
        deployments::create_deployment,
        deployments::update_deployment_status,
        deployments::delete_deployment,
        // Logging
        logging::list_logs,
        logging::list_platform_logs,
        logging::list_org_logs,
        logging::list_app_logs,
        logging::list_instance_logs,
        logging::insert_logs
    ]
}
