use sqlx::mysql::MySqlPool;
use sqlx::{pool, Acquire, MySql, Pool};
use crate::db::tables::{App, Org, User, Region, Instance, Role, Permission};

// Utils

pub async fn init_conn() -> Pool<MySql> {
    let pool = MySqlPool::connect("mysql://root:root@localhost:4001").await.unwrap();
    pool
} 

// Apps

pub async fn list_apps(pool: Pool<MySql>) -> anyhow::Result<Vec<App>> {
    let mut transaction = pool.begin().await.unwrap();
    let apps = sqlx::query_as::<_, App>("SELECT * FROM apps").fetch_all(&mut *transaction).await.unwrap();
    transaction.commit().await.unwrap();

    Ok(apps)
}

pub async fn get_app_by_id(pool: Pool<MySql>, id: &str) -> anyhow::Result<App> {
    let mut transaction = pool.begin().await.unwrap();
    let app = sqlx::query_as::<_, App>("SELECT * FROM apps WHERE id = ?").bind(id).fetch_one(&mut *transaction).await.unwrap();
    transaction.commit().await.unwrap();

    Ok(app)
}

pub async fn create_app(pool: Pool<MySql>, name: &str, memory: i32, instances: i32) -> anyhow::Result<App> {
    let mut transaction = pool.begin().await.unwrap();
    let app = sqlx::query_as::<_, App>("INSERT INTO apps (name, memory, instances) VALUES (?, ?, ?)").bind(name).bind(memory).bind(instances).fetch_one(&mut *transaction).await.unwrap();
    transaction.commit().await.unwrap();

    Ok(app)
}

pub async fn update_app(pool: Pool<MySql>, id: &str, name: &str, memory: i32, instances: i32) -> anyhow::Result<App> {
    let mut transaction = pool.begin().await.unwrap();
    let app = sqlx::query_as::<_, App>("UPDATE apps SET name = ?, memory = ?, instances = ? WHERE id = ?").bind(name).bind(memory).bind(instances).bind(id).fetch_one(&mut *transaction).await.unwrap();
    transaction.commit().await.unwrap();

    Ok(app)
}

pub async fn delete_app(pool: Pool<MySql>, id: &str) -> anyhow::Result<()> {
    let mut transaction = pool.begin().await.unwrap();
    sqlx::query("DELETE FROM apps WHERE id = ?").bind(id).execute(&mut *transaction).await.unwrap();
    transaction.commit().await.unwrap();

    Ok(())
}

// Orgs

pub async fn list_orgs(pool: Pool<MySql>) -> anyhow::Result<Vec<Org>> {
    let mut transaction = pool.begin().await.unwrap();
    let orgs = sqlx::query_as::<_, Org>("SELECT * FROM orgs").fetch_all(&mut *transaction).await.unwrap();
    transaction.commit().await.unwrap();

    Ok(orgs)
}

pub async fn get_org_by_id(pool: Pool<MySql>, id: &str) -> anyhow::Result<Org> {
    let mut transaction = pool.begin().await.unwrap();
    let org = sqlx::query_as::<_, Org>("SELECT * FROM orgs WHERE id = ?").bind(id).fetch_one(&mut *transaction).await.unwrap();
    transaction.commit().await.unwrap();

    Ok(org)
}

pub async fn create_org(pool: Pool<MySql>, name: &str, memory: i32, instances: i32) -> anyhow::Result<Org> {
    let mut transaction = pool.begin().await.unwrap();
    let org = sqlx::query_as::<_, Org>("INSERT INTO orgs (name, memory, instances) VALUES (?, ?, ?)").bind(name).bind(memory).bind(instances).fetch_one(&mut *transaction).await.unwrap();
    transaction.commit().await.unwrap();

    Ok(org)
}

pub async fn update_org(pool: Pool<MySql>, id: &str, name: &str, memory: i32, instances: i32) -> anyhow::Result<Org> {
    let mut transaction = pool.begin().await.unwrap();
    let org = sqlx::query_as::<_, Org>("UPDATE orgs SET name = ?, memory = ?, instances = ? WHERE id = ?").bind(name).bind(memory).bind(instances).bind(id).fetch_one(&mut *transaction).await.unwrap();
    transaction.commit().await.unwrap();

    Ok(org)
}

