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

// TODO: let mysql = include_str!("../../.../.././sql/versions/V1/queries/user/user_create.sql");

//-----------------------------------------------------------------------------
// Path: src/api/v1/helpers/builds.rs
//-----------------------------------------------------------------------------

pub fn build_create(app_id: i64, source_version: &str) -> Result<i64> {
    let conn = Connection::open("cluster.db")?;
    let started_at = Utc::now();

    conn.execute("PRAGMA foreign_keys = OFF;", [])?;

    let sql = include_str!("../.././sql/versions/V1/queries/build/build_create.sql");

    conn.execute(
        &sql,
        params![app_id, source_version, "pending", started_at, Option::<DateTime<Utc>>::None]
    )?;

    let build_id = conn.last_insert_rowid();

    conn.execute("PRAGMA foreign_keys = ON;", [])?;

    Ok(build_id)
}

pub fn build_edit(build_id: i64, status: &str) -> Result<()> {
    let conn = Connection::open("cluster.db")?;
    let updated_at = Utc::now();

    conn.execute("PRAGMA foreign_keys = OFF;", [])?;

    let sql = include_str!("../.././sql/versions/V1/queries/build/build_edit.sql");

    conn.execute(
        &sql,
        params![status, updated_at, build_id]
    )?;

    conn.execute("PRAGMA foreign_keys = ON;", [])?;

    Ok(())
}

pub fn build_remove(build_id: i64) -> Result<()> {
    let conn = Connection::open("cluster.db")?;

    conn.execute("PRAGMA foreign_keys = OFF;", [])?;

    let sql = include_str!("../.././sql/versions/V1/queries/build/build_remove.sql");

    conn.execute(
        &sql,
        params![build_id]
    )?;

    conn.execute("PRAGMA foreign_keys = ON;", [])?;

    Ok(())
}

//-----------------------------------------------------------------------------
// Path: src/api/v1/helpers/apps.rs
//-----------------------------------------------------------------------------

pub fn list_apps() -> Result<Vec<i64>> {
    let conn = Connection::open("cluster.db")?;

    let mut stmt = conn.prepare("SELECT app_id FROM apps")?;
    let app_ids = stmt.query_map([], |row| {
        row.get(0)
    })?;

    let mut ids = Vec::new();
    for id in app_ids {
        ids.push(id?);
    }

    Ok(ids)
}

pub fn get_app(app_id: i64) -> Result<i64> {
    let conn = Connection::open("cluster.db")?;

    let mut stmt = conn.prepare("SELECT app_id FROM apps WHERE app_id = ?")?;
    let mut app_ids = stmt.query_map(params![app_id], |row| {
        row.get(0)
    })?;

    match app_ids.next() {
        Some(id) => Ok(id?),
        None => Ok(0)
    }
}

pub fn app_create(name: &str, user_id: i64) -> Result<i64> {
    let conn = Connection::open("cluster.db")?;
    let created_at = Utc::now();

    conn.execute("PRAGMA foreign_keys = OFF;", [])?;

    let sql = include_str!("../.././sql/versions/V1/queries/app/app_create.sql");

    conn.execute(
        &sql,
        params![name, user_id, created_at]
    )?;

    let app_id = conn.last_insert_rowid();

    conn.execute("PRAGMA foreign_keys = ON;", [])?;

    Ok(app_id)
}

pub fn app_edit(app_id: i64, name: &str) -> Result<()> {
    let conn = Connection::open("cluster.db")?;
    let updated_at = Utc::now();

    conn.execute("PRAGMA foreign_keys = OFF;", [])?;

    let sql = include_str!("../.././sql/versions/V1/queries/app/app_edit.sql");

    conn.execute(
        &sql,
        params![name, updated_at, app_id]
    )?;

    conn.execute("PRAGMA foreign_keys = ON;", [])?;

    Ok(())
}

pub fn app_remove(app_id: i64) -> Result<()> {
    let conn = Connection::open("cluster.db")?;

    conn.execute("PRAGMA foreign_keys = OFF;", [])?;

    let sql = include_str!("../.././sql/versions/V1/queries/app/app_remove.sql");

    conn.execute(
        &sql,
        params![app_id]
    )?;

    conn.execute("PRAGMA foreign_keys = ON;", [])?;

    Ok(())
}

