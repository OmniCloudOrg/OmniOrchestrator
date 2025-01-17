use mysql::prelude::FromRow;
use crate::db::Row;

pub struct Users {
    pub user_id: i64,
    pub username: String,
    pub active: bool,
    pub email: String,
    pub created_at: i64,
    pub updated_at: i64
}

impl FromRow for Users {
    fn from_row_opt(row: Row) -> std::result::Result<Self, mysql::FromRowError> {
        Ok(Users {
            user_id: row.get("user_id").unwrap(),
            username: row.get("username").unwrap(),
            email: row.get("email").unwrap(),
            created_at: row.get("created_at").unwrap(),
            updated_at: row.get("updated_at").unwrap(),
            active: row.get("active").unwrap()
        })
    }
    
}

pub struct Roles {
    pub role_id: i64,
    pub name: String,
    pub description: String,
    pub created_at: i64,
    pub updated_at: i64
}

impl FromRow for Roles {
    fn from_row_opt(row: Row) -> std::result::Result<Self, mysql::FromRowError> {
        Ok(Roles {
            role_id: row.get("role_id").unwrap(),
            name: row.get("name").unwrap(),
            description: row.get("description").unwrap(),
            created_at: row.get("created_at").unwrap(),
            updated_at: row.get("updated_at").unwrap()
        })
    }
}

pub struct Permissions {
    pub permission_id: i64,
    pub name: String,
    pub description: String,
    pub created_at: i64,
    pub updated_at: i64
}


impl FromRow for Permissions {
    fn from_row_opt(row: Row) -> std::result::Result<Self, mysql::FromRowError> {
        Ok(Permissions {
            permission_id: row.get("permission_id").unwrap(),
            name: row.get("name").unwrap(),
            description: row.get("description").unwrap(),
            created_at: row.get("created_at").unwrap(),
            updated_at: row.get("updated_at").unwrap()
        })
    }
    
}

pub struct Regions {
    pub region_id: i64,
    pub name: String,
    pub description: String,
    pub created_at: i64,
    pub updated_at: i64
}

impl FromRow for Regions {
    fn from_row_opt(row: Row) -> std::result::Result<Self, mysql::FromRowError> {
        Ok(Regions {
            region_id: row.get("region_id").unwrap(),
            name: row.get("name").unwrap(),
            description: row.get("description").unwrap(),
            created_at: row.get("created_at").unwrap(),
            updated_at: row.get("updated_at").unwrap()
        })
    }
}

pub struct Orgs {
    pub org_id: i64,
    pub name: String,
    pub created_at: i64,
    pub updated_at: i64
}

impl FromRow for Orgs {
    fn from_row_opt(row: Row) -> std::result::Result<Self, mysql::FromRowError> {
        Ok(Orgs {
            org_id: row.get("org_id").unwrap(),
            name: row.get("name").unwrap(),
            created_at: row.get("created_at").unwrap(),
            updated_at: row.get("updated_at").unwrap()
        })
    }
}

pub struct APIKeys {
    pub api_key_id: i64,
    pub org_id: i64,
    pub name: String,
    pub key_hash: String,
    pub created_at: i64,
    pub updated_at: i64
}

#[derive(Debug)]
pub struct App {
    pub app_id: i64,
    pub name: String,
    pub org_id: i64,
    pub git_repo: String,
    pub git_branch: String,
    pub container_image_url: String,
    pub region_id: i64,
    pub maintainance_mode: bool,
    pub created_at: i64,
    pub updated_at: i64
}

impl FromRow for App {
    fn from_row_opt(row: Row) -> std::result::Result<Self, mysql::FromRowError> {
        Ok(App {
            app_id: row.get("app_id").unwrap(),
            name: row.get("name").unwrap(),
            created_at: row.get("created_at").unwrap(),
            updated_at: row.get("updated_at").unwrap(),
            org_id: row.get("org_id").unwrap(),
            git_repo: row.get("git_repo").unwrap(),
            git_branch: row.get("git_branch").unwrap(),
            container_image_url: row.get("container_image_url").unwrap(),
            region_id: row.get("region_id").unwrap(),
            maintainance_mode: row.get("maintainance_mode").unwrap()
        })
    }
}

pub enum InstanceStatus {
    Running,
    Stopping,
    Stopped,
    Terminated,
    Failed
}

pub struct AppInstances {
    pub instance_id: i64,
    pub app_id: i64,
    pub instance_type: String,
    pub instance_name: String,
    pub container_id: String,
    pub node_name: String,
    pub instance_status: InstanceStatus,
    pub created_at: i64,
    pub updated_at: i64
}

impl FromRow for AppInstances {
    fn from_row_opt(row: Row) -> std::result::Result<Self, mysql::FromRowError> {
        Ok(AppInstances {
            instance_id: row.get("instance_id").unwrap(),
            app_id: row.get("app_id").unwrap(),
            instance_type: row.get("instance_type").unwrap(),
            instance_name: row.get("instance_name").unwrap(),
            container_id: row.get("container_id").unwrap(),
            node_name: row.get("node_name").unwrap(),
            instance_status: match row.get::<String, _>("instance_status").unwrap().as_str() {
                "running" => InstanceStatus::Running,
                "stopping" => InstanceStatus::Stopping,
                "stopped" => InstanceStatus::Stopped,
                "terminated" => InstanceStatus::Terminated,
                "failed" => InstanceStatus::Failed,
                _ => panic!("Invalid instance status in database, is the database corrupt?")
            },
            created_at: row.get("created_at").unwrap(),
            updated_at: row.get("updated_at").unwrap()
        })
    }
}