pub async fn delete_org(pool: Pool<MySql>, id: &str) -> anyhow::Result<()> {
    let mut transaction = pool.begin().await.unwrap();
    sqlx::query("DELETE FROM orgs WHERE id = ?").bind(id).execute(&mut *transaction).await.unwrap();
    transaction.commit().await.unwrap();

    Ok(())
}

// Users

pub async fn list_users(pool: Pool<MySql>) -> anyhow::Result<Vec<User>> {
    let mut transaction = pool.begin().await.unwrap();
    let users = sqlx::query_as::<_, User>("SELECT * FROM users").fetch_all(&mut *transaction).await.unwrap();
    transaction.commit().await.unwrap();

    Ok(users)
}

pub async fn get_user_by_id(pool: Pool<MySql>, id: &str) -> anyhow::Result<User> {
    let mut transaction = pool.begin().await.unwrap();
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = ?").bind(id).fetch_one(&mut *transaction).await.unwrap();
    transaction.commit().await.unwrap();

    Ok(user)
}

pub async fn create_user(pool: Pool<MySql>, name: &str, memory: i32, instances: i32) -> anyhow::Result<User> {
    let mut transaction = pool.begin().await.unwrap();
    let user = sqlx::query_as::<_, User>("INSERT INTO users (name, memory, instances) VALUES (?, ?, ?)").bind(name).bind(memory).bind(instances).fetch_one(&mut *transaction).await.unwrap();
    transaction.commit().await.unwrap();

    Ok(user)
}

pub async fn update_user(pool: Pool<MySql>, id: &str, name: &str, memory: i32, instances: i32) -> anyhow::Result<User> {
    let mut transaction = pool.begin().await.unwrap();
    let user = sqlx::query_as::<_, User>("UPDATE users SET name = ?, memory = ?, instances = ? WHERE id = ?").bind(name).bind(memory).bind(instances).bind(id).fetch_one(&mut *transaction).await.unwrap();
    transaction.commit().await.unwrap();

    Ok(user)
}

pub async fn delete_user(pool: Pool<MySql>, id: &str) -> anyhow::Result<()> {
    let mut transaction = pool.begin().await.unwrap();
    sqlx::query("DELETE FROM users WHERE id = ?").bind(id).execute(&mut *transaction).await.unwrap();
    transaction.commit().await.unwrap();

    Ok(())
}

// Regions

pub async fn list_regions(pool: Pool<MySql>) -> anyhow::Result<Vec<App>> {
    let mut transaction = pool.begin().await.unwrap();
    let regions = sqlx::query_as::<_, App>("SELECT * FROM regions").fetch_all(&mut *transaction).await.unwrap();
    transaction.commit().await.unwrap();

    Ok(regions)
}

pub async fn get_region_by_id(pool: Pool<MySql>, id: &str) -> anyhow::Result<App> {
    let mut transaction = pool.begin().await.unwrap();
    let region = sqlx::query_as::<_, App>("SELECT * FROM regions WHERE id = ?").bind(id).fetch_one(&mut *transaction).await.unwrap();
    transaction.commit().await.unwrap();

    Ok(region)
}

pub async fn create_region(pool: Pool<MySql>, name: &str, memory: i32, instances: i32) -> anyhow::Result<App> {
    let mut transaction = pool.begin().await.unwrap();
    let region = sqlx::query_as::<_, App>("INSERT INTO regions (name, memory, instances) VALUES (?, ?, ?)").bind(name).bind(memory).bind(instances).fetch_one(&mut *transaction).await.unwrap();
    transaction.commit().await.unwrap();

    Ok(region)
}

pub async fn update_region(pool: Pool<MySql>, id: &str, name: &str, memory: i32, instances: i32) -> anyhow::Result<App> {
    let mut transaction = pool.begin().await.unwrap();
    let region = sqlx::query_as::<_, App>("UPDATE regions SET name = ?, memory = ?, instances = ? WHERE id = ?").bind(name).bind(memory).bind(instances).bind(id).fetch_one(&mut *transaction).await.unwrap();
    transaction.commit().await.unwrap();

    Ok(region)
}

pub async fn delete_region(pool: Pool<MySql>, id: &str) -> anyhow::Result<()> {
    let mut transaction = pool.begin().await.unwrap();
    sqlx::query("DELETE FROM regions WHERE id = ?").bind(id).execute(&mut *transaction).await.unwrap();
    transaction.commit().await.unwrap();

    Ok(())
}

// Builds