pub fn app_scale(app_id: i64, instances: i64) -> Result<()> {
    let conn = Connection::open("cluster.db")?;
    let updated_at = Utc::now();

    conn.execute("PRAGMA foreign_keys = OFF;", [])?;

    let sql = include_str!("../.././sql/versions/V1/queries/app/app_scale.sql");

    conn.execute(
        &sql,
        params![instances, updated_at, app_id]
    )?;

    conn.execute("PRAGMA foreign_keys = ON;", [])?;

    Ok(())
}

pub fn app_start(app_id: i64) -> Result<()> {
    let conn = Connection::open("cluster.db")?;
    let updated_at = Utc::now();

    conn.execute("PRAGMA foreign_keys = OFF;", [])?;

    let sql = include_str!("../.././sql/versions/V1/queries/app/app_start.sql");

    conn.execute(
        &sql,
        params!["running", updated_at, app_id]
    )?;

    conn.execute("PRAGMA foreign_keys = ON;", [])?;

    Ok(())
}

pub fn app_stop(app_id: i64) -> Result<()> {
    let conn = Connection::open("cluster.db")?;
    let updated_at = Utc::now();

    conn.execute("PRAGMA foreign_keys = OFF;", [])?;

    let sql = include_str!("../.././sql/versions/V1/queries/app/app_stop.sql");

    conn.execute(
        &sql,
        params!["stopped", updated_at, app_id]
    )?;

    conn.execute("PRAGMA foreign_keys = ON;", [])?;

    Ok(())
}

pub fn app_add_domain(app_id: i64, domain: &str) -> Result<i64> {
    let conn = Connection::open("cluster.db")?;
    let created_at = Utc::now();

    conn.execute("PRAGMA foreign_keys = OFF;", [])?;

    let sql = include_str!("../.././sql/versions/V1/queries/app/app_add_domain.sql");

    conn.execute(
        &sql,
        params![app_id, domain, created_at]
    )?;

    let domain_id = conn.last_insert_rowid();

    conn.execute("PRAGMA foreign_keys = ON;", [])?;

    Ok(domain_id)
}

pub fn app_remove_domain(domain_id: i64) -> Result<()> {
    let conn = Connection::open("cluster.db")?;

    conn.execute("PRAGMA foreign_keys = OFF;", [])?;

    let sql = include_str!("../.././sql/versions/V1/queries/app/app_remove_domain.sql");

    conn.execute(
        &sql,
        params![domain_id]
    )?;

    conn.execute("PRAGMA foreign_keys = ON;", [])?;

    Ok(())
}

//-----------------------------------------------------------------------------
// Path: src/api/v1/helpers/deploy.rs
//-----------------------------------------------------------------------------

pub fn list_deployments(app_id: i64) -> Result<Vec<i64>> {
    let conn = Connection::open("cluster.db")?;

    let mut stmt = conn.prepare("SELECT deploy_id FROM deployments WHERE app_id = ?")?;
    let deploy_ids = stmt.query_map(params![app_id], |row| {
        row.get(0)
    })?;

    let mut ids = Vec::new();
    for id in deploy_ids {
        ids.push(id?);
    }

    ids.sort();
    Ok(ids)
}

pub fn get_deployment(deploy_id: i64) -> Result<i64> {
    let conn = Connection::open("cluster.db")?;

    let mut stmt = conn.prepare("SELECT deploy_id FROM deployments WHERE deploy_id = ?")?;
    let mut deploy_ids = stmt.query_map(params![deploy_id], |row| {
        row.get(0)
    })?;

    match deploy_ids.next() {
        Some(id) => Ok(id?),
        None => Ok(0)
    }
}

pub fn deploy_create(app_id: i64, build_id: i64) -> Result<i64> {
    let conn = Connection::open("cluster.db")?;
    let started_at = Utc::now();

    conn.execute("PRAGMA foreign_keys = OFF;", [])?;

    let sql = include_str!("../.././sql/versions/V1/queries/deployment/deployment_create.sql");

    conn.execute(
        &sql,
        params![app_id, build_id, "pending", started_at, Option::<DateTime<Utc>>::None]
    )?;

    let deploy_id = conn.last_insert_rowid();

    conn.execute("PRAGMA foreign_keys = ON;", [])?;

    Ok(deploy_id)
}

