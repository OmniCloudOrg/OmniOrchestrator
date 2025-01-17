//------------------------------- queries.rs V1 -------------------------------//
// This file contains the database query functions for OmniOrchestrator.       //
// The queries are separated by their respective tables.                       //
// The queries are stored in separate files in the sql/versions/V1/queries     //
// directory.                                                                  //
//                                                                             //
// Authors: Tristan J. Poland                                                  //
// License: GNU License                                                        //
//-----------------------------------------------------------------------------//

use mysql::prelude::*;
use mysql::*;
use chrono::{ DateTime, Utc };
use super::get_conn;
use crate::db::data_types::*;

macro_rules! db_operation {
    // Query pattern for no parameters
    (query $sql:expr) => {{
        let mut conn = get_conn()?;
        let mut ids: Vec<i64> = conn.query_map($sql, |row: Row| {
            row.get::<i64, _>(0).unwrap_or(0)
        })?;
        ids.sort();
        Ok(ids)
    }};

    // Query pattern with parameters
    (query $sql:expr, $($name:expr => $value:expr),+) => {{
        let mut conn = get_conn()?;
        let mut ids: Vec<i64> = conn.exec_map(
            $sql,
            params! { $($name => $value),* },
            |row: Row| row.get::<i64, _>(0).unwrap_or(0)
        )?;
        ids.sort();
        Ok(ids)
    }};

    // Get single value pattern with parameters
    (get $sql:expr, $($name:expr => $value:expr),+) => {{
        let mut conn = get_conn()?;
        let row: Option<i64> = conn.exec_first($sql, params! { $($name => $value),* })?;
        Ok(row.unwrap_or(0))
    }};

    // Execute pattern for no parameters
    (execute $sql:expr) => {{
        let mut conn = get_conn()?;
        conn.query_drop("SET FOREIGN_KEY_CHECKS=0")?;
        let result = conn.query_drop($sql)?;
        conn.query_drop("SET FOREIGN_KEY_CHECKS=1")?;
        Ok(conn.last_insert_id() as i64)
    }};

    // Execute pattern with parameters
    (execute $sql:expr, $($name:expr => $value:expr),+) => {{
        let mut conn = get_conn()?;
        conn.query_drop("SET FOREIGN_KEY_CHECKS=0")?;
        conn.exec_drop($sql, params! { $($name => $value),* })?;
        let id = conn.last_insert_id() as i64;
        conn.query_drop("SET FOREIGN_KEY_CHECKS=1")?;
        Ok(id)
    }};

    // pattern for mapping to struct
    (map_to $sql:expr, $struct_type:ty) => {{
        let mut conn = get_conn()?;
        let results: Result<Vec<$struct_type>> = conn.query_map($sql, |row: Row| {
            FromRow::from_row_opt(row).unwrap()
        });
        results
    }};

    // pattern for mapping to struct with parameters
    (map_to $sql:expr, $struct_type:ty, $($name:expr => $value:expr),+) => {{
        let mut conn = get_conn()?;
        let results: Result<Vec<$struct_type>> = conn.exec_map(
            $sql,
            params! { $($name => $value),* },
            |row: Row| FromRow::from_row_opt(row).unwrap()
        );
        results
    }};

    // pattern for getting single struct
    (get_as $sql:expr, $struct_type:ty, $($name:expr => $value:expr),+) => {{
        let mut conn = get_conn()?;
        let result: Result<Option<$struct_type>> = conn.exec_first(
            $sql,
            params! { $($name => $value),* }
        );
        result
    }};
}

// Apps
pub fn list_apps() -> Result<Vec<i64>> {
    db_operation!(query "SELECT app_id FROM apps")
}

// Example of a function using the new macro pattern
pub fn get_app_details(app_id: i64) -> Result<Option<App>> {
    db_operation!(get_as 
        "SELECT * FROM apps WHERE app_id = :app_id",
        App,
        "app_id" => app_id
    )
}

pub fn get_app_by_id(app_id: i64) -> Result<i64> {
    db_operation!(get "SELECT app_id FROM apps WHERE app_id = :app_id", 
        "app_id" => app_id
    )
}

pub fn get_app_by_name(app_name: String) -> Result<i64> {
    db_operation!(get "SELECT * FROM apps WHERE name = :app_name", 
        "app_name" => app_name
    )
}

