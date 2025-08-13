use rocket::{Rocket, Build};
use std::sync::Arc;
use tokio::sync::RwLock;
use colored::Colorize;
use libomni::types::db::auth::AuthConfig;

use crate::cluster::ClusterManager;
use crate::state::SharedState;
use crate::db_manager::DatabaseManager;
use crate::cors::CORS;
use crate::endpoints::{health_check, cluster_status};
use crate::cors::cors_preflight;
use crate::schemas::v1::api;

pub trait RocketExt {
    fn mount_routes(self, routes: Vec<(&'static str, Vec<rocket::Route>)>) -> Self;
}

impl RocketExt for Rocket<Build> {
    fn mount_routes(self, routes: Vec<(&'static str, Vec<rocket::Route>)>) -> Self {
        let mut rocket = self;
        for (path, routes) in routes {
            log::info!("{}", format!("Mounting routes at {}", path).green());
            rocket = rocket.mount(path, routes);
        }
        rocket
    }
}

pub fn build_rocket(
    port: u16,
    db_manager: Arc<DatabaseManager>,
    pool: sqlx::Pool<sqlx::MySql>,
    cluster_manager: Arc<RwLock<ClusterManager>>,
    clickhouse_client: clickhouse::Client,
    shared_state: Arc<RwLock<SharedState>>,
    auth_config: AuthConfig,
) -> Rocket<Build> {
    println!(
        "{}",
        "╔═══════════════════════════════════════════════════════════════╗".bright_cyan()
    );
    println!(
        "{}",
        "║                       SERVER STARTUP                          ║".bright_cyan()
    );
    println!(
        "{}",
        "╚═══════════════════════════════════════════════════════════════╝".bright_cyan()
    );

    log::info!("{}", "Defining API routes".cyan());
    let routes = vec![
        (
            "/",
            routes![
                health_check,
                api::index::routes_ui,
                cluster_status,
                cors_preflight
            ],
        ),
        ("/api/v1", api::routes()),
    ];

    log::info!("{}", "Building Rocket instance".cyan());
    let rocket_instance = rocket::build()
        .configure(rocket::Config {
            port,
            address: std::net::IpAddr::V4(std::net::Ipv4Addr::new(0, 0, 0, 0)),
            ..Default::default()
        })
        .manage(db_manager)
        .manage(pool)
        .manage(cluster_manager)
        .manage(clickhouse_client)
        .manage(shared_state)
        .manage(auth_config)
        .attach(CORS);

    log::info!("{}", "Mounting API routes".cyan());
    let rocket_with_routes = rocket_instance.mount_routes(routes);

    api::index::collect_routes(&rocket_with_routes);

    rocket_with_routes
}