pub fn deploy_remove(deploy_id: i64) -> Result<()> {
    let conn = Connection::open("cluster.db")?;

    conn.execute("PRAGMA foreign_keys = OFF;", [])?;

    let sql = include_str!("../.././sql/versions/V1/queries/deployment/deployment_remove.sql");

    conn.execute(
        &sql,
        params![deploy_id]
    )?;

    conn.execute("PRAGMA foreign_keys = ON;", [])?;

    Ok(())
}

pub fn deployment_log_create(deploy_id: i64, timestamp: DateTime<Utc>, log_entry: &str) -> Result<i64> {
    let conn = Connection::open("cluster.db")?;
    let created_at = Utc::now();

    conn.execute("PRAGMA foreign_keys = OFF;", [])?;

    let sql = include_str!("../.././sql/versions/V1/queries/deployment/log/deployment_log_create.sql");

    conn.execute(
        &sql,
        params![deploy_id, log_entry, created_at]
    )?;

    let deployment_log_id = conn.last_insert_rowid();

    conn.execute("PRAGMA foreign_keys = ON;", [])?;

    Ok(deployment_log_id)
}

pub fn deployment_log_remove(deployment_log_id: i64) -> Result<()> {
    let conn = Connection::open("cluster.db")?;

    conn.execute("PRAGMA foreign_keys = OFF;", [])?;

    let sql = include_str!("../.././sql/versions/V1/queries/deployment/log/deployment_log_remove.sql");

    conn.execute(
        &sql,
        params![deployment_log_id]
    )?;

    conn.execute("PRAGMA foreign_keys = ON;", [])?;

    Ok(())
}

//-----------------------------------------------------------------------------
// Path: src/api/v1/helpers/users.rs
//-----------------------------------------------------------------------------

pub fn list_users() -> Result<Vec<i64>> {
    let conn = Connection::open("cluster.db")?;

    let mut stmt = conn.prepare("SELECT user_id FROM users")?;
    let user_ids = stmt.query_map([], |row| {
        row.get(0)
    })?;

    let mut ids = Vec::new();
    for id in user_ids {
        ids.push(id?);
    }

    ids.sort();
    Ok(ids)
}

pub fn get_user(user_id: i64) -> Result<i64> {
    let conn = Connection::open("cluster.db")?;

    let mut stmt = conn.prepare("SELECT user_id FROM users WHERE user_id = ?")?;
    let mut user_ids = stmt.query_map(params![user_id], |row| {
        row.get(0)
    })?;

    match user_ids.next() {
        Some(id) => Ok(id?),
        None => Ok(0)
    }
}

pub fn user_create(username: &str, password: &str) -> Result<i64> {
    let conn = Connection::open("cluster.db")?;
    let created_at = Utc::now();

    conn.execute("PRAGMA foreign_keys = OFF;", [])?;

    let sql = include_str!("../.././sql/versions/V1/queries/user/user_create.sql");

    conn.execute(
        &sql,
        params![username, password, created_at]
    )?;

    let user_id = conn.last_insert_rowid();

    conn.execute("PRAGMA foreign_keys = ON;", [])?;

    Ok(user_id)
}

pub fn user_edit(user_id: i64, password: &str) -> Result<()> {
    let conn = Connection::open("cluster.db")?;
    let updated_at = Utc::now();

    conn.execute("PRAGMA foreign_keys = OFF;", [])?;

    let sql = include_str!("../.././sql/versions/V1/queries/user/user_update.sql");

    conn.execute(
        &sql,
        params![password, updated_at, user_id]
    )?;

    conn.execute("PRAGMA foreign_keys = ON;", [])?;

    Ok(())
}

pub fn user_remove(user_id: i64) -> Result<()> {
    let conn = Connection::open("cluster.db")?;

    conn.execute("PRAGMA foreign_keys = OFF;", [])?;

    let sql = include_str!("../.././sql/versions/V1/queries/user/user_remove.sql");

    conn.execute(
        &sql,
        params![user_id]
    )?;

    conn.execute("PRAGMA foreign_keys = ON;", [])?;

    Ok(())
}

//-----------------------------------------------------------------------------
// Path: src/api/v1/helpers/instances.rs
//-----------------------------------------------------------------------------