pub async fn list_builds(pool: Pool<MySql>) -> anyhow::Result<Vec<App>> {
    let mut transaction = pool.begin().await.unwrap();
    let builds = sqlx::query_as::<_, App>("SELECT * FROM builds").fetch_all(&mut *transaction).await.unwrap();
    transaction.commit().await.unwrap();

    Ok(builds)
}

pub async fn get_build_by_id(pool: Pool<MySql>, id: &str) -> anyhow::Result<App> {
    let mut transaction = pool.begin().await.unwrap();
    let build = sqlx::query_as::<_, App>("SELECT * FROM builds WHERE id = ?").bind(id).fetch_one(&mut *transaction).await.unwrap();
    transaction.commit().await.unwrap();

    Ok(build)
}

pub async fn create_build(pool: Pool<MySql>, name: &str, memory: i32, instances: i32) -> anyhow::Result<App> {
    let mut transaction = pool.begin().await.unwrap();
    let build = sqlx::query_as::<_, App>("INSERT INTO builds (name, memory, instances) VALUES (?, ?, ?)").bind(name).bind(memory).bind(instances).fetch_one(&mut *transaction).await.unwrap();
    transaction.commit().await.unwrap();

    Ok(build)
}

pub async fn update_build(pool: Pool<MySql>, id: &str, name: &str, memory: i32, instances: i32) -> anyhow::Result<App> {
    let mut transaction = pool.begin().await.unwrap();
    let build = sqlx::query_as::<_, App>("UPDATE builds SET name = ?, memory = ?, instances = ? WHERE id = ?").bind(name).bind(memory).bind(instances).bind(id).fetch_one(&mut *transaction).await.unwrap();
    transaction.commit().await.unwrap();

    Ok(build)
}

pub async fn delete_build(pool: Pool<MySql>, id: &str) -> anyhow::Result<()> {
    let mut transaction = pool.begin().await.unwrap();
    sqlx::query("DELETE FROM builds WHERE id = ?").bind(id).execute(&mut *transaction).await.unwrap();
    transaction.commit().await.unwrap();

    Ok(())
}

// Deployments

pub async fn list_deployments(pool: Pool<MySql>) -> anyhow::Result<Vec<App>> {
    let mut transaction = pool.begin().await.unwrap();
    let deployments = sqlx::query_as::<_, App>("SELECT * FROM deployments").fetch_all(&mut *transaction).await.unwrap();
    transaction.commit().await.unwrap();

    Ok(deployments)
}

pub async fn get_deployment_by_id(pool: Pool<MySql>, id: &str) -> anyhow::Result<App> {
    let mut transaction = pool.begin().await.unwrap();
    let deployment = sqlx::query_as::<_, App>("SELECT * FROM deployments WHERE id = ?").bind(id).fetch_one(&mut *transaction).await.unwrap();
    transaction.commit().await.unwrap();

    Ok(deployment)
}

pub async fn create_deployment(pool: Pool<MySql>, name: &str, memory: i32, instances: i32) -> anyhow::Result<App> {
    let mut transaction = pool.begin().await.unwrap();
    let deployment = sqlx::query_as::<_, App>("INSERT INTO deployments (name, memory, instances) VALUES (?, ?, ?)").bind(name).bind(memory).bind(instances).fetch_one(&mut *transaction).await.unwrap();
    transaction.commit().await.unwrap();

    Ok(deployment)
}

pub async fn update_deployment(pool: Pool<MySql>, id: &str, name: &str, memory: i32, instances: i32) -> anyhow::Result<App> {
    let mut transaction = pool.begin().await.unwrap();
    let deployment = sqlx::query_as::<_, App>("UPDATE deployments SET name = ?, memory = ?, instances = ? WHERE id = ?").bind(name).bind(memory).bind(instances).bind(id).fetch_one(&mut *transaction).await.unwrap();
    transaction.commit().await.unwrap();

    Ok(deployment)
}

pub async fn delete_deployment(pool: Pool<MySql>, id: &str) -> anyhow::Result<()> {
    let mut transaction = pool.begin().await.unwrap();
    sqlx::query("DELETE FROM deployments WHERE id = ?").bind(id).execute(&mut *transaction).await.unwrap();
    transaction.commit().await.unwrap();

    Ok(())
}

// Permissions

