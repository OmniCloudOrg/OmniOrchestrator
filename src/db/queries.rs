//-------------------------------- queries.rs V1 ------------------------------//                                                           //
// This file contains the database query functions for OmniOrchestrator.       //
// The queries are separated by their respective tables.                       //
// The queries are stored in separate files in the sql/versions/V1/queries     //
// directory. The queries are read from the files and executed using rusqlite. //
//                                                                             //
// Authors: Tristan J. Poland                                                  //
// License: GNU License                                                        //
//-----------------------------------------------------------------------------//

use rusqlite::{ Connection, Result, params };
use chrono::{ DateTime, Utc };

macro_rules! db_operation {
    (query $sql:expr $(,)?) => {{
        let conn = Connection::open("cluster.db")?;
        let mut stmt = conn.prepare($sql)?;
        let rows = stmt.query_map([], |row| row.get(0))?;
        let mut ids: Vec<i64> = rows.filter_map(|r| r.ok()).collect();
        ids.sort();
        Ok(ids)
    }};

    (query $sql:expr, $($param:expr),+ $(,)?) => {{
        let conn = Connection::open("cluster.db")?;
        let mut stmt = conn.prepare($sql)?;
        let rows = stmt.query_map(params![$($param),*], |row| row.get(0))?;
        let mut ids: Vec<i64> = rows.filter_map(|r| r.ok()).collect();
        ids.sort();
        Ok(ids)
    }};

    (get $sql:expr, $($param:expr),+ $(,)?) => {{
        let conn = Connection::open("cluster.db")?;
        let mut stmt = conn.prepare($sql)?;
        let mut rows = stmt.query_map(params![$($param),*], |row| row.get(0))?;
        match rows.next() {
            Some(id) => Ok(id?),
            None => Ok(0)
        }
    }};

    (execute $sql:expr $(,)?) => {{
        let conn = Connection::open("cluster.db")?;
        conn.execute("PRAGMA foreign_keys = OFF;", [])?;
        let result = conn.execute($sql, [])?;
        conn.execute("PRAGMA foreign_keys = ON;", [])?;
        Ok(conn.last_insert_rowid())
    }};

    (execute $sql:expr, $($param:expr),+ $(,)?) => {{
        let conn = Connection::open("cluster.db")?;
        conn.execute("PRAGMA foreign_keys = OFF;", [])?;
        conn.execute($sql, params![$($param),*])?;
        let id = conn.last_insert_rowid();
        conn.execute("PRAGMA foreign_keys = ON;", [])?;
        Ok(id)
    }};
}

// Apps
pub fn list_apps() -> Result<Vec<i64>> {
    db_operation!(query "SELECT app_id FROM apps")
}

pub fn get_app(app_id: i64) -> Result<i64> {
    db_operation!(get "SELECT app_id FROM apps WHERE app_id = ?", app_id)
}

pub fn app_create(name: &str, user_id: i64) -> Result<i64> {
    let sql = include_str!("../.././sql/versions/V1/queries/app/app_create.sql");
    db_operation!(execute sql, name, user_id, Utc::now())
}

pub fn app_edit(app_id: i64, name: &str) -> Result<i64> {
    let sql = include_str!("../.././sql/versions/V1/queries/app/app_edit.sql");
    db_operation!(execute sql, name, Utc::now(), app_id)
}

pub fn app_remove(app_id: i64) -> Result<i64> {
    let sql = include_str!("../.././sql/versions/V1/queries/app/app_remove.sql");
    db_operation!(execute sql, app_id)
}

pub fn app_scale(app_id: i64, instances: i64) -> Result<i64> {
    let sql = include_str!("../.././sql/versions/V1/queries/app/app_scale.sql");
    db_operation!(execute sql, instances, Utc::now(), app_id)
}

pub fn app_start(app_id: i64) -> Result<i64> {
    let sql = include_str!("../.././sql/versions/V1/queries/app/app_start.sql");
    db_operation!(execute sql, "running", Utc::now(), app_id)
}

pub fn app_stop(app_id: i64) -> Result<i64> {
    let sql = include_str!("../.././sql/versions/V1/queries/app/app_stop.sql");
    db_operation!(execute sql, "stopped", Utc::now(), app_id)
}