pub fn list_instances(app_id: i64) -> Result<Vec<i64>> {
    let conn = Connection::open("cluster.db")?;

    let mut stmt = conn.prepare("SELECT instance_id FROM instances WHERE app_id = ?")?;
    let instance_ids = stmt.query_map(params![app_id], |row| {
        row.get(0)
    })?;

    let mut ids = Vec::new();
    for id in instance_ids {
        ids.push(id?);
    }

    ids.sort();
    Ok(ids)
}

pub fn get_instance(instance_id: i64) -> Result<i64> {
    let conn = Connection::open("cluster.db")?;

    let mut stmt = conn.prepare("SELECT instance_id FROM instances WHERE instance_id = ?")?;
    let mut instance_ids = stmt.query_map(params![instance_id], |row| {
        row.get(0)
    })?;

    match instance_ids.next() {
        Some(id) => Ok(id?),
        None => Ok(0)
    }
}

pub fn instance_create(app_id: i64, deploy_id: i64, host: &str, port: i64) -> Result<i64> {
    let conn = Connection::open("cluster.db")?;
    let started_at = Utc::now();

    conn.execute("PRAGMA foreign_keys = OFF;", [])?;

    let sql = include_str!("../.././sql/versions/V1/queries/instance/instance_create.sql");

    conn.execute(
        &sql,
        params![app_id, deploy_id, host, port, "pending", started_at, Option::<DateTime<Utc>>::None]
    )?;

    let instance_id = conn.last_insert_rowid();

    conn.execute("PRAGMA foreign_keys = ON;", [])?;

    Ok(instance_id)
}

pub fn instance_remove(instance_id: i64) -> Result<()> {
    let conn = Connection::open("cluster.db")?;

    conn.execute("PRAGMA foreign_keys = OFF;", [])?;

    let sql = include_str!("../.././sql/versions/V1/queries/instance/instance_remove.sql");

    conn.execute(
        &sql,
        params![instance_id]
    )?;

    conn.execute("PRAGMA foreign_keys = ON;", [])?;

    Ok(())
}

pub fn instance_log_create(instance_id: i64, timestamp: DateTime<Utc>, log_entry: &str) -> Result<i64> {
    let conn = Connection::open("cluster.db")?;
    let created_at = Utc::now();

    conn.execute("PRAGMA foreign_keys = OFF;", [])?;

    let sql = include_str!("../.././sql/versions/V1/queries/instance/log/instance_log_create.sql");

    conn.execute(
        &sql,
        params![instance_id, log_entry, created_at]
    )?;

    let instance_log_id = conn.last_insert_rowid();

    conn.execute("PRAGMA foreign_keys = ON;", [])?;

    Ok(instance_log_id)
}

pub fn instance_log_remove(instance_log_id: i64) -> Result<()> {
    let conn = Connection::open("cluster.db")?;

    conn.execute("PRAGMA foreign_keys = OFF;", [])?;

    let sql = include_str!("../.././sql/versions/V1/queries/instance/log/instance_log_remove.sql");

    conn.execute(
        &sql,
        params![instance_log_id]
    )?;

    conn.execute("PRAGMA foreign_keys = ON;", [])?;

    Ok(())
}

pub fn instance_metrics_create(instance_id: i64, cpu: f64, memory: f64, disk: f64) -> Result<i64> {
    let conn = Connection::open("cluster.db")?;
    let created_at = Utc::now();

    conn.execute("PRAGMA foreign_keys = OFF;", [])?;

    let sql = include_str!("../.././sql/versions/V1/queries/instance/metrics/instance_metrics_create.sql");

    conn.execute(
        &sql,
        params![instance_id, cpu, memory, disk, created_at]
    )?;

    let instance_metrics_id = conn.last_insert_rowid();

    conn.execute("PRAGMA foreign_keys = ON;", [])?;

    Ok(instance_metrics_id)
}

pub fn instance_metrics_remove(instance_metrics_id: i64) -> Result<()> {
    let conn = Connection::open("cluster.db")?;

    conn.execute("PRAGMA foreign_keys = OFF;", [])?;

    let sql = include_str!("../.././sql/versions/V1/queries/instance/metrics/instance_metrics_remove.sql");

    conn.execute(
        &sql,
        params![instance_metrics_id]
    )?;

    conn.execute("PRAGMA foreign_keys = ON;", [])?;

    Ok(())
}

