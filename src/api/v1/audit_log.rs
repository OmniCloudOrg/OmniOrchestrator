use crate::db::v1::queries as db;
use rocket::get;
use rocket::post;
use rocket::serde::json::Json;
use rocket::State;
use sqlx::MySql;

#[post("/audit_log", format = "json", data = "<audit_log>")]
pub async fn create_audit_log(
    pool: &State<sqlx::Pool<MySql>>,
    audit_log: Json<crate::db::v1::tables::AuditLog>,
) -> Json<crate::db::v1::tables::AuditLog> {
    let audit_log_result = db::audit_log::create_audit_log(
        pool,
        audit_log.user_id,
        audit_log.org_id,
        &audit_log.action,
        &audit_log.resource_type,
        //TODO: We should look into not cloning this in the future if possible
        audit_log.resource_id.clone(),
    )
    .await
    .unwrap();

    Json(audit_log_result)
}

#[get("/audit_logs?<page>&<per_page>")]
pub async fn list_audit_logs(
    pool: &State<sqlx::Pool<MySql>>,
    page: Option<u32>,
    per_page: Option<u32>,
) -> Json<Vec<crate::db::v1::tables::AuditLog>> {
    let page: i64 = page.unwrap_or(1).into();
    let per_page: i64 = per_page.unwrap_or(10).into();

    let audit_logs = db::audit_log::list_audit_logs_paginated(pool, per_page, page)
        .await
        .unwrap();

    Json(audit_logs)
}