// Builds
pub fn build_create(app_id: i64, source_version: &str) -> Result<i64> {
    let sql = include_str!("../.././sql/versions/V1/queries/build/build_create.sql");
    db_operation!(execute sql, app_id, source_version, "pending", Utc::now(), Option::<DateTime<Utc>>::None)
}

pub fn build_edit(build_id: i64, status: &str) -> Result<i64> {
    let sql = include_str!("../.././sql/versions/V1/queries/build/build_edit.sql");
    db_operation!(execute sql, status, Utc::now(), build_id)
}

pub fn build_remove(build_id: i64) -> Result<i64> {
    let sql = include_str!("../.././sql/versions/V1/queries/build/build_remove.sql");
    db_operation!(execute sql, build_id)
}

// Deployments
pub fn list_deployments(app_id: i64) -> Result<Vec<i64>> {
    db_operation!(query "SELECT deploy_id FROM deployments WHERE app_id = ?", app_id)
}

pub fn get_deployment(deploy_id: i64) -> Result<i64> {
    db_operation!(get "SELECT deploy_id FROM deployments WHERE deploy_id = ?", deploy_id)
}

pub fn deploy_create(app_id: i64, build_id: i64) -> Result<i64> {
    let sql = include_str!("../.././sql/versions/V1/queries/deployment/deployment_create.sql");
    db_operation!(execute sql, app_id, build_id, "pending", Utc::now(), Option::<DateTime<Utc>>::None)
}

pub fn deploy_remove(deploy_id: i64) -> Result<i64> {
    let sql = include_str!("../.././sql/versions/V1/queries/deployment/deployment_remove.sql");
    db_operation!(execute sql, deploy_id)
}

pub fn deployment_log_create(deploy_id: i64, timestamp: DateTime<Utc>, log_entry: &str) -> Result<i64> {
    let sql = include_str!("../.././sql/versions/V1/queries/deployment/log/deployment_log_create.sql");
    db_operation!(execute sql, deploy_id, log_entry, Utc::now())
}

pub fn deployment_log_remove(deployment_log_id: i64) -> Result<i64> {
    let sql = include_str!("../.././sql/versions/V1/queries/deployment/log/deployment_log_remove.sql");
    db_operation!(execute sql, deployment_log_id)
}

// Users
pub fn list_users() -> Result<Vec<i64>> {
    db_operation!(query "SELECT user_id FROM users")
}

pub fn get_user(user_id: i64) -> Result<i64> {
    db_operation!(get "SELECT user_id FROM users WHERE user_id = ?", user_id)
}

pub fn user_create(username: &str, password: &str, email: &str, active: i32) -> Result<i64> {
    let sql = include_str!("../.././sql/versions/V1/queries/user/user_create.sql");
    db_operation!(execute sql, email, email, username, password, active)
}

pub fn user_edit(user_id: i64, password: &str) -> Result<i64> {
    let sql = include_str!("../.././sql/versions/V1/queries/user/user_update.sql");
    db_operation!(execute sql, password, Utc::now(), user_id)
}

pub fn user_login(username: &str, password: &str) -> Result<i64> {
    let sql = include_str!("../.././sql/versions/V1/queries/user/user_check_login.sql");
    db_operation!(get sql, username, password)
}

pub fn user_remove(user_id: i64) -> Result<i64> {
    let sql = include_str!("../.././sql/versions/V1/queries/user/user_remove.sql");
    db_operation!(execute sql, user_id)
}

// Instances
pub fn list_instances(app_id: i64) -> Result<Vec<i64>> {
    db_operation!(query "SELECT instance_id FROM instances WHERE app_id = ?", app_id)
}

pub fn get_instance(instance_id: i64) -> Result<i64> {
    db_operation!(get "SELECT instance_id FROM instances WHERE instance_id = ?", instance_id)
}

pub fn instance_create(app_id: i64, deploy_id: i64, host: &str, port: i64) -> Result<i64> {
    let sql = include_str!("../.././sql/versions/V1/queries/instance/instance_create.sql");
    db_operation!(execute sql, app_id, deploy_id, host, port, "pending", Utc::now(), Option::<DateTime<Utc>>::None)
}

pub fn instance_remove(instance_id: i64) -> Result<i64> {
    let sql = include_str!("../.././sql/versions/V1/queries/instance/instance_remove.sql");
    db_operation!(execute sql, instance_id)
}