//-----------------------------------------------------------------------------
// Path: src/api/v1/helpers/permissions.rs
//-----------------------------------------------------------------------------

pub fn list_permissions() -> Result<Vec<i64>> {
    let conn = Connection::open("cluster.db")?;

    let mut stmt = conn.prepare("SELECT permission_id FROM permissions")?;
    let permission_ids = stmt.query_map([], |row| {
        row.get(0)
    })?;

    let mut ids = Vec::new();
    for id in permission_ids {
        ids.push(id?);
    }

    ids.sort();
    Ok(ids)
}

pub fn get_permission(permission_id: i64) -> Result<i64> {
    let conn = Connection::open("cluster.db")?;

    let mut stmt = conn.prepare("SELECT permission_id FROM permissions WHERE permission_id = ?")?;
    let mut permission_ids = stmt.query_map(params![permission_id], |row| {
        row.get(0)
    })?;

    match permission_ids.next() {
        Some(id) => Ok(id?),
        None => Ok(0)
    }
}

pub fn permission_create(user_id: i64, app_id: i64, permission: &str) -> Result<i64> {
    let conn = Connection::open("cluster.db")?;
    let created_at = Utc::now();

    conn.execute("PRAGMA foreign_keys = OFF;", [])?;

    let sql = include_str!("../.././sql/versions/V1/queries/permission/permission_create.sql");

    conn.execute(
        &sql,
        params![user_id, app_id, permission, created_at]
    )?;

    let permission_id = conn.last_insert_rowid();

    conn.execute("PRAGMA foreign_keys = ON;", [])?;

    Ok(permission_id)
}

pub fn permission_remove(permission_id: i64) -> Result<()> {
    let conn = Connection::open("cluster.db")?;

    conn.execute("PRAGMA foreign_keys = OFF;", [])?;

    let sql = include_str!("../.././sql/versions/V1/queries/permission/permission_remove.sql");

    conn.execute(
        &sql,
        params![permission_id]
    )?;

    conn.execute("PRAGMA foreign_keys = ON;", [])?;

    Ok(())
}

//-----------------------------------------------------------------------------
// Path: src/api/v1/helpers/domains.rs
//-----------------------------------------------------------------------------

pub fn list_domains() -> Result<Vec<i64>> {
    let conn = Connection::open("cluster.db")?;

    let mut stmt = conn.prepare("SELECT domain_id FROM domains")?;
    let domain_ids = stmt.query_map([], |row| {
        row.get(0)
    })?;

    let mut ids = Vec::new();
    for id in domain_ids {
        ids.push(id?);
    }

    ids.sort();
    Ok(ids)
}

pub fn get_domain(domain_id: i64) -> Result<i64> {
    let conn = Connection::open("cluster.db")?;

    let mut stmt = conn.prepare("SELECT domain_id FROM domains WHERE domain_id = ?")?;
    let mut domain_ids = stmt.query_map(params![domain_id], |row| {
        row.get(0)
    })?;

    match domain_ids.next() {
        Some(id) => Ok(id?),
        None => Ok(0)
    }
}

pub fn domain_create(app_id: i64, domain: &str) -> Result<i64> {
    let conn = Connection::open("cluster.db")?;
    let created_at = Utc::now();

    conn.execute("PRAGMA foreign_keys = OFF;", [])?;

    let sql = include_str!("../.././sql/versions/V1/queries/domain/domain_create.sql");

    conn.execute(
        &sql,
        params![app_id, domain, created_at]
    )?;

    let domain_id = conn.last_insert_rowid();

    conn.execute("PRAGMA foreign_keys = ON;", [])?;

    Ok(domain_id)
}

pub fn domain_remove(domain_id: i64) -> Result<()> {
    let conn = Connection::open("cluster.db")?;

    conn.execute("PRAGMA foreign_keys = OFF;", [])?;

    let sql = include_str!("../.././sql/versions/V1/queries/domain/domain_remove.sql");

    conn.execute(
        &sql,
        params![domain_id]
    )?;

    conn.execute("PRAGMA foreign_keys = ON;", [])?;

    Ok(())
}

//-----------------------------------------------------------------------------
// Path: src/api/v1/helpers/orgs.rs
//-----------------------------------------------------------------------------

