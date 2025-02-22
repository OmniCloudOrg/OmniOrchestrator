use rocket::routes;

pub mod helpers;
pub mod apps;
pub mod instances;
pub mod deploy;
pub mod builds;
pub mod users;


use apps::*;
use instances::*;
use deploy::*;
use users::*;

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

        // instances
        list_instances,
        get_instance,

        // deploy
        deploy_permissions,

        // users
        handle_create_user,
        handle_login
    ]
}