pub fn app_create(name: &str, user_id: i64) -> Result<i64> {
    let sql = include_str!("../.././sql/versions/V1/queries/app/app_create.sql");
    db_operation!(execute sql,
        "name" => name,
        "user_id" => user_id,
        "created_at" => Utc::now().format("%Y-%m-%d %H:%M:%S").to_string()
    )
}

pub fn app_edit(app_id: i64, name: &str) -> Result<i64> {
    let sql = include_str!("../.././sql/versions/V1/queries/app/app_edit.sql");
    db_operation!(execute sql,
        "name" => name,
        "updated_at" => Utc::now().naive_utc().format("%Y-%m-%d %H:%M:%S").to_string(),
        "app_id" => app_id
    )
}

pub fn app_remove(app_id: i64) -> Result<i64> {
    let sql = include_str!("../.././sql/versions/V1/queries/app/app_remove.sql");
    db_operation!(execute sql, "app_id" => app_id)
}

pub fn app_scale(app_id: i64, instances: i64) -> Result<i64> {
    let sql = include_str!("../.././sql/versions/V1/queries/app/app_scale.sql");
    db_operation!(execute sql,
        "instances" => instances,
        "updated_at" => Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        "app_id" => app_id
    )
}

pub fn app_start(app_id: i64) -> Result<i64> {
    let sql = include_str!("../.././sql/versions/V1/queries/app/app_start.sql");
    db_operation!(execute sql,
        "status" => "running",
        "updated_at" => Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        "app_id" => app_id
    )
}

pub fn app_stop(app_id: i64) -> Result<i64> {
    let sql = include_str!("../.././sql/versions/V1/queries/app/app_stop.sql");
    db_operation!(execute sql,
        "status" => "stopped",
        "updated_at" => Utc::now().naive_utc().format("%Y-%m-%d %H:%M:%S").to_string(),
        "app_id" => app_id
    )
}

// Builds
pub fn build_create(app_id: i64, source_version: &str) -> Result<i64> {
    let sql = include_str!("../.././sql/versions/V1/queries/build/build_create.sql");
    db_operation!(execute sql,
        "app_id" => app_id,
        "source_version" => source_version,
        "status" => "pending",
        "created_at" => Utc::now().naive_utc().format("%Y-%m-%d %H:%M:%S").to_string(),
        "completed_at" => Option::<String>::None
    )
}

pub fn build_edit(build_id: i64, status: &str) -> Result<i64> {
    let sql = include_str!("../.././sql/versions/V1/queries/build/build_edit.sql");
    db_operation!(execute sql,
        "status" => status,
        "updated_at" => Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        "build_id" => build_id
    )
}

pub fn build_remove(build_id: i64) -> Result<i64> {
    let sql = include_str!("../.././sql/versions/V1/queries/build/build_remove.sql");
    db_operation!(execute sql, "build_id" => build_id)
}

// Deployments
pub fn list_deployments(app_id: i64) -> Result<Vec<i64>> {
    db_operation!(query "SELECT deploy_id FROM deployments WHERE app_id = :app_id",
        "app_id" => app_id
    )
}

pub fn get_deployment(deploy_id: i64) -> Result<i64> {
    db_operation!(get "SELECT deploy_id FROM deployments WHERE deploy_id = :deploy_id",
        "deploy_id" => deploy_id
    )
}

pub fn deploy_create(app_id: i64, build_id: i64) -> Result<i64> {
    let sql = include_str!("../.././sql/versions/V1/queries/deployment/deployment_create.sql");
    db_operation!(execute sql,
        "app_id" => app_id,
        "build_id" => build_id,
        "status" => "pending",
        "created_at" => Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        "completed_at" => Option::<String>::None
    )
}

pub fn deploy_remove(deploy_id: i64) -> Result<i64> {
    let sql = include_str!("../.././sql/versions/V1/queries/deployment/deployment_remove.sql");
    db_operation!(execute sql, "deploy_id" => deploy_id)
}

pub fn deployment_log_create(deploy_id: i64, log_entry: &str) -> Result<i64> {
    let sql = include_str!("../.././sql/versions/V1/queries/deployment/log/deployment_log_create.sql");
    db_operation!(execute sql,
        "deploy_id" => deploy_id,
        "log_entry" => log_entry,
        "created_at" => Utc::now().format("%Y-%m-%d %H:%M:%S").to_string()
    )
}