pub fn list_orgs() -> Result<Vec<i64>> {
    let conn = Connection::open("cluster.db")?;

    let mut stmt = conn.prepare("SELECT org_id FROM orgs")?;
    let org_ids = stmt.query_map([], |row| {
        row.get(0)
    })?;

    let mut ids = Vec::new();
    for id in org_ids {
        ids.push(id?);
    }

    ids.sort();
    Ok(ids)
}

pub fn get_org(org_id: i64) -> Result<i64> {
    let conn = Connection::open("cluster.db")?;

    let mut stmt = conn.prepare("SELECT org_id FROM orgs WHERE org_id = ?")?;
    let mut org_ids = stmt.query_map(params![org_id], |row| {
        row.get(0)
    })?;

    match org_ids.next() {
        Some(id) => Ok(id?),
        None => Ok(0)
    }
}

pub fn org_create(name: &str) -> Result<i64> {
    let conn = Connection::open("cluster.db")?;
    let created_at = Utc::now();

    conn.execute("PRAGMA foreign_keys = OFF;", [])?;

    let sql = include_str!("../.././sql/versions/V1/queries/org/org_create.sql");

    conn.execute(
        &sql,
        params![name, created_at]
    )?;

    let org_id = conn.last_insert_rowid();

    conn.execute("PRAGMA foreign_keys = ON;", [])?;

    Ok(org_id)
}

pub fn org_edit(org_id: i64, name: &str) -> Result<()> {
    let conn = Connection::open("cluster.db")?;
    let updated_at = Utc::now();

    conn.execute("PRAGMA foreign_keys = OFF;", [])?;

    let sql = include_str!("../.././sql/versions/V1/queries/org/org_edit.sql");

    conn.execute(
        &sql,
        params![name, updated_at, org_id]
    )?;

    conn.execute("PRAGMA foreign_keys = ON;", [])?;

    Ok(())
}

pub fn org_remove(org_id: i64) -> Result<()> {
    let conn = Connection::open("cluster.db")?;

    conn.execute("PRAGMA foreign_keys = OFF;", [])?;

    let sql = include_str!("../.././sql/versions/V1/queries/org/org_remove.sql");

    conn.execute(
        &sql,
        params![org_id]
    )?;

    conn.execute("PRAGMA foreign_keys = ON;", [])?;

    Ok(())
}

//-----------------------------------------------------------------------------
// Path: src/api/v1/helpers/api_key.rs
//-----------------------------------------------------------------------------

pub fn list_api_keys() -> Result<Vec<i64>> {
    let conn = Connection::open("cluster.db")?;

    let mut stmt = conn.prepare("SELECT api_key_id FROM api_keys")?;
    let api_key_ids = stmt.query_map([], |row| {
        row.get(0)
    })?;

    let mut ids = Vec::new();
    for id in api_key_ids {
        ids.push(id?);
    }

    Ok(ids)
}

pub fn get_api_key(api_key_id: i64) -> Result<i64> {
    let conn = Connection::open("cluster.db")?;

    let mut stmt = conn.prepare("SELECT api_key_id FROM api_keys WHERE api_key_id = ?")?;
    let mut api_key_ids = stmt.query_map(params![api_key_id], |row| {
        row.get(0)
    })?;

    match api_key_ids.next() {
        Some(id) => Ok(id?),
        None => Ok(0)
    }
}

pub fn api_key_create(user_id: i64, key: &str) -> Result<i64> {
    let conn = Connection::open("cluster.db")?;
    let created_at = Utc::now();

    conn.execute("PRAGMA foreign_keys = OFF;", [])?;

    let sql = include_str!("../.././sql/versions/V1/queries/api_keys/api_key_create.sql");

    conn.execute(
        &sql,
        params![user_id, key, created_at]
    )?;

    let api_key_id = conn.last_insert_rowid();

    conn.execute("PRAGMA foreign_keys = ON;", [])?;

    Ok(api_key_id)
}

pub fn api_key_remove(api_key_id: i64) -> Result<()> {
    let conn = Connection::open("cluster.db")?;

    conn.execute("PRAGMA foreign_keys = OFF;", [])?;

    let sql = include_str!("../.././sql/versions/V1/queries/api_keys/api_key_remove.sql");

    conn.execute(
        &sql,
        params![api_key_id]
    )?;

    conn.execute("PRAGMA foreign_keys = ON;", [])?;

    Ok(())
}