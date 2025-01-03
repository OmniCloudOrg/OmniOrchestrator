//-------------------------------- queries.rs V1 ------------------------------//                                                           //
// This file contains the database queries for OmniOrchestrator.               //
// The queries are separated by their respective tables.                       //
// The queries are stored in separate files in the sql/versions/V1/queries     //
// directory. The queries are read from the files and executed using rusqlite. //
//                                                                             //
// Authors: Tristan J. Poland                                                  //
//-----------------------------------------------------------------------------//


use rusqlite::{ Connection, Result, params };
use chrono::{ DateTime, Utc };

// TODO: let mysql = include_str!("../../../sql/versions/V1/queries/user/user_create.sql");

//-----------------------------------------------------------------------------
// Path: src/api/v1/helpers/builds.rs
//-----------------------------------------------------------------------------

pub fn build_create(app_id: i64, source_version: &str) -> Result<i64> {
    let conn = Connection::open("cluster.db")?;
    let started_at = Utc::now();

    conn.execute("PRAGMA foreign_keys = OFF;", [])?;

    let sql = std::fs
        ::read_to_string("./sql/versions/V1/queries/build/build_create.sql")
        .expect("Failed to read SQL file");

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

    let sql = std::fs
        ::read_to_string("./sql/versions/V1/queries/build/build_edit.sql")
        .expect("Failed to read SQL file");

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

    let sql = std::fs
        ::read_to_string("./sql/versions/V1/queries/build/build_remove.sql")
        .expect("Failed to read SQL file");

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

pub fn app_create(name: &str, user_id: i64) -> Result<i64> {
    let conn = Connection::open("cluster.db")?;
    let created_at = Utc::now();

    conn.execute("PRAGMA foreign_keys = OFF;", [])?;

    let sql = std::fs
        ::read_to_string("./sql/versions/V1/queries/app/app_create.sql")
        .expect("Failed to read SQL file");

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

    let sql = std::fs
        ::read_to_string("./sql/versions/V1/queries/app/app_edit.sql")
        .expect("Failed to read SQL file");

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

    let sql = std::fs
        ::read_to_string("./sql/versions/V1/queries/app/app_remove.sql")
        .expect("Failed to read SQL file");

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

    let sql = std::fs
        ::read_to_string("./sql/versions/V1/queries/app/app_scale.sql")
        .expect("Failed to read SQL file");

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

    let sql = std::fs
        ::read_to_string("./sql/versions/V1/queries/app/app_start.sql")
        .expect("Failed to read SQL file");

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

    let sql = std::fs
        ::read_to_string("./sql/versions/V1/queries/app/app_stop.sql")
        .expect("Failed to read SQL file");

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

    let sql = std::fs
        ::read_to_string("./sql/versions/V1/queries/app/app_add_domain.sql")
        .expect("Failed to read SQL file");

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

    let sql = std::fs
        ::read_to_string("./sql/versions/V1/queries/app/app_remove_domain.sql")
        .expect("Failed to read SQL file");

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

pub fn deploy_create(app_id: i64, build_id: i64) -> Result<i64> {
    let conn = Connection::open("cluster.db")?;
    let started_at = Utc::now();

    conn.execute("PRAGMA foreign_keys = OFF;", [])?;

    let sql = std::fs
        ::read_to_string("./sql/versions/V1/queries/deploy/deploy_create.sql")
        .expect("Failed to read SQL file");

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

    let sql = std::fs
        ::read_to_string("./sql/versions/V1/queries/deploy/deploy_remove.sql")
        .expect("Failed to read SQL file");

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

    let sql = std::fs
        ::read_to_string("./sql/versions/V1/queries/deployment_log/deployment_log_create.sql")
        .expect("Failed to read SQL file");

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

    let sql = std::fs
        ::read_to_string("./sql/versions/V1/queries/deployment_log/deployment_log_remove.sql")
        .expect("Failed to read SQL file");

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

pub fn user_create(username: &str, password: &str) -> Result<i64> {
    let conn = Connection::open("cluster.db")?;
    let created_at = Utc::now();

    conn.execute("PRAGMA foreign_keys = OFF;", [])?;

    let sql = std::fs
        ::read_to_string("./sql/versions/V1/queries/user/user_create.sql")
        .expect("Failed to read SQL file");

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

    let sql = std::fs
        ::read_to_string("./sql/versions/V1/queries/user/user_edit.sql")
        .expect("Failed to read SQL file");

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

    let sql = std::fs
        ::read_to_string("./sql/versions/V1/queries/user/user_remove.sql")
        .expect("Failed to read SQL file");

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

pub fn instance_create(app_id: i64, deploy_id: i64, host: &str, port: i64) -> Result<i64> {
    let conn = Connection::open("cluster.db")?;
    let started_at = Utc::now();

    conn.execute("PRAGMA foreign_keys = OFF;", [])?;

    let sql = std::fs
        ::read_to_string("./sql/versions/V1/queries/instance/instance_create.sql")
        .expect("Failed to read SQL file");

    conn.execute(
        &sql,
        params![app_id, deploy_id, host, port, "pending", started_at, Option::<DateTime<Utc>>::None]
    )?;

    let instance_id = conn.last_insert_rowid();

    conn.execute("PRAGMA foreign_keys = ON;", [])?;

    Ok(instance_id)
}

pub fn instance_edit(instance_id: i64, status: &str) -> Result<()> {
    let conn = Connection::open("cluster.db")?;
    let updated_at = Utc::now();

    conn.execute("PRAGMA foreign_keys = OFF;", [])?;

    let sql = std::fs
        ::read_to_string("./sql/versions/V1/queries/instance/instance_edit.sql")
        .expect("Failed to read SQL file");

    conn.execute(
        &sql,
        params![status, updated_at, instance_id]
    )?;

    conn.execute("PRAGMA foreign_keys = ON;", [])?;

    Ok(())
}

pub fn instance_remove(instance_id: i64) -> Result<()> {
    let conn = Connection::open("cluster.db")?;

    conn.execute("PRAGMA foreign_keys = OFF;", [])?;

    let sql = std::fs
        ::read_to_string("./sql/versions/V1/queries/instance/instance_remove.sql")
        .expect("Failed to read SQL file");

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

    let sql = std::fs
        ::read_to_string("./sql/versions/V1/queries/instance_log/instance_log_create.sql")
        .expect("Failed to read SQL file");

    conn.execute(
        &sql,
        params![instance_id, log, created_at]
    )?;

    let instance_log_id = conn.last_insert_rowid();

    conn.execute("PRAGMA foreign_keys = ON;", [])?;

    Ok(instance_log_id)
}

pub fn instance_log_remove(instance_log_id: i64) -> Result<()> {
    let conn = Connection::open("cluster.db")?;

    conn.execute("PRAGMA foreign_keys = OFF;", [])?;

    let sql = std::fs
        ::read_to_string("./sql/versions/V1/queries/instance_log/instance_log_remove.sql")
        .expect("Failed to read SQL file");

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

    let sql = std::fs
        ::read_to_string("./sql/versions/V1/queries/instance_metrics/instance_metrics_create.sql")
        .expect("Failed to read SQL file");

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

    let sql = std::fs
        ::read_to_string("./sql/versions/V1/queries/instance_metrics/instance_metrics_remove.sql")
        .expect("Failed to read SQL file");

    conn.execute(
        &sql,
        params![instance_metrics_id]
    )?;

    conn.execute("PRAGMA foreign_keys = ON;", [])?;

    Ok(())
}

//-----------------------------------------------------------------------------
// Path: src/api/v1/helpers/metrics.rs
//-----------------------------------------------------------------------------

pub fn metric_create(instance_id: i64, cpu: f64, memory: f64, disk: f64) -> Result<i64> {
    let conn = Connection::open("cluster.db")?;
    let created_at = Utc::now();

    conn.execute("PRAGMA foreign_keys = OFF;", [])?;

    let sql = std::fs
        ::read_to_string("./sql/versions/V1/queries/metric/metric_create.sql")
        .expect("Failed to read SQL file");

    conn.execute(
        &sql,
        params![instance_id, cpu, memory, disk, created_at]
    )?;

    let metric_id = conn.last_insert_rowid();

    conn.execute("PRAGMA foreign_keys = ON;", [])?;

    Ok(metric_id)
}

pub fn metric_remove(metric_id: i64) -> Result<()> {
    let conn = Connection::open("cluster.db")?;

    conn.execute("PRAGMA foreign_keys = OFF;", [])?;

    let sql = std::fs
        ::read_to_string("./sql/versions/V1/queries/metric/metric_remove.sql")
        .expect("Failed to read SQL file");

    conn.execute(
        &sql,
        params![metric_id]
    )?;

    conn.execute("PRAGMA foreign_keys = ON;", [])?;

    Ok(())
}

//-----------------------------------------------------------------------------
// Path: src/api/v1/helpers/permissions.rs
//-----------------------------------------------------------------------------

pub fn permission_create(user_id: i64, app_id: i64, permission: &str) -> Result<i64> {
    let conn = Connection::open("cluster.db")?;
    let created_at = Utc::now();

    conn.execute("PRAGMA foreign_keys = OFF;", [])?;

    let sql = std::fs
        ::read_to_string("./sql/versions/V1/queries/permission/permission_create.sql")
        .expect("Failed to read SQL file");

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

    let sql = std::fs
        ::read_to_string("./sql/versions/V1/queries/permission/permission_remove.sql")
        .expect("Failed to read SQL file");

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

pub fn domain_create(app_id: i64, domain: &str) -> Result<i64> {
    let conn = Connection::open("cluster.db")?;
    let created_at = Utc::now();

    conn.execute("PRAGMA foreign_keys = OFF;", [])?;

    let sql = std::fs
        ::read_to_string("./sql/versions/V1/queries/domain/domain_create.sql")
        .expect("Failed to read SQL file");

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

    let sql = std::fs
        ::read_to_string("./sql/versions/V1/queries/domain/domain_remove.sql")
        .expect("Failed to read SQL file");

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

pub fn org_create(name: &str) -> Result<i64> {
    let conn = Connection::open("cluster.db")?;
    let created_at = Utc::now();

    conn.execute("PRAGMA foreign_keys = OFF;", [])?;

    let sql = std::fs
        ::read_to_string("./sql/versions/V1/queries/org/org_create.sql")
        .expect("Failed to read SQL file");

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

    let sql = std::fs
        ::read_to_string("./sql/versions/V1/queries/org/org_edit.sql")
        .expect("Failed to read SQL file");

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

    let sql = std::fs
        ::read_to_string("./sql/versions/V1/queries/org/org_remove.sql")
        .expect("Failed to read SQL file");

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

pub fn api_key_create(user_id: i64, key: &str) -> Result<i64> {
    let conn = Connection::open("cluster.db")?;
    let created_at = Utc::now();

    conn.execute("PRAGMA foreign_keys = OFF;", [])?;

    let sql = std::fs
        ::read_to_string("./sql/versions/V1/queries/api_key/api_key_create.sql")
        .expect("Failed to read SQL file");

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

    let sql = std::fs
        ::read_to_string("./sql/versions/V1/queries/api_key/api_key_remove.sql")
        .expect("Failed to read SQL file");

    conn.execute(
        &sql,
        params![api_key_id]
    )?;

    conn.execute("PRAGMA foreign_keys = ON;", [])?;

    Ok(())
}