pub async fn list_permissions(pool: Pool<MySql>) -> anyhow::Result<Vec<Permission>> {
    let mut transaction = pool.begin().await.unwrap();
    let permissions = sqlx::query_as::<_, Permission>("SELECT * FROM permissions").fetch_all(&mut *transaction).await.unwrap();
    transaction.commit().await.unwrap();

    Ok(permissions)
}

pub async fn get_permission_by_id(pool: Pool<MySql>, id: &str) -> anyhow::Result<Permission> {
    let mut transaction = pool.begin().await.unwrap();
    let permission = sqlx::query_as::<_, Permission>("SELECT * FROM permissions WHERE id = ?").bind(id).fetch_one(&mut *transaction).await.unwrap();
    transaction.commit().await.unwrap();

    Ok(permission)
}

pub async fn create_permission(pool: Pool<MySql>, name: &str, memory: i32, instances: i32) -> anyhow::Result<Permission> {
    let mut transaction = pool.begin().await.unwrap();
    let permission = sqlx::query_as::<_, Permission>("INSERT INTO permissions (name, memory, instances) VALUES (?, ?, ?)").bind(name).bind(memory).bind(instances).fetch_one(&mut *transaction).await.unwrap();
    transaction.commit().await.unwrap();

    Ok(permission)
}

pub async fn update_permission(pool: Pool<MySql>, id: &str, name: &str, memory: i32, instances: i32) -> anyhow::Result<Permission> {
    let mut transaction = pool.begin().await.unwrap();
    let permission = sqlx::query_as::<_, Permission>("UPDATE permissions SET name = ?, memory = ?, instances = ? WHERE id = ?").bind(name).bind(memory).bind(instances).bind(id).fetch_one(&mut *transaction).await.unwrap();
    transaction.commit().await.unwrap();

    Ok(permission)
}

pub async fn delete_permission(pool: Pool<MySql>, id: &str) -> anyhow::Result<()> {
    let mut transaction = pool.begin().await.unwrap();
    sqlx::query("DELETE FROM permissions WHERE id = ?").bind(id).execute(&mut *transaction).await.unwrap();
    transaction.commit().await.unwrap();

    Ok(())
}

// Roles

pub async fn list_roles(pool: Pool<MySql>) -> anyhow::Result<Vec<Role>> {
    let mut transaction = pool.begin().await.unwrap();
    let roles = sqlx::query_as::<_, Role>("SELECT * FROM roles").fetch_all(&mut *transaction).await.unwrap();
    transaction.commit().await.unwrap();

    Ok(roles)
}

pub async fn get_role_by_id(pool: Pool<MySql>, id: &str) -> anyhow::Result<Role> {
    let mut transaction = pool.begin().await.unwrap();
    let role = sqlx::query_as::<_, Role>("SELECT * FROM roles WHERE id = ?").bind(id).fetch_one(&mut *transaction).await.unwrap();
    transaction.commit().await.unwrap();

    Ok(role)
}

pub async fn create_role(pool: Pool<MySql>, name: &str, memory: i32, instances: i32) -> anyhow::Result<Role> {
    let mut transaction = pool.begin().await.unwrap();
    let role = sqlx::query_as::<_, Role>("INSERT INTO roles (name, memory, instances) VALUES (?, ?, ?)").bind(name).bind(memory).bind(instances).fetch_one(&mut *transaction).await.unwrap();
    transaction.commit().await.unwrap();

    Ok(role)
}

pub async fn update_role(pool: Pool<MySql>, id: &str, name: &str, memory: i32, instances: i32) -> anyhow::Result<Role> {
    let mut transaction = pool.begin().await.unwrap();
    let role = sqlx::query_as::<_, Role>("UPDATE roles SET name = ?, memory = ?, instances = ? WHERE id = ?").bind(name).bind(memory).bind(instances).bind(id).fetch_one(&mut *transaction).await.unwrap();
    transaction.commit().await.unwrap();

    Ok(role)
}

pub async fn delete_role(pool: Pool<MySql>, id: &str) -> anyhow::Result<()> {
    let mut transaction = pool.begin().await.unwrap();
    sqlx::query("DELETE FROM roles WHERE id = ?").bind(id).execute(&mut *transaction).await.unwrap();
    transaction.commit().await.unwrap();

    Ok(())
}

// Role Permissions

