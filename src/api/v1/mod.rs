use rocket::routes;

pub mod helpers;
pub mod apps;
pub mod instances;
pub mod deploy;
pub mod builds;
pub mod users;
pub mod permissions;
pub mod metadata;
pub mod audit_log;

use apps::*;
use instances::*;
use deploy::*;
use users::*;
use permissions::*;
use metadata::*;
use audit_log::*;

pub fn routes() -> Vec<rocket::Route> {
    routes![
        // apps
        get_app,
        list_apps,
        create_app,
        release,
        delete_app,
        scale_app,
        start_app,
        stop_app,
        get_app_stats,
        update_app,

        // instances
        list_instances,
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
    ]
}