pub struct Domains {
    pub domain_id: i64,
    pub app_id: i64,
    pub name: String,
    pub ssl_enabled: bool,
    pub created_at: i64,
    pub updated_at: i64
}

impl FromRow for Domains {
    fn from_row_opt(row: Row) -> std::result::Result<Self, mysql::FromRowError> {
        Ok(Domains {
            domain_id: row.get("domain_id").unwrap(),
            app_id: row.get("app_id").unwrap(),
            name: row.get("name").unwrap(),
            ssl_enabled: row.get("ssl_enabled").unwrap(),
            created_at: row.get("created_at").unwrap(),
            updated_at: row.get("updated_at").unwrap()
        })
    }
}

pub enum BuildStatus {
    Pending,
    Building,
    Failed,
    Success
}

pub struct Builds {
    pub build_id: i64,
    pub app_id: i64,
    pub source_version: String,
    pub build_status: BuildStatus,
    pub started_at: i64,
    pub completed_at: i64,
    pub created_at: i64,
    pub updated_at: i64
}

impl FromRow for Builds {
    fn from_row_opt(row: Row) -> std::result::Result<Self, mysql::FromRowError> {
        Ok(Builds {
            build_id: row.get("build_id").unwrap(),
            app_id: row.get("app_id").unwrap(),
            source_version: row.get("source_version").unwrap(),
            build_status: match row.get::<String, _>("build_status").unwrap().as_str() {
                "pending" => BuildStatus::Pending,
                "building" => BuildStatus::Building,
                "failed" => BuildStatus::Failed,
                "success" => BuildStatus::Success,
                _ => panic!("Invalid build status in database, is the database corrupt?")
            },
            started_at: row.get("started_at").unwrap(),
            completed_at: row.get("completed_at").unwrap(),
            created_at: row.get("created_at").unwrap(),
            updated_at: row.get("updated_at").unwrap()
        })
    }
}

enum DeploymentStatus {
    Pending,
    InProgress,
    Deployed,
    Failed,
}

pub struct Deployments {
    pub deployment_id: i64,
    pub app_id: i64,
    pub build_id: i64,
    pub deployment_status: DeploymentStatus,
    pub instance_id: i64,
    pub created_at: i64,
    pub updated_at: i64
}

impl FromRow for Deployments {
    fn from_row_opt(row: Row) -> std::result::Result<Self, mysql::FromRowError> {
        Ok(Deployments {
            deployment_id: row.get("deployment_id").unwrap(),
            app_id: row.get("app_id").unwrap(),
            build_id: row.get("build_id").unwrap(),
            deployment_status: match row.get::<String, _>("deployment_status").unwrap().as_str() {
                "pending" => DeploymentStatus::Pending,
                "in_progress" => DeploymentStatus::InProgress,
                "deployed" => DeploymentStatus::Deployed,
                "failed" => DeploymentStatus::Failed,
                _ => panic!("Invalid deployment status in database, is the database corrupt?")
            },
            instance_id: row.get("instance_id").unwrap(),
            created_at: row.get("created_at").unwrap(),
            updated_at: row.get("updated_at").unwrap()
        })
    }
}

pub struct ConfigVars {
    pub config_var_id: i64,
    pub app_id: i64,
    pub key: String,
    pub value: String,
    pub created_at: i64,
    pub updated_at: i64
}

impl FromRow for ConfigVars {
    fn from_row_opt(row: Row) -> std::result::Result<Self, mysql::FromRowError> {
        Ok(ConfigVars {
            config_var_id: row.get("config_var_id").unwrap(),
            app_id: row.get("app_id").unwrap(),
            key: row.get("key").unwrap(),
            value: row.get("value").unwrap(),
            created_at: row.get("created_at").unwrap(),
            updated_at: row.get("updated_at").unwrap()
        })
    }
}

struct Metrics {
    id: i64,
    instance_id: i64,
    metric_name: String,
    metric_value: f64,
    timestamp: i64,
    
}

impl FromRow for Metrics {
    fn from_row_opt(row: Row) -> std::result::Result<Self, mysql::FromRowError> {
        Ok(Metrics {
            id: row.get("id").unwrap(),
            instance_id: row.get("instance_id").unwrap(),
            metric_name: row.get("metric_name").unwrap(),
            metric_value: row.get("metric_value").unwrap(),
            timestamp: row.get("current_timestamp").unwrap()
        })
    }
    
}

enum LogType {
    App,
    System,
    Deployment
}

pub struct InstanceLogs {
    pub log_id: i64,
    pub instance_id: i64,
    pub log_type: LogType,
    pub message: String,
    pub created_at: i64,
    pub updated_at: i64
}

impl FromRow for InstanceLogs {
    fn from_row_opt(row: Row) -> std::result::Result<Self, mysql::FromRowError> {
        Ok(InstanceLogs {
            log_id: row.get("log_id").unwrap(),
            instance_id: row.get("instance_id").unwrap(),
            log_type: match row.get::<String, _>("log_type").unwrap().as_str() {
                "app" => LogType::App,
                "system" => LogType::System,
                "deployment" => LogType::Deployment,
                _ => panic!("Invalid log type in database, is the database corrupt?")
            },
            message: row.get("message").unwrap(),
            created_at: row.get("created_at").unwrap(),
            updated_at: row.get("updated_at").unwrap()
        })
    }
}