pub async fn list_role_permissions(pool: Pool<MySql>) -> anyhow::Result<Vec<App>> {
    let mut transaction = pool.begin().await.unwrap();
    let role_permissions = sqlx::query_as::<_, App>("SELECT * FROM role_permissions").fetch_all(&mut *transaction).await.unwrap();
    transaction.commit().await.unwrap();

    Ok(role_permissions)
}

pub async fn get_role_permission_by_id(pool: Pool<MySql>, id: &str) -> anyhow::Result<App> {
    let mut transaction = pool.begin().await.unwrap();
    let role_permission = sqlx::query_as::<_, App>("SELECT * FROM role_permissions WHERE id = ?").bind(id).fetch_one(&mut *transaction).await.unwrap();
    transaction.commit().await.unwrap();

    Ok(role_permission)
}

pub async fn create_role_permission(pool: Pool<MySql>, name: &str, memory: i32, instances: i32) -> anyhow::Result<App> {
    let mut transaction = pool.begin().await.unwrap();
    let role_permission = sqlx::query_as::<_, App>("INSERT INTO role_permissions (name, memory, instances) VALUES (?, ?, ?)").bind(name).bind(memory).bind(instances).fetch_one(&mut *transaction).await.unwrap();
    transaction.commit().await.unwrap();

    Ok(role_permission)
}

pub async fn update_role_permission(pool: Pool<MySql>, id: &str, name: &str, memory: i32, instances: i32) -> anyhow::Result<App> {
    let mut transaction = pool.begin().await.unwrap();
    let role_permission = sqlx::query_as::<_, App>("UPDATE role_permissions SET name = ?, memory = ?, instances = ? WHERE id = ?").bind(name).bind(memory).bind(instances).bind(id).fetch_one(&mut *transaction).await.unwrap();
    transaction.commit().await.unwrap();

    Ok(role_permission)
}

pub async fn delete_role_permission(pool: Pool<MySql>, id: &str) -> anyhow::Result<()> {
    let mut transaction = pool.begin().await.unwrap();
    sqlx::query("DELETE FROM role_permissions WHERE id = ?").bind(id).execute(&mut *transaction).await.unwrap();
    transaction.commit().await.unwrap();

    Ok(())
}

// Role Users

pub async fn list_role_users(pool: Pool<MySql>) -> anyhow::Result<Vec<Role>> {
    let mut transaction = pool.begin().await.unwrap();
    let role_users = sqlx::query_as::<_, Role>("SELECT * FROM role_users").fetch_all(&mut *transaction).await.unwrap();
    transaction.commit().await.unwrap();

    Ok(role_users)
}

pub async fn get_role_user_by_id(pool: Pool<MySql>, id: &str) -> anyhow::Result<App> {
    let mut transaction = pool.begin().await.unwrap();
    let role_user = sqlx::query_as::<_, App>("SELECT * FROM role_users WHERE id = ?").bind(id).fetch_one(&mut *transaction).await.unwrap();
    transaction.commit().await.unwrap();

    Ok(role_user)
}

pub async fn create_role_user(pool: Pool<MySql>, name: &str, memory: i32, instances: i32) -> anyhow::Result<App> {
    let mut transaction = pool.begin().await.unwrap();
    let role_user = sqlx::query_as::<_, App>("INSERT INTO role_users (name, memory, instances) VALUES (?, ?, ?)").bind(name).bind(memory).bind(instances).fetch_one(&mut *transaction).await.unwrap();
    transaction.commit().await.unwrap();

    Ok(role_user)
}

pub async fn update_role_user(pool: Pool<MySql>, id: &str, name: &str, memory: i32, instances: i32) -> anyhow::Result<App> {
    let mut transaction = pool.begin().await.unwrap();
    let role_user = sqlx::query_as::<_, App>("UPDATE role_users SET name = ?, memory = ?, instances = ? WHERE id = ?").bind(name).bind(memory).bind(instances).bind(id).fetch_one(&mut *transaction).await.unwrap();
    transaction.commit().await.unwrap();

    Ok(role_user)
}

pub async fn delete_role_user(pool: Pool<MySql>, id: &str) -> anyhow::Result<()> {
    let mut transaction = pool.begin().await.unwrap();
    sqlx::query("DELETE FROM role_users WHERE id = ?").bind(id).execute(&mut *transaction).await.unwrap();
    transaction.commit().await.unwrap();

    Ok(())
}

// Org Members

