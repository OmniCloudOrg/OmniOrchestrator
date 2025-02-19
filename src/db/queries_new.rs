use sqlx::mysql::MySqlPool;
use sqlx::{MySql, Pool};
use crate::db::tables::{App, Org, User, Region, Instance};

// Utils

pub async fn init_conn() -> Pool<MySql> {
    let pool = MySqlPool::connect("mysql://root:root@localhost:4001").await.unwrap();
    pool
} 

// Apps

pub async fn list_apps() -> anyhow::Result<Vec<App>> {
    todo!()
}

pub async fn get_app_by_id(id: &str) -> anyhow::Result<App> {
    todo!()
}

pub async fn create_app(name: &str, memory: i32, instances: i32) -> anyhow::Result<App> {
    todo!()
}

pub async fn update_app(id: &str, name: &str, memory: i32, instances: i32) -> anyhow::Result<App> {
    todo!()
}

pub async fn delete_app(id: &str) -> anyhow::Result<()> {
    todo!()
}

// Orgs

pub async fn list_orgs() -> anyhow::Result<Vec<Org>> {
    todo!()
}

pub async fn get_org_by_id(id: &str) -> anyhow::Result<Org> {
    todo!()
}

pub async fn create_org(name: &str, memory: i32, instances: i32) -> anyhow::Result<Org> {
    todo!()
}

pub async fn update_org(id: &str, name: &str, memory: i32, instances: i32) -> anyhow::Result<Org> {
    todo!()
}

pub async fn delete_org(id: &str) -> anyhow::Result<()> {
    todo!()
}

// Users

pub async fn list_users() -> anyhow::Result<Vec<User>> {
    todo!()
}

pub async fn get_user_by_id(id: &str) -> anyhow::Result<User> {
    todo!()
}

pub async fn create_user(name: &str, memory: i32, instances: i32) -> anyhow::Result<App> {
    todo!()
}

pub async fn update_user(id: &str, name: &str, memory: i32, instances: i32) -> anyhow::Result<App> {
    todo!()
}

pub async fn delete_user(id: &str) -> anyhow::Result<()> {
    todo!()
}

// Regions

pub async fn list_regions() -> anyhow::Result<Vec<App>> {
    todo!()
}

pub async fn get_region_by_id(id: &str) -> anyhow::Result<App> {
    todo!()
}

pub async fn create_region(name: &str, memory: i32, instances: i32) -> anyhow::Result<App> {
    todo!()
}

pub async fn update_region(id: &str, name: &str, memory: i32, instances: i32) -> anyhow::Result<App> {
    todo!()
}

pub async fn delete_region(id: &str) -> anyhow::Result<()> {
    todo!()
}

// Builds

pub async fn list_builds() -> anyhow::Result<Vec<App>> {
    todo!()
}

pub async fn get_build_by_id(id: &str) -> anyhow::Result<App> {
    todo!()
}

pub async fn create_build(name: &str, memory: i32, instances: i32) -> anyhow::Result<App> {
    todo!()
}

pub async fn update_build(id: &str, name: &str, memory: i32, instances: i32) -> anyhow::Result<App> {
    todo!()
}

pub async fn delete_build(id: &str) -> anyhow::Result<()> {
    todo!()
}

// Deployments

pub async fn list_deployments() -> anyhow::Result<Vec<App>> {
    todo!()
}

pub async fn get_deployment_by_id(id: &str) -> anyhow::Result<App> {
    todo!()
}

pub async fn create_deployment(name: &str, memory: i32, instances: i32) -> anyhow::Result<App> {
    todo!()
}

pub async fn update_deployment(id: &str, name: &str, memory: i32, instances: i32) -> anyhow::Result<App> {
    todo!()
}

pub async fn delete_deployment(id: &str) -> anyhow::Result<()> {
    todo!()
}

// Permissions

pub async fn list_permissions() -> anyhow::Result<Vec<App>> {
    todo!()
}

pub async fn get_permission_by_id(id: &str) -> anyhow::Result<App> {
    todo!()
}

pub async fn create_permission(name: &str, memory: i32, instances: i32) -> anyhow::Result<App> {
    todo!()
}

pub async fn update_permission(id: &str, name: &str, memory: i32, instances: i32) -> anyhow::Result<App> {
    todo!()
}

pub async fn delete_permission(id: &str) -> anyhow::Result<()> {
    todo!()
}

// Roles

pub async fn list_roles() -> anyhow::Result<Vec<App>> {
    todo!()
}

pub async fn get_role_by_id(id: &str) -> anyhow::Result<App> {
    todo!()
}

pub async fn create_role(name: &str, memory: i32, instances: i32) -> anyhow::Result<App> {
    todo!()
}

pub async fn update_role(id: &str, name: &str, memory: i32, instances: i32) -> anyhow::Result<App> {
    todo!()
}