pub fn deployment_log_remove(deployment_log_id: i64) -> Result<i64> {
    let sql = include_str!("../.././sql/versions/V1/queries/deployment/log/deployment_log_remove.sql");
    db_operation!(execute sql, "deployment_log_id" => deployment_log_id)
}

// Users
pub fn list_users() -> Result<Vec<i64>> {
    db_operation!(query "SELECT user_id FROM users")
}

pub fn get_user(user_id: i64) -> Result<i64> {
    db_operation!(get "SELECT user_id FROM users WHERE user_id = :user_id",
        "user_id" => user_id
    )
}

pub fn user_create(username: &str, password: &str, email: &str, active: i32) -> Result<i64> {
    let sql = include_str!("../.././sql/versions/V1/queries/user/user_create.sql");
    db_operation!(execute sql,
        "email" => email,
        "username" => username,
        "password" => password,
        "active" => active
    )
}

pub fn user_edit(user_id: i64, password: &str) -> Result<i64> {
    let sql = include_str!("../.././sql/versions/V1/queries/user/user_update.sql");
    db_operation!(execute sql,
        "password" => password,
        "updated_at" => Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        "user_id" => user_id
    )
}

pub fn user_login(username: &str, password: &str) -> Result<i64> {
    let sql = include_str!("../.././sql/versions/V1/queries/user/user_check_login.sql");
    db_operation!(get sql,
        "username" => username,
        "password" => password
    )
}

pub fn user_remove(user_id: i64) -> Result<i64> {
    let sql = include_str!("../.././sql/versions/V1/queries/user/user_remove.sql");
    db_operation!(execute sql, "user_id" => user_id)
}

// Instances
pub fn list_instances(app_id: i64) -> Result<Vec<i64>> {
    db_operation!(query "SELECT instance_id FROM instances WHERE app_id = :app_id",
        "app_id" => app_id
    )
}

pub fn get_instance(instance_id: i64) -> Result<i64> {
    db_operation!(get "SELECT instance_id FROM instances WHERE instance_id = :instance_id",
        "instance_id" => instance_id
    )
}

pub fn instance_create(app_id: i64, deploy_id: i64, host: &str, port: i64) -> Result<i64> {
    let sql = include_str!("../.././sql/versions/V1/queries/instance/instance_create.sql");
    db_operation!(execute sql,
        "app_id" => app_id,
        "deploy_id" => deploy_id,
        "host" => host,
        "port" => port,
        "status" => "pending",
        "created_at" => Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        "terminated_at" => Option::<String>::None
    )
}

pub fn instance_remove(instance_id: i64) -> Result<i64> {
    let sql = include_str!("../.././sql/versions/V1/queries/instance/instance_remove.sql");
    db_operation!(execute sql, "instance_id" => instance_id)
}

pub fn instance_log_create(instance_id: i64, log_entry: &str) -> Result<i64> {
    let sql = include_str!("../.././sql/versions/V1/queries/instance/log/instance_log_create.sql");
    db_operation!(execute sql,
        "instance_id" => instance_id,
        "log_entry" => log_entry,
        "created_at" => Utc::now().format("%Y-%m-%d %H:%M:%S").to_string()
    )
}

pub fn instance_log_remove(instance_log_id: i64) -> Result<i64> {
    let sql = include_str!("../.././sql/versions/V1/queries/instance/log/instance_log_remove.sql");
    db_operation!(execute sql, "instance_log_id" => instance_log_id)
}

pub fn instance_metrics_create(instance_id: i64, cpu: f64, memory: f64, disk: f64) -> Result<i64> {
    let sql = include_str!("../.././sql/versions/V1/queries/instance/metrics/instance_metrics_create.sql");
    db_operation!(execute sql,
        "instance_id" => instance_id,
        "cpu" => cpu,
        "memory" => memory,
        "disk" => disk,
        "created_at" => Utc::now().format("%Y-%m-%d %H:%M:%S").to_string()
    )
}

pub fn instance_metrics_remove(instance_metrics_id: i64) -> Result<i64> {
    let sql = include_str!("../.././sql/versions/V1/queries/instance/metrics/instance_metrics_remove.sql");
    db_operation!(execute sql, "instance_metrics_id" => instance_metrics_id)
}