pub async fn list_org_members(pool: Pool<MySql>) -> anyhow::Result<Vec<App>> {
    let mut transaction = pool.begin().await.unwrap();
    let org_members = sqlx::query_as::<_, App>("SELECT * FROM org_members").fetch_all(&mut *transaction).await.unwrap();
    transaction.commit().await.unwrap();

    Ok(org_members)
}

pub async fn get_org_member_by_id(pool: Pool<MySql>, id: &str) -> anyhow::Result<App> {
    let mut transaction = pool.begin().await.unwrap();
    let org_member = sqlx::query_as::<_, App>("SELECT * FROM org_members WHERE id = ?").bind(id).fetch_one(&mut *transaction).await.unwrap();
    transaction.commit().await.unwrap();

    Ok(org_member)
}

pub async fn create_org_member(pool: Pool<MySql>, name: &str, memory: i32, instances: i32) -> anyhow::Result<App> {
    let mut transaction = pool.begin().await.unwrap();
    let org_member = sqlx::query_as::<_, App>("INSERT INTO org_members (name, memory, instances) VALUES (?, ?, ?)").bind(name).bind(memory).bind(instances).fetch_one(&mut *transaction).await.unwrap();
    transaction.commit().await.unwrap();

    Ok(org_member)
}

pub async fn update_org_member(pool: Pool<MySql>, id: &str, name: &str, memory: i32, instances: i32) -> anyhow::Result<App> {
    let mut transaction = pool.begin().await.unwrap();
    let org_member = sqlx::query_as::<_, App>("UPDATE org_members SET name = ?, memory = ?, instances = ? WHERE id = ?").bind(name).bind(memory).bind(instances).bind(id).fetch_one(&mut *transaction).await.unwrap();
    transaction.commit().await.unwrap();

    Ok(org_member)
}

pub async fn delete_org_member(pool: Pool<MySql>, id: &str) -> anyhow::Result<()> {
    let mut transaction = pool.begin().await.unwrap();
    sqlx::query("DELETE FROM org_members WHERE id = ?").bind(id).execute(&mut *transaction).await.unwrap();
    transaction.commit().await.unwrap();

    Ok(())
}

// App Regions

pub async fn list_app_regions(pool: Pool<MySql>) -> anyhow::Result<Vec<Region>> {
    let mut transaction = pool.begin().await.unwrap();
    let regions = sqlx::query_as::<_, Region>("SELECT * FROM regions").fetch_all(&mut *transaction).await.unwrap();
    transaction.commit().await.unwrap();

    Ok(regions)
}

pub async fn get_app_region_by_id(pool: Pool<MySql>, id: &str) -> anyhow::Result<Region> {
    let mut transaction = pool.begin().await.unwrap();
    let region = sqlx::query_as::<_, Region>("SELECT * FROM regions WHERE id = ?").bind(id).fetch_one(&mut *transaction).await.unwrap();
    transaction.commit().await.unwrap();

    Ok(region)
}

pub async fn create_app_region(pool: Pool<MySql>, name: &str, memory: i32, instances: i32) -> anyhow::Result<Region> {
    let mut transaction = pool.begin().await.unwrap();
    let region = sqlx::query_as::<_, Region>("INSERT INTO regions (name, memory, instances) VALUES (?, ?, ?)").bind(name).bind(memory).bind(instances).fetch_one(&mut *transaction).await.unwrap();
    transaction.commit().await.unwrap();

    Ok(region)
}

pub async fn update_app_region(pool: Pool<MySql>, id: &str, name: &str, memory: i32, instances: i32) -> anyhow::Result<Region> {
    let mut transaction = pool.begin().await.unwrap();
    let region = sqlx::query_as::<_, Region>("UPDATE regions SET name = ?, memory = ?, instances = ? WHERE id = ?").bind(name).bind(memory).bind(instances).bind(id).fetch_one(&mut *transaction).await.unwrap();
    transaction.commit().await.unwrap();

    Ok(region)
}

pub async fn delete_app_region(pool: Pool<MySql>, id: &str) -> anyhow::Result<()> {
    let mut transaction = pool.begin().await.unwrap();
    sqlx::query("DELETE FROM regions WHERE id = ?").bind(id).execute(&mut *transaction).await.unwrap();
    transaction.commit().await.unwrap();

    Ok(())
}

// App Instances

