use rocket::routes;

pub mod helpers;
pub mod apps;
pub mod deploy;
pub mod builds;
pub mod users;


use apps::*;
use deploy::*;

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

        // deploy
        deploy_permissions
    ]
}