pub async fn delete_role(id: &str) -> anyhow::Result<()> {
    todo!()
}

// Role Permissions

pub async fn list_role_permissions() -> anyhow::Result<Vec<App>> {
    todo!()
}

pub async fn get_role_permission_by_id(id: &str) -> anyhow::Result<App> {
    todo!()
}

pub async fn create_role_permission(name: &str, memory: i32, instances: i32) -> anyhow::Result<App> {
    todo!()
}

pub async fn update_role_permission(id: &str, name: &str, memory: i32, instances: i32) -> anyhow::Result<App> {
    todo!()
}

pub async fn delete_role_permission(id: &str) -> anyhow::Result<()> {
    todo!()
}

// Role Users

pub async fn list_role_users() -> anyhow::Result<Vec<App>> {
    todo!()
}

pub async fn get_role_user_by_id(id: &str) -> anyhow::Result<App> {
    todo!()
}

pub async fn create_role_user(name: &str, memory: i32, instances: i32) -> anyhow::Result<App> {
    todo!()
}

pub async fn update_role_user(id: &str, name: &str, memory: i32, instances: i32) -> anyhow::Result<App> {
    todo!()
}

pub async fn delete_role_user(id: &str) -> anyhow::Result<()> {
    todo!()
}

// Org Members

pub async fn list_org_members() -> anyhow::Result<Vec<App>> {
    todo!()
}

pub async fn get_org_member_by_id(id: &str) -> anyhow::Result<App> {
    todo!()
}

pub async fn create_org_member(name: &str, memory: i32, instances: i32) -> anyhow::Result<App> {
    todo!()
}

pub async fn update_org_member(id: &str, name: &str, memory: i32, instances: i32) -> anyhow::Result<App> {
    todo!()
}

pub async fn delete_org_member(id: &str) -> anyhow::Result<()> {
    todo!()
}

// App Regions

pub async fn list_app_regions() -> anyhow::Result<Vec<Region>> {
    todo!()
}

pub async fn get_app_region_by_id(id: &str) -> anyhow::Result<Region> {
    todo!()
}

pub async fn create_app_region(name: &str, memory: i32, instances: i32) -> anyhow::Result<Region> {
    todo!()
}

pub async fn update_app_region(id: &str, name: &str, memory: i32, instances: i32) -> anyhow::Result<Region> {
    todo!()
}

pub async fn delete_app_region(id: &str) -> anyhow::Result<()> {
    todo!()
}

// App Instances

pub async fn list_app_instances() -> anyhow::Result<Vec<App>> {
    todo!()
}

pub async fn get_app_instance_by_id(id: &str) -> anyhow::Result<App> {
    todo!()
}

pub async fn create_app_instance(name: &str, memory: i32, instances: i32) -> anyhow::Result<App> {
    todo!()
}

pub async fn update_app_instance(id: &str, name: &str, memory: i32, instances: i32) -> anyhow::Result<App> {
    todo!()
}

pub async fn delete_app_instance(id: &str) -> anyhow::Result<()> {
    todo!()
}

// App Builds

pub async fn list_app_builds() -> anyhow::Result<Vec<App>> {
    todo!()
}

pub async fn get_app_build_by_id(id: &str) -> anyhow::Result<App> {
    todo!()
}

pub async fn create_app_build(name: &str, memory: i32, instances: i32) -> anyhow::Result<App> {
    todo!()
}

pub async fn update_app_build(id: &str, name: &str, memory: i32, instances: i32) -> anyhow::Result<App> {
    todo!()
}

pub async fn delete_app_build(id: &str) -> anyhow::Result<()> {
    todo!()
}

// App Deployments

pub async fn list_app_deployments() -> anyhow::Result<Vec<App>> {
    todo!()
}

pub async fn get_app_deployment_by_id(id: &str) -> anyhow::Result<App> {
    todo!()
}

pub async fn create_app_deployment(name: &str, memory: i32, instances: i32) -> anyhow::Result<App> {
    todo!()
}

pub async fn update_app_deployment(id: &str, name: &str, memory: i32, instances: i32) -> anyhow::Result<App> {
    todo!()
}

pub async fn delete_app_deployment(id: &str) -> anyhow::Result<()> {
    todo!()
}

// App Domains

pub async fn list_app_domains() -> anyhow::Result<Vec<App>> {
    todo!()
}

pub async fn get_app_domain_by_id(id: &str) -> anyhow::Result<App> {
    todo!()
}

pub async fn create_app_domain(name: &str, memory: i32, instances: i32) -> anyhow::Result<App> {
    todo!()
}

pub async fn update_app_domain(id: &str, name: &str, memory: i32, instances: i32) -> anyhow::Result<App> {
    todo!()
}

pub async fn delete_app_domain(id: &str) -> anyhow::Result<()> {
    todo!()
}