pub async fn list_app_instances(pool: Pool<MySql>) -> anyhow::Result<Vec<App>> {
    let mut transaction = pool.begin().await.unwrap();
    let app_instances = sqlx::query_as::<_, App>("SELECT * FROM app_instances").fetch_all(&mut *transaction).await.unwrap();
    transaction.commit().await.unwrap();

    Ok(app_instances)
}

pub async fn get_app_instance_by_id(pool: Pool<MySql>, id: &str) -> anyhow::Result<App> {
    let mut transaction = pool.begin().await.unwrap();
    let app_instance = sqlx::query_as::<_, App>("SELECT * FROM app_instances WHERE id = ?").bind(id).fetch_one(&mut *transaction).await.unwrap();
    transaction.commit().await.unwrap();

    Ok(app_instance)
}

pub async fn create_app_instance(pool: Pool<MySql>, name: &str, memory: i32, instances: i32) -> anyhow::Result<App> {
    let mut transaction = pool.begin().await.unwrap();
    let app_instance = sqlx::query_as::<_, App>("INSERT INTO app_instances (name, memory, instances) VALUES (?, ?, ?)").bind(name).bind(memory).bind(instances).fetch_one(&mut *transaction).await.unwrap();
    transaction.commit().await.unwrap();

    Ok(app_instance)
}

pub async fn update_app_instance(pool: Pool<MySql>, id: &str, name: &str, memory: i32, instances: i32) -> anyhow::Result<App> {
    let mut transaction = pool.begin().await.unwrap();
    let app_instance = sqlx::query_as::<_, App>("UPDATE app_instances SET name = ?, memory = ?, instances = ? WHERE id = ?").bind(name).bind(memory).bind(instances).bind(id).fetch_one(&mut *transaction).await.unwrap();
    transaction.commit().await.unwrap();

    Ok(app_instance)
}

pub async fn delete_app_instance(pool: Pool<MySql>, id: &str) -> anyhow::Result<()> {
    let mut transaction = pool.begin().await.unwrap();
    sqlx::query("DELETE FROM app_instances WHERE id = ?").bind(id).execute(&mut *transaction).await.unwrap();
    transaction.commit().await.unwrap();

    Ok(())
}

// App Builds

pub async fn list_app_builds(pool: Pool<MySql>) -> anyhow::Result<Vec<App>> {
    let mut transaction = pool.begin().await.unwrap();
    let app_builds = sqlx::query_as::<_, App>("SELECT * FROM app_builds").fetch_all(&mut *transaction).await.unwrap();
    transaction.commit().await.unwrap();

    Ok(app_builds)
}

pub async fn get_app_build_by_id(pool: Pool<MySql>, id: &str) -> anyhow::Result<App> {
    let mut transaction = pool.begin().await.unwrap();
    let app_build = sqlx::query_as::<_, App>("SELECT * FROM app_builds WHERE id = ?").bind(id).fetch_one(&mut *transaction).await.unwrap();
    transaction.commit().await.unwrap();

    Ok(app_build)
}

pub async fn create_app_build(pool: Pool<MySql>, name: &str, memory: i32, instances: i32) -> anyhow::Result<App> {
    let mut transaction = pool.begin().await.unwrap();
    let app_build = sqlx::query_as::<_, App>("INSERT INTO app_builds (name, memory, instances) VALUES (?, ?, ?)").bind(name).bind(memory).bind(instances).fetch_one(&mut *transaction).await.unwrap();
    transaction.commit().await.unwrap();

    Ok(app_build)
}

pub async fn update_app_build(pool: Pool<MySql>, id: &str, name: &str, memory: i32, instances: i32) -> anyhow::Result<App> {
    let mut transaction = pool.begin().await.unwrap();
    let app_build = sqlx::query_as::<_, App>("UPDATE app_builds SET name = ?, memory = ?, instances = ? WHERE id = ?").bind(name).bind(memory).bind(instances).bind(id).fetch_one(&mut *transaction).await.unwrap();
    transaction.commit().await.unwrap();

    Ok(app_build)
}

pub async fn delete_app_build(pool: Pool<MySql>, id: &str) -> anyhow::Result<()> {
    let mut transaction = pool.begin().await.unwrap();
    sqlx::query("DELETE FROM app_builds WHERE id = ?").bind(id).execute(&mut *transaction).await.unwrap();
    transaction.commit().await.unwrap();

    Ok(())
}

// App Deployments