// Permissions
pub fn list_permissions() -> Result<Vec<i64>> {
    db_operation!(query "SELECT permission_id FROM permissions")
}

pub fn get_permission(permission_id: i64) -> Result<i64> {
    db_operation!(get "SELECT permission_id FROM permissions WHERE permission_id = :permission_id",
        "permission_id" => permission_id
    )
}

pub fn permission_create(user_id: i64, app_id: i64, permission: &str) -> Result<i64> {
    let sql = include_str!("../.././sql/versions/V1/queries/permission/permission_create.sql");
    db_operation!(execute sql,
        "user_id" => user_id,
        "app_id" => app_id,
        "permission" => permission,
        "created_at" => Utc::now().format("%Y-%m-%d %H:%M:%S").to_string()
    )
}

pub fn permission_remove(permission_id: i64) -> Result<i64> {
    let sql = include_str!("../.././sql/versions/V1/queries/permission/permission_remove.sql");
    db_operation!(execute sql, "permission_id" => permission_id)
}

// Domains
pub fn list_domains() -> Result<Vec<i64>> {
    db_operation!(query "SELECT domain_id FROM domains")
}

pub fn get_domain(domain_id: i64) -> Result<i64> {
    db_operation!(get "SELECT domain_id FROM domains WHERE domain_id = :domain_id",
        "domain_id" => domain_id
    )
}

pub fn domain_create(app_id: i64, domain: &str) -> Result<i64> {
    let sql = include_str!("../.././sql/versions/V1/queries/domain/domain_create.sql");
    db_operation!(execute sql,
        "app_id" => app_id,
        "domain" => domain,
        "created_at" => Utc::now().format("%Y-%m-%d %H:%M:%S").to_string()
    )
}

pub fn domain_remove(domain_id: i64) -> Result<i64> {
    let sql = include_str!("../.././sql/versions/V1/queries/domain/domain_remove.sql");
    db_operation!(execute sql, "domain_id" => domain_id)
}

// Orgs
pub fn list_orgs() -> Result<Vec<i64>> {
    db_operation!(query "SELECT org_id FROM orgs")
}

pub fn get_org(org_id: i64) -> Result<i64> {
    db_operation!(get "SELECT org_id FROM orgs WHERE org_id = :org_id",
        "org_id" => org_id
    )
}

pub fn org_create(name: &str) -> Result<i64> {
    let sql = include_str!("../.././sql/versions/V1/queries/org/org_create.sql");
    db_operation!(execute sql,
        "name" => name,
        "created_at" => Utc::now().format("%Y-%m-%d %H:%M:%S").to_string()
    )
}

pub fn org_edit(org_id: i64, name: &str) -> Result<i64> {
    let sql = include_str!("../.././sql/versions/V1/queries/org/org_edit.sql");
    db_operation!(execute sql,
        "name" => name,
        "updated_at" => Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        "org_id" => org_id
    )
}

pub fn org_remove(org_id: i64) -> Result<i64> {
    let sql = include_str!("../.././sql/versions/V1/queries/org/org_remove.sql");
    db_operation!(execute sql, "org_id" => org_id)
}

// API Keys
pub fn list_api_keys() -> Result<Vec<i64>> {
    db_operation!(query "SELECT api_key_id FROM api_keys")
}

pub fn get_api_key(api_key_id: i64) -> Result<i64> {
    db_operation!(get "SELECT api_key_id FROM api_keys WHERE api_key_id = :api_key_id",
        "api_key_id" => api_key_id
    )
}

pub fn api_key_create(user_id: i64, key: &str) -> Result<i64> {
    let sql = include_str!("../.././sql/versions/V1/queries/api_keys/api_key_create.sql");
    db_operation!(execute sql,
        "user_id" => user_id,
        "key" => key,
        "created_at" => Utc::now().format("%Y-%m-%d %H:%M:%S").to_string()
    )
}

pub fn api_key_remove(api_key_id: i64) -> Result<i64> {
    let sql = include_str!("../.././sql/versions/V1/queries/api_keys/api_key_remove.sql");
    db_operation!(execute sql, "api_key_id" => api_key_id)
}