pub fn instance_log_create(instance_id: i64, timestamp: DateTime<Utc>, log_entry: &str) -> Result<i64> {
    let sql = include_str!("../.././sql/versions/V1/queries/instance/log/instance_log_create.sql");
    db_operation!(execute sql, instance_id, log_entry, Utc::now())
}

pub fn instance_log_remove(instance_log_id: i64) -> Result<i64> {
    let sql = include_str!("../.././sql/versions/V1/queries/instance/log/instance_log_remove.sql");
    db_operation!(execute sql, instance_log_id)
}

pub fn instance_metrics_create(instance_id: i64, cpu: f64, memory: f64, disk: f64) -> Result<i64> {
    let sql = include_str!("../.././sql/versions/V1/queries/instance/metrics/instance_metrics_create.sql");
    db_operation!(execute sql, instance_id, cpu, memory, disk, Utc::now())
}

pub fn instance_metrics_remove(instance_metrics_id: i64) -> Result<i64> {
    let sql = include_str!("../.././sql/versions/V1/queries/instance/metrics/instance_metrics_remove.sql");
    db_operation!(execute sql, instance_metrics_id)
}

// Permissions
pub fn list_permissions() -> Result<Vec<i64>> {
    db_operation!(query "SELECT permission_id FROM permissions")
}

pub fn get_permission(permission_id: i64) -> Result<i64> {
    db_operation!(get "SELECT permission_id FROM permissions WHERE permission_id = ?", permission_id)
}

pub fn permission_create(user_id: i64, app_id: i64, permission: &str) -> Result<i64> {
    let sql = include_str!("../.././sql/versions/V1/queries/permission/permission_create.sql");
    db_operation!(execute sql, user_id, app_id, permission, Utc::now())
}

pub fn permission_remove(permission_id: i64) -> Result<i64> {
    let sql = include_str!("../.././sql/versions/V1/queries/permission/permission_remove.sql");
    db_operation!(execute sql, permission_id)
}

// Domains
pub fn list_domains() -> Result<Vec<i64>> {
    db_operation!(query "SELECT domain_id FROM domains")
}

pub fn get_domain(domain_id: i64) -> Result<i64> {
    db_operation!(get "SELECT domain_id FROM domains WHERE domain_id = ?", domain_id)
}

pub fn domain_create(app_id: i64, domain: &str) -> Result<i64> {
    let sql = include_str!("../.././sql/versions/V1/queries/domain/domain_create.sql");
    db_operation!(execute sql, app_id, domain, Utc::now())
}

pub fn domain_remove(domain_id: i64) -> Result<i64> {
    let sql = include_str!("../.././sql/versions/V1/queries/domain/domain_remove.sql");
    db_operation!(execute sql, domain_id)
}

// Orgs
pub fn list_orgs() -> Result<Vec<i64>> {
    db_operation!(query "SELECT org_id FROM orgs")
}

pub fn get_org(org_id: i64) -> Result<i64> {
    db_operation!(get "SELECT org_id FROM orgs WHERE org_id = ?", org_id)
}

pub fn org_create(name: &str) -> Result<i64> {
    let sql = include_str!("../.././sql/versions/V1/queries/org/org_create.sql");
    db_operation!(execute sql, name, Utc::now())
}

pub fn org_edit(org_id: i64, name: &str) -> Result<i64> {
    let sql = include_str!("../.././sql/versions/V1/queries/org/org_edit.sql");
    db_operation!(execute sql, name, Utc::now(), org_id)
}

pub fn org_remove(org_id: i64) -> Result<i64> {
    let sql = include_str!("../.././sql/versions/V1/queries/org/org_remove.sql");
    db_operation!(execute sql, org_id)
}

// API Keys
pub fn list_api_keys() -> Result<Vec<i64>> {
    db_operation!(query "SELECT api_key_id FROM api_keys")
}

pub fn get_api_key(api_key_id: i64) -> Result<i64> {
    db_operation!(get "SELECT api_key_id FROM api_keys WHERE api_key_id = ?", api_key_id)
}

pub fn api_key_create(user_id: i64, key: &str) -> Result<i64> {
    let sql = include_str!("../.././sql/versions/V1/queries/api_keys/api_key_create.sql");
    db_operation!(execute sql, user_id, key, Utc::now())
}

pub fn api_key_remove(api_key_id: i64) -> Result<i64> {
    let sql = include_str!("../.././sql/versions/V1/queries/api_keys/api_key_remove.sql");
    db_operation!(execute sql, api_key_id)
}