pub async fn list_app_deployments(pool: Pool<MySql>) -> anyhow::Result<Vec<App>> {
    let mut transaction = pool.begin().await.unwrap();
    let app_deployments = sqlx::query_as::<_, App>("SELECT * FROM app_deployments").fetch_all(&mut *transaction).await.unwrap();
    transaction.commit().await.unwrap();

    Ok(app_deployments)
}

pub async fn get_app_deployment_by_id(pool: Pool<MySql>, id: &str) -> anyhow::Result<App> {
    let mut transaction = pool.begin().await.unwrap();
    let app_deployment = sqlx::query_as::<_, App>("SELECT * FROM app_deployments WHERE id = ?").bind(id).fetch_one(&mut *transaction).await.unwrap();
    transaction.commit().await.unwrap();

    Ok(app_deployment)
}

pub async fn create_app_deployment(pool: Pool<MySql>, name: &str, memory: i32, instances: i32) -> anyhow::Result<App> {
    let mut transaction = pool.begin().await.unwrap();
    let app_deployment = sqlx::query_as::<_, App>("INSERT INTO app_deployments (name, memory, instances) VALUES (?, ?, ?)").bind(name).bind(memory).bind(instances).fetch_one(&mut *transaction).await.unwrap();
    transaction.commit().await.unwrap();

    Ok(app_deployment)
}

pub async fn update_app_deployment(pool: Pool<MySql>, id: &str, name: &str, memory: i32, instances: i32) -> anyhow::Result<App> {
    let mut transaction = pool.begin().await.unwrap();
    let app_deployment = sqlx::query_as::<_, App>("UPDATE app_deployments SET name = ?, memory = ?, instances = ? WHERE id = ?").bind(name).bind(memory).bind(instances).bind(id).fetch_one(&mut *transaction).await.unwrap();
    transaction.commit().await.unwrap();

    Ok(app_deployment)
}

pub async fn delete_app_deployment(pool: Pool<MySql>, id: &str) -> anyhow::Result<()> {
    let mut transaction = pool.begin().await.unwrap();
    sqlx::query("DELETE FROM app_deployments WHERE id = ?").bind(id).execute(&mut *transaction).await.unwrap();
    transaction.commit().await.unwrap();

    Ok(())
}

// App Domains

pub async fn list_app_domains(pool: Pool<MySql>) -> anyhow::Result<Vec<App>> {
    let mut transaction = pool.begin().await.unwrap();
    let app_domains = sqlx::query_as::<_, App>("SELECT * FROM app_domains").fetch_all(&mut *transaction).await.unwrap();
    transaction.commit().await.unwrap();

    Ok(app_domains)
}

pub async fn get_app_domain_by_id(pool: Pool<MySql>, id: &str) -> anyhow::Result<App> {
    let mut transaction = pool.begin().await.unwrap();
    let app_domain = sqlx::query_as::<_, App>("SELECT * FROM app_domains WHERE id = ?").bind(id).fetch_one(&mut *transaction).await.unwrap();
    transaction.commit().await.unwrap();

    Ok(app_domain)
}

pub async fn create_app_domain(pool: Pool<MySql>, name: &str, memory: i32, instances: i32) -> anyhow::Result<App> {
    let mut transaction = pool.begin().await.unwrap();
    let app_domain = sqlx::query_as::<_, App>("INSERT INTO app_domains (name, memory, instances) VALUES (?, ?, ?)").bind(name).bind(memory).bind(instances).fetch_one(&mut *transaction).await.unwrap();
    transaction.commit().await.unwrap();

    Ok(app_domain)
}

pub async fn update_app_domain(pool: Pool<MySql>, id: &str, name: &str, memory: i32, instances: i32) -> anyhow::Result<App> {
    let mut transaction = pool.begin().await.unwrap();
    let app_domain = sqlx::query_as::<_, App>("UPDATE app_domains SET name = ?, memory = ?, instances = ? WHERE id = ?").bind(name).bind(memory).bind(instances).bind(id).fetch_one(&mut *transaction).await.unwrap();
    transaction.commit().await.unwrap();

    Ok(app_domain)
}

pub async fn delete_app_domain(pool: Pool<MySql>, id: &str) -> anyhow::Result<()> {
    let mut transaction = pool.begin().await.unwrap();
    sqlx::query("DELETE FROM app_domains WHERE id = ?").bind(id).execute(&mut *transaction).await.unwrap();
    transaction.commit().await.unwrap();

    Ok(())
}