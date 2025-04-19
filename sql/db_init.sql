-- Drop all tables first (in correct dependency order)
DROP TABLE IF EXISTS backups, notifications, host_creds, metrics, allocations, instance_logs, audit_logs, api_keys, 
    config_vars, deployment_logs, rollbacks, deployments, builds, tasks, 
    autoscaling_rules, health_checks, network_policies, service_bindings,
    routes, instances, domains, apps, spaces, orgmember, permissions_role, 
    role_user, permissions, roles, quotas, orgs, user_sessions, user_pii, user_meta, users, 
    data_services, nodes, workers, regions, providers, providers_regions, user_notifications, role_notifications, notification_acknowledgments, alerts, alert_acknowledgments, alert_escalations, alert_history;

-- Create independent tables first (no foreign keys)

-- Users split BIGINTo three tables: users, user_meta, and user_pii
CREATE TABLE users (
    id BIGINT NOT NULL AUTO_INCREMENT,
    email VARCHAR(255) NOT NULL,
    email_verified TINYINT(1) DEFAULT 0,
    password VARCHAR(255) NOT NULL,
    salt VARCHAR(255) NOT NULL,
    password_changed_at DATETIME,
    login_attempts BIGINT DEFAULT 0,
    locked_until DATETIME,
    two_factor_enabled TINYINT(1) DEFAULT 0,
    two_factor_verified TINYINT(1) DEFAULT 0,
    active TINYINT(1) DEFAULT 1,
    status ENUM('active', 'deactivated', 'suspended', 'pending') DEFAULT 'pending',
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    deleted_at DATETIME,
    last_login_at DATETIME,
    PRIMARY KEY (id),
    UNIQUE KEY unique_email (email),
    INDEX idx_users_email_verified (email_verified),
    INDEX idx_users_active (active),
    INDEX idx_users_deleted_at (deleted_at),
    INDEX idx_users_status (status)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- User metadata - preferences and non-sensitive settings
CREATE TABLE user_meta (
    id BIGINT NOT NULL AUTO_INCREMENT,
    user_id BIGINT NOT NULL,
    timezone VARCHAR(50) DEFAULT 'UTC',
    language VARCHAR(10) DEFAULT 'en',
    theme VARCHAR(50) DEFAULT 'light',
    notification_preferences JSON,
    profile_image VARCHAR(255),
    dashboard_layout JSON,
    onboarding_completed TINYINT(1) DEFAULT 0,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    PRIMARY KEY (id),
    UNIQUE KEY unique_user_id (user_id),
    INDEX idx_user_meta_user_id (user_id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- Personally Identifiable Information (PII) - sensitive data
CREATE TABLE user_pii (
    id BIGINT NOT NULL AUTO_INCREMENT,
    user_id BIGINT NOT NULL,
    first_name VARCHAR(100),
    last_name VARCHAR(100),
    full_name VARCHAR(255),
    identity_verified TINYINT(1) DEFAULT 0,
    identity_verification_date DATETIME,
    identity_verification_method VARCHAR(100),
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    PRIMARY KEY (id),
    UNIQUE KEY unique_user_id (user_id),
    INDEX idx_user_pii_user_id (user_id),
    INDEX idx_user_pii_identity_verified (identity_verified)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- User sessions table for better session management
CREATE TABLE user_sessions (
    id BIGINT NOT NULL AUTO_INCREMENT,
    user_id BIGINT NOT NULL,
    session_token VARCHAR(255) NOT NULL,
    refresh_token VARCHAR(255),
    ip_address VARCHAR(45),
    user_agent TEXT,
    device_info JSON,
    location_info JSON,
    is_active TINYINT(1) DEFAULT 1,
    last_activity DATETIME,
    expires_at DATETIME NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (id),
    UNIQUE KEY unique_session_token (session_token),
    INDEX idx_user_sessions_user_id (user_id),
    INDEX idx_user_sessions_is_active (is_active),
    INDEX idx_user_sessions_expires_at (expires_at)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE roles (
    id BIGINT NOT NULL AUTO_INCREMENT,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    is_system_role TINYINT(1) DEFAULT 0,
    scope ENUM('global', 'organization', 'space', 'application') DEFAULT 'global',
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    PRIMARY KEY (id),
    UNIQUE KEY unique_name (name)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE allocations (
    id BIGINT NOT NULL AUTO_INCREMENT,
    name VARCHAR(100) NOT NULL,
    cpu DOUBLE NOT NULL,
    memory DOUBLE NOT NULL, -- in MB
    uplink DOUBLE NOT NULL, -- in Mbps
    downlink DOUBLE NOT NULL, -- in Mbps
    disk DOUBLE NOT NULL, -- in MB
    price_per_hour DECIMAL(10,4) DEFAULT 0,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    PRIMARY KEY (id),
    UNIQUE KEY unique_name (name)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE permissions (
    id BIGINT NOT NULL AUTO_INCREMENT,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    resource_type VARCHAR(255),
    action VARCHAR(50) NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    PRIMARY KEY (id),
    UNIQUE KEY unique_name_action_resource (name, action, resource_type)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE providers (
    id BIGINT NOT NULL AUTO_INCREMENT,
    name VARCHAR(255) NOT NULL,
    display_name VARCHAR(255) NOT NULL,
    provider_type ENUM('cloud', 'on-prem') DEFAULT 'cloud',
    status ENUM('active', 'maintenance', 'offline', 'deprecated') DEFAULT 'active',
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    PRIMARY KEY (id),
    UNIQUE KEY unique_name (name),
    INDEX idx_providers_status (status)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE regions (
    id BIGINT NOT NULL AUTO_INCREMENT,
    name VARCHAR(255) NOT NULL,
    display_name VARCHAR(255) NOT NULL,
    provider BIGINT NOT NULL,
    location VARCHAR(255),
    coordinates POINT,
    is_public TINYINT(1) DEFAULT 1,
    class VARCHAR(50) DEFAULT 'primary',
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    PRIMARY KEY (id),
    ForEIGN KEY (provider) REFERENCES providers(id) ON DELETE CASCADE,
    UNIQUE KEY unique_name (name),
    -- Spatial index removed
    INDEX idx_regions_provider (provider),
    INDEX idx_regions_class (class)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE providers_regions (
    id BIGINT NOT NULL AUTO_INCREMENT,
    provider_id BIGINT NOT NULL,
    region_id BIGINT NOT NULL,
    status ENUM('active', 'maintenance', 'offline', 'deprecated') DEFAULT 'active',
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    PRIMARY KEY (id),
    UNIQUE KEY unique_provider_region (provider_id, region_id),
    FOREIGN KEY (provider_id) REFERENCES providers(id) ON DELETE CASCADE,
    FOREIGN KEY (region_id) REFERENCES regions(id) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE orgs (
    id BIGINT NOT NULL AUTO_INCREMENT,
    name VARCHAR(255) NOT NULL,
    display_name VARCHAR(255),
    description TEXT,
    logo_url VARCHAR(255),
    website VARCHAR(255),
    billing_email VARCHAR(255),
    plan ENUM('free', 'starter', 'professional', 'enterprise') DEFAULT 'free',
    status ENUM('active', 'suspended', 'pending') DEFAULT 'active',
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    deleted_at DATETIME,
    PRIMARY KEY (id),
    UNIQUE KEY unique_name (name),
    INDEX idx_orgs_status (status),
    INDEX idx_orgs_plan (plan),
    INDEX idx_orgs_deleted_at (deleted_at)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- Create tables with single foreign key dependencies
CREATE TABLE permissions_role (
    permissions_id BIGINT NOT NULL,
    role_id BIGINT NOT NULL,
    PRIMARY KEY (permissions_id, role_id),
    FOREIGN KEY (permissions_id) REFERENCES permissions(id) ON DELETE CASCADE,
    FOREIGN KEY (role_id) REFERENCES roles(id) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE role_user (
    user_id BIGINT NOT NULL,
    role_id BIGINT NOT NULL,
    scope_type ENUM('global', 'organization', 'space', 'application') DEFAULT 'global',
    scope_id BIGINT NOT NULL DEFAULT 0,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    PRIMARY KEY (user_id, role_id, scope_type, scope_id), -- Simple composite key
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (role_id) REFERENCES roles(id) ON DELETE CASCADE,
    INDEX idx_role_user_scope (scope_type, scope_id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE orgmember (
    id BIGINT NOT NULL AUTO_INCREMENT,
    org_id BIGINT NOT NULL,
    user_id BIGINT NOT NULL,
    role ENUM('owner', 'admin', 'billing', 'member', 'guest') DEFAULT 'member',
    invitation_status ENUM('pending', 'accepted', 'rejected') DEFAULT 'accepted',
    invitation_token VARCHAR(255),
    invitation_expires_at DATETIME,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    PRIMARY KEY (id),
    UNIQUE KEY unique_org_user (org_id, user_id),
    KEY idx_orgmember_org_id (org_id),
    KEY idx_orgmember_user_id (user_id),
    KEY idx_orgmember_invitation_status (invitation_status),
    FOREIGN KEY (org_id) REFERENCES orgs(id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE api_keys (
    id BIGINT NOT NULL AUTO_INCREMENT,
    org_id BIGINT NOT NULL,
    user_id BIGINT NOT NULL,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    key_hash VARCHAR(255) NOT NULL,
    prefix VARCHAR(10) NOT NULL,
    scopes JSON, -- Specific permissions granted to this key
    expires_at DATETIME,
    last_used_at DATETIME,
    revoked TINYINT(1) DEFAULT 0,
    revoked_at DATETIME,
    revoked_by BIGINT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    PRIMARY KEY (id),
    UNIQUE KEY unique_prefix (prefix),
    KEY idx_api_keys_org_id (org_id),
    KEY idx_api_keys_user_id (user_id),
    KEY idx_api_keys_revoked (revoked),
    KEY idx_api_keys_expires_at (expires_at),
    FOREIGN KEY (org_id) REFERENCES orgs(id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (revoked_by) REFERENCES users(id) ON DELETE SET NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE quotas (
    id BIGINT NOT NULL AUTO_INCREMENT,
    name VARCHAR(255) NOT NULL,
    org_id BIGINT NOT NULL,
    memory_limit BIGINT NOT NULL COMMENT 'in MB',
    cpu_limit DOUBLE,
    disk_limit BIGINT COMMENT 'in MB',
    instance_limit BIGINT NOT NULL,
    routes_limit BIGINT NOT NULL,
    services_limit BIGINT NOT NULL,
    buildpack_limit BIGINT,
    domains_limit BIGINT,
    allow_paid_services TINYINT(1) DEFAULT 1,
    concurrent_builds_limit BIGINT DEFAULT 5,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    PRIMARY KEY (id),
    UNIQUE KEY unique_name (name),
    KEY idx_quotas_org_id (org_id),
    FOREIGN KEY (org_id) REFERENCES orgs(id) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE spaces (
    id BIGINT NOT NULL AUTO_INCREMENT,
    org_id BIGINT NOT NULL,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    status ENUM('active', 'suspended', 'archived') DEFAULT 'active',
    isolation_segment VARCHAR(255),
    network_isolation TINYINT(1) DEFAULT 0,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    deleted_at DATETIME,
    PRIMARY KEY (id),
    UNIQUE KEY unique_name_org (name, org_id),
    KEY idx_spaces_org_id (org_id),
    KEY idx_spaces_status (status),
    KEY idx_spaces_deleted_at (deleted_at),
    FOREIGN KEY (org_id) REFERENCES orgs(id) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE workers (
    id BIGINT NOT NULL AUTO_INCREMENT,
    region_id BIGINT NOT NULL,
    name VARCHAR(255) NOT NULL,
    provider_id VARCHAR(255),
    instance_type VARCHAR(255),
    status ENUM('active', 'provisioning', 'maintenance', 'powered_off', 'unreachable', 'degraded', 'decommissioning') DEFAULT 'active',
    cpu_total DOUBLE NOT NULL,
    cpu_available DOUBLE NOT NULL,
    cpu_reserved DOUBLE DEFAULT 0,
    memory_total DOUBLE NOT NULL COMMENT 'in MB',
    memory_available DOUBLE NOT NULL COMMENT 'in MB',
    memory_reserved DOUBLE DEFAULT 0 COMMENT 'in MB',
    disk_total DOUBLE NOT NULL COMMENT 'in MB',
    disk_available DOUBLE NOT NULL COMMENT 'in MB',
    disk_reserved DOUBLE DEFAULT 0 COMMENT 'in MB',
    network_in_capacity DOUBLE COMMENT 'in Mbps',
    network_out_capacity DOUBLE COMMENT 'in Mbps',
    docker_version VARCHAR(50),
    ssh_address VARCHAR(255),
    ssh_port BIGINT DEFAULT 22,
    ssh_user VARCHAR(50),
    ssh_key VARCHAR(255),
    labels JSON,
    taints JSON,
    annotations JSON,
    last_heartbeat DATETIME,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    PRIMARY KEY (id),
    UNIQUE KEY unique_name_region (name, region_id),
    KEY idx_workers_region_id (region_id),
    KEY idx_workers_status (status),
    KEY idx_workers_instance_type (instance_type),
    FOREIGN KEY (region_id) REFERENCES regions(id) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE data_services (
    id BIGINT NOT NULL AUTO_INCREMENT,
    region_id BIGINT NOT NULL,
    name VARCHAR(255) NOT NULL,
    display_name VARCHAR(255),
    service_type ENUM('database', 'cache', 'message_queue', 'network_filesystem', 'cdn', 'search', 'ai', 'analytics') NOT NULL,
    service_subtype VARCHAR(100) COMMENT 'e.g. mysql, redis, rabbitmq',
    status ENUM('active', 'provisioning', 'maintenance', 'powered_off', 'unreachable', 'degraded', 'decommissioning') DEFAULT 'active',
    version VARCHAR(50),
    plan VARCHAR(50) NOT NULL,
    tier ENUM('free', 'basic', 'standard', 'premium', 'enterprise') DEFAULT 'standard',
    is_highly_available TINYINT(1) DEFAULT 0,
    backup_enabled TINYINT(1) DEFAULT 1,
    backup_retention_days BIGINT DEFAULT 7,
    encryption_at_rest TINYINT(1) DEFAULT 1,
    connection_string VARCHAR(255),
    endpoint VARCHAR(255),
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    deleted_at DATETIME,
    PRIMARY KEY (id),
    UNIQUE KEY unique_name_region (name, region_id),
    KEY idx_data_services_region_id (region_id),
    KEY idx_data_services_status (status),
    KEY idx_data_services_type (service_type),
    KEY idx_data_services_subtype (service_subtype),
    KEY idx_data_services_deleted_at (deleted_at),
    FOREIGN KEY (region_id) REFERENCES regions(id) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- Create application-related tables
CREATE TABLE apps (
    id BIGINT NOT NULL AUTO_INCREMENT,
    name VARCHAR(255) NOT NULL,
    display_name VARCHAR(255),
    org_id BIGINT NOT NULL,
    space_id BIGINT,
    git_repo VARCHAR(255),
    git_branch VARCHAR(255) DEFAULT 'main',
    git_credentials_id BIGINT,
    container_image_url VARCHAR(255),
    container_registry_credentials_id BIGINT,
    buildpack_url VARCHAR(255),
    default_allocation_id BIGINT,
    region_id BIGINT,
    instances BIGINT DEFAULT 1,
    health_check_type ENUM('http', 'port', 'process', 'tcp', 'custom') DEFAULT 'port',
    health_check_endpoint VARCHAR(255),
    health_check_interval BIGINT DEFAULT 30,
    health_check_timeout BIGINT DEFAULT 30,
    health_check_retries BIGINT DEFAULT 3,
    runtime VARCHAR(255),
    restart_policy ENUM('always', 'on-failure', 'no') DEFAULT 'always',
    maintenance_mode TINYINT(1) DEFAULT 0,
    auto_scaling_enabled TINYINT(1) DEFAULT 0,
    status ENUM('started', 'stopped', 'crashed', 'starting', 'stopping', 'staged') DEFAULT 'stopped',
    deployment_strategy ENUM('rolling', 'blue-green', 'canary', 'recreate') DEFAULT 'rolling',
    canary_percentage BIGINT DEFAULT 20,
    idle_timeout BIGINT DEFAULT 300,
    max_concurrent_builds BIGINT DEFAULT 1,
    labels JSON,
    annotations JSON,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    deleted_at DATETIME,
    PRIMARY KEY (id),
    UNIQUE KEY unique_name_org (name, org_id),
    KEY idx_apps_name (name),
    KEY idx_apps_org_id (org_id),
    KEY idx_apps_space_id (space_id),
    KEY idx_apps_region_id (region_id),
    KEY idx_apps_status (status),
    KEY idx_apps_deleted_at (deleted_at),
    FOREIGN KEY (org_id) REFERENCES orgs(id) ON DELETE CASCADE,
    FOREIGN KEY (space_id) REFERENCES spaces(id),
    FOREIGN KEY (default_allocation_id) REFERENCES allocations(id),
    FOREIGN KEY (region_id) REFERENCES regions(id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE instances (
    id BIGINT NOT NULL AUTO_INCREMENT,
    app_id BIGINT NOT NULL,
    instance_type VARCHAR(255) NOT NULL,
    guid VARCHAR(36) NOT NULL,
    status ENUM('running', 'starting', 'stopping', 'stopped', 'crashed', 'terminated', 'unknown') DEFAULT 'starting',
    container_id VARCHAR(255),
    container_ip VARCHAR(45),
    allocation_id BIGINT,
    node_id BIGINT,
    instance_index BIGINT NOT NULL,
    last_health_check DATETIME,
    health_status ENUM('healthy', 'unhealthy', 'unknown') DEFAULT 'unknown',
    cpu_usage DOUBLE,
    memory_usage DOUBLE,
    disk_usage DOUBLE,
    uptime BIGINT DEFAULT 0,
    restart_count BIGINT DEFAULT 0,
    last_restart_reason TEXT,
    start_time DATETIME,
    stop_time DATETIME,
    exit_code BIGINT,
    exit_reason TEXT,
    scheduler_metadata JSON,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    PRIMARY KEY (id),
    UNIQUE KEY unique_app_index (app_id, instance_index),
    UNIQUE KEY unique_guid (guid),
    KEY idx_instances_app_id (app_id),
    KEY idx_instances_status (status),
    KEY idx_instances_health_status (health_status),
    KEY idx_instances_node_id (node_id),
    FOREIGN KEY (app_id) REFERENCES apps(id) ON DELETE CASCADE,
    FOREIGN KEY (allocation_id) REFERENCES allocations(id),
    FOREIGN KEY (node_id) REFERENCES workers(id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE domains (
    id BIGINT NOT NULL AUTO_INCREMENT,
    org_id BIGINT,
    name VARCHAR(255) NOT NULL,
    domain_type ENUM('system', 'private', 'shared') DEFAULT 'private',
    ssl_enabled TINYINT(1) DEFAULT 0,
    ssl_cert_path VARCHAR(255),
    ssl_key_path VARCHAR(255),
    ssl_cert_data TEXT,
    ssl_issuer VARCHAR(255),
    ssl_expiry_date DATE,
    auto_renew TINYINT(1) DEFAULT 1,
    dns_validation_record VARCHAR(255),
    verified TINYINT(1) DEFAULT 0,
    verification_method ENUM('dns', 'http', 'email', 'manual') DEFAULT 'dns',
    verification_status ENUM('pending', 'verified', 'failed') DEFAULT 'pending',
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    PRIMARY KEY (id),
    UNIQUE KEY unique_name (name),
    KEY idx_domains_ssl_expiry (ssl_expiry_date),
    KEY idx_domains_org_id (org_id),
    KEY idx_domains_domain_type (domain_type),
    KEY idx_domains_verified (verified),
    FOREIGN KEY (org_id) REFERENCES orgs(id) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE routes (
    id BIGINT NOT NULL AUTO_INCREMENT,
    domain_id BIGINT NOT NULL,
    host VARCHAR(255) DEFAULT '',
    path VARCHAR(255) DEFAULT '',
    app_id BIGINT,
    port BIGINT,
    weight BIGINT DEFAULT 100,
    https_only TINYINT(1) DEFAULT 0,
    BIGINTernal TINYINT(1) DEFAULT 0,
    status ENUM('active', 'reserved', 'suspended') DEFAULT 'active',
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    PRIMARY KEY (id),
    UNIQUE KEY unique_domain_host_path (domain_id, host, path),
    KEY idx_routes_domain_id (domain_id),
    KEY idx_routes_app_id (app_id),
    KEY idx_routes_status (status),
    FOREIGN KEY (domain_id) REFERENCES domains(id) ON DELETE CASCADE,
    FOREIGN KEY (app_id) REFERENCES apps(id) ON DELETE SET NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE service_bindings (
    id BIGINT NOT NULL AUTO_INCREMENT,
    app_id BIGINT NOT NULL,
    service_id BIGINT NOT NULL,
    credentials TEXT,
    credentials_encryption_key_id VARCHAR(255),
    binding_name VARCHAR(255),
    mount_path VARCHAR(255),
    environment_injection TINYINT(1) DEFAULT 1,
    status ENUM('creating', 'created', 'deleting', 'deleted', 'failed') DEFAULT 'creating',
    last_operation VARCHAR(255),
    error_message TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    PRIMARY KEY (id),
    UNIQUE KEY unique_app_service (app_id, service_id),
    KEY idx_service_bindings_app_id (app_id),
    KEY idx_service_bindings_service_id (service_id),
    KEY idx_service_bindings_status (status),
    FOREIGN KEY (app_id) REFERENCES apps(id) ON DELETE CASCADE,
    FOREIGN KEY (service_id) REFERENCES data_services(id) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE builds (
    id BIGINT NOT NULL AUTO_INCREMENT,
    app_id BIGINT NOT NULL,
    source_version VARCHAR(255),
    commit_sha VARCHAR(255),
    commit_message TEXT,
    author VARCHAR(255),
    status ENUM('pending', 'building', 'succeeded', 'failed', 'canceled') DEFAULT 'pending',
    build_pack_used VARCHAR(255),
    build_pack_url VARCHAR(255),
    build_pack_version VARCHAR(50),
    build_image VARCHAR(255),
    build_arguments JSON,
    build_environment JSON,
    build_cache_key VARCHAR(255),
    log_url VARCHAR(255),
    artifact_url VARCHAR(255),
    artifact_checksum VARCHAR(255),
    artifact_size BIGINT,
    error_message TEXT,
    started_at DATETIME,
    completed_at DATETIME,
    build_duration BIGINT COMMENT 'in seconds',
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (id),
    KEY idx_builds_app_id (app_id),
    KEY idx_builds_status (status),
    KEY idx_builds_created_at (created_at),
    KEY idx_builds_source_version (source_version),
    KEY idx_builds_commit_sha (commit_sha),
    FOREIGN KEY (app_id) REFERENCES apps(id) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE deployments (
    id BIGINT NOT NULL AUTO_INCREMENT,
    app_id BIGINT NOT NULL,
    build_id BIGINT NOT NULL,
    version VARCHAR(50),
    status ENUM('pending', 'in_progress', 'deployed', 'failed', 'canceled') DEFAULT 'pending',
    deployment_strategy ENUM('rolling', 'blue-green', 'canary', 'recreate') DEFAULT 'rolling',
    previous_deployment_id BIGINT,
    canary_percentage BIGINT DEFAULT 20,
    staged_instances BIGINT DEFAULT 0,
    total_instances BIGINT DEFAULT 0,
    environment_variables JSON,
    annotations JSON,
    labels JSON,
    started_at DATETIME,
    completed_at DATETIME,
    deployment_duration BIGINT COMMENT 'in seconds',
    error_message TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    created_by BIGINT,
    PRIMARY KEY (id),
    KEY idx_deployments_app_id (app_id),
    KEY idx_deployments_build_id (build_id),
    KEY idx_deployments_status (status),
    KEY idx_deployments_created_at (created_at),
    KEY idx_deployments_version (version),
    KEY idx_deployments_created_by (created_by),
    FOREIGN KEY (app_id) REFERENCES apps(id) ON DELETE CASCADE,
    FOREIGN KEY (build_id) REFERENCES builds(id),
    FOREIGN KEY (previous_deployment_id) REFERENCES deployments(id),
    FOREIGN KEY (created_by) REFERENCES users(id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE rollbacks (
    id BIGINT NOT NULL AUTO_INCREMENT,
    app_id BIGINT NOT NULL,
    from_deployment_id BIGINT NOT NULL,
    to_deployment_id BIGINT NOT NULL,
    status ENUM('pending', 'in_progress', 'completed', 'failed') DEFAULT 'pending',
    reason TEXT,
    automatic TINYINT(1) DEFAULT 0,
    trigger_condition VARCHAR(255),
    started_at DATETIME,
    completed_at DATETIME,
    rollback_duration BIGINT COMMENT 'in seconds',
    error_message TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    created_by BIGINT,
    PRIMARY KEY (id),
    KEY idx_rollbacks_app_id (app_id),
    KEY idx_rollbacks_status (status),
    KEY idx_rollbacks_created_at (created_at),
    KEY idx_rollbacks_created_by (created_by),
    KEY idx_rollbacks_automatic (automatic),
    FOREIGN KEY (app_id) REFERENCES apps(id) ON DELETE CASCADE,
    FOREIGN KEY (from_deployment_id) REFERENCES deployments(id),
    FOREIGN KEY (to_deployment_id) REFERENCES deployments(id),
    FOREIGN KEY (created_by) REFERENCES users(id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE deployment_logs (
    id BIGINT NOT NULL AUTO_INCREMENT,
    deployment_id BIGINT NOT NULL,
    log_type ENUM('app', 'system', 'deployment', 'build') NOT NULL,
    log_level ENUM('debug', 'info', 'warn', 'error', 'fatal') DEFAULT 'info',
    message TEXT NOT NULL,
    metadata JSON,
    timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (id),
    KEY idx_deployment_logs_deployment_id (deployment_id),
    KEY idx_deployment_logs_timestamp (timestamp),
    KEY idx_deployment_logs_log_type (log_type),
    KEY idx_deployment_logs_log_level (log_level),
    FOREIGN KEY (deployment_id) REFERENCES deployments(id) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE config_vars (
    id BIGINT NOT NULL AUTO_INCREMENT,
    app_id BIGINT NOT NULL,
    `key` VARCHAR(255) NOT NULL,
    value TEXT,
    is_secret TINYINT(1) DEFAULT 0,
    encryption_key_id VARCHAR(255),
    description TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    created_by BIGINT,
    updated_by BIGINT,
    PRIMARY KEY (id),
    UNIQUE KEY unique_app_key (app_id, `key`),
    KEY idx_config_vars_app_id (app_id),
    KEY idx_config_vars_is_secret (is_secret),
    FOREIGN KEY (app_id) REFERENCES apps(id) ON DELETE CASCADE,
    FOREIGN KEY (created_by) REFERENCES users(id),
    FOREIGN KEY (updated_by) REFERENCES users(id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE tasks (
    id BIGINT NOT NULL AUTO_INCREMENT,
    app_id BIGINT NOT NULL,
    command TEXT NOT NULL,
    name VARCHAR(255),
    status ENUM('pending', 'running', 'succeeded', 'failed', 'canceled') DEFAULT 'pending',
    memory_in_mb BIGINT,
    disk_in_mb BIGINT,
    cpu DOUBLE,
    timeout_seconds BIGINT DEFAULT 3600,
    result TEXT,
    exit_code BIGINT,
    sequence_id BIGINT,
    node_id BIGINT,
    started_at DATETIME,
    completed_at DATETIME,
    duration BIGINT COMMENT 'in seconds',
    error_message TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    created_by BIGINT,
    PRIMARY KEY (id),
    KEY idx_tasks_app_id (app_id),
    KEY idx_tasks_status (status),
    KEY idx_tasks_created_at (created_at),
    KEY idx_tasks_created_by (created_by),
    KEY idx_tasks_node_id (node_id),
    FOREIGN KEY (app_id) REFERENCES apps(id) ON DELETE CASCADE,
    FOREIGN KEY (node_id) REFERENCES workers(id),
    FOREIGN KEY (created_by) REFERENCES users(id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE health_checks (
    id BIGINT NOT NULL AUTO_INCREMENT,
    app_id BIGINT NOT NULL,
    type ENUM('http', 'tcp', 'port', 'process', 'custom') NOT NULL DEFAULT 'port',
    endpoint VARCHAR(255),
    timeout BIGINT NOT NULL DEFAULT 60,
    check_interval BIGINT NOT NULL DEFAULT 10,
    healthy_threshold BIGINT NOT NULL DEFAULT 3,
    unhealthy_threshold BIGINT NOT NULL DEFAULT 3,
    port BIGINT,
    protocol ENUM('http', 'https', 'tcp') DEFAULT 'http',
    http_status_codes VARCHAR(255) DEFAULT '200-399',
    response_body_regex VARCHAR(255),
    follow_redirects TINYINT(1) DEFAULT 1,
    initial_delay_seconds BIGINT DEFAULT 0,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    PRIMARY KEY (id),
    KEY idx_health_checks_app_id (app_id),
    FOREIGN KEY (app_id) REFERENCES apps(id) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE autoscaling_rules (
    id BIGINT NOT NULL AUTO_INCREMENT,
    app_id BIGINT NOT NULL,
    name VARCHAR(255),
    min_instances BIGINT NOT NULL DEFAULT 1,
    max_instances BIGINT NOT NULL DEFAULT 1,
    target_instances BIGINT,
    metric_type ENUM('cpu', 'memory', 'http_throughput', 'http_latency', 'queue_depth', 'custom') NOT NULL,
    custom_metric_name VARCHAR(255),
    custom_metric_query TEXT,
    threshold_value DOUBLE NOT NULL,
    threshold_unit VARCHAR(50),
    comparison_operator ENUM('GreaterThanOrEqualToThreshold', 'GreaterThanThreshold', 'LessThanThreshold', 'LessThanOrEqualToThreshold') DEFAULT 'GreaterThanOrEqualToThreshold',
    evaluation_periods BIGINT DEFAULT 1,
    period_seconds BIGINT DEFAULT 60,
    scaling_adjustment BIGINT DEFAULT 1,
    cooldown_period_seconds BIGINT NOT NULL DEFAULT 300,
    enabled TINYINT(1) DEFAULT 1,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    created_by BIGINT,
    PRIMARY KEY (id),
    KEY idx_autoscaling_rules_app_id (app_id),
    KEY idx_autoscaling_rules_metric_type (metric_type),
    KEY idx_autoscaling_rules_enabled (enabled),
    KEY idx_autoscaling_rules_created_by (created_by),
    FOREIGN KEY (app_id) REFERENCES apps(id) ON DELETE CASCADE,
    FOREIGN KEY (created_by) REFERENCES users(id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE network_policies (
    id BIGINT NOT NULL AUTO_INCREMENT,
    source_app_id BIGINT NOT NULL,
    destination_app_id BIGINT NOT NULL,
    protocol ENUM('tcp', 'udp', 'icmp', 'all') NOT NULL DEFAULT 'tcp',
    port_range_start BIGINT,
    port_range_end BIGINT,
    description TEXT,
    enabled TINYINT(1) DEFAULT 1,
    priority BIGINT DEFAULT 1000,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    created_by BIGINT,
    PRIMARY KEY (id),
    UNIQUE KEY unique_policy (source_app_id, destination_app_id, protocol, port_range_start, port_range_end),
    KEY idx_network_policies_source (source_app_id),
    KEY idx_network_policies_destination (destination_app_id),
    KEY idx_network_policies_protocol (protocol),
    KEY idx_network_policies_enabled (enabled),
    KEY idx_network_policies_created_by (created_by),
    FOREIGN KEY (source_app_id) REFERENCES apps(id) ON DELETE CASCADE,
    FOREIGN KEY (destination_app_id) REFERENCES apps(id) ON DELETE CASCADE,
    FOREIGN KEY (created_by) REFERENCES users(id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- Host credentials table for unified management of physical and virtual infrastructure
CREATE TABLE host_creds (
    -- Primary key and basic identification
    host_id CHAR(36) PRIMARY KEY,
    host_name VARCHAR(255) NOT NULL,
    host_address VARCHAR(255) NOT NULL,
    
    -- Physical vs. Cloud classification
    is_physical_node BOOLEAN NOT NULL,
    provider_type VARCHAR(255),  -- 'aws', 'azure', 'gcp', 'on-prem', etc.
    region_id BIGINT,           -- Reference to region this node exists in
    
    -- Authentication credentials - stored encrypted
    -- For physical nodes: SSH credentials
    -- For cloud providers: API credentials
    auth_type VARCHAR(50) NOT NULL,  -- 'ssh-key', 'ssh-password', 'api-key', 'iam-role', etc.
    username VARCHAR(255),
    password_encrypted TEXT,
    key_encrypted TEXT,
    key_id VARCHAR(255),        -- For referring to externally stored keys
    secret_encrypted TEXT,      -- For API secret keys
    
    -- Connection settings
    port INTEGER,
    ssh_key_path VARCHAR(512),  -- Optional: path to SSH key file
    connection_timeout INTEGER DEFAULT 30,
    
    -- Resource capacity (-1 for unlimited [recommended for non-physical hosts])
    cpu_cores INTEGER,
    memory_gb INTEGER,
    storage_gb INTEGER,
    
    -- Metadata
    tags JSON,                  -- MySQL uses JSON instead of JSONB
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    last_connected_at TIMESTAMP NULL,
    
    -- Constraints and indexes
    CONSTRAINT unique_host_name UNIQUE (host_name),
    CONSTRAINT unique_host_address UNIQUE (host_address),
    FOREIGN KEY (region_id) REFERENCES regions(id) ON DELETE CASCADE  -- Foreign key syntax fixed
);

-- Create indexes for better performance
CREATE INDEX idx_host_creds_physical ON host_creds (is_physical_node);
CREATE INDEX idx_host_creds_provider ON host_creds (provider_type);
CREATE INDEX idx_host_creds_region ON host_creds (region_id);


-- Create monitoring and logging tables with optimized storage
CREATE TABLE metrics (
    id BIGINT NOT NULL AUTO_INCREMENT,
    app_id BIGINT,
    metric_name VARCHAR(255) NOT NULL,
    metric_value DOUBLE NOT NULL,
    labels JSON,
    timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (id),
    KEY idx_metrics_app_id_timestamp (app_id, timestamp, metric_name),
    KEY idx_metrics_metric_name (metric_name),
    KEY idx_metrics_timestamp (timestamp),
    FOREIGN KEY (app_id) REFERENCES apps(id) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci
ROW_FORMAT=COMPRESSED KEY_BLOCK_SIZE=8;
-- PARTITION BY RANGE (TO_DAYS(timestamp)) (
--     PARTITION p_oldest VALUES LESS THAN (TO_DAYS('2020-01-01')),
--     PARTITION p_2020_q1 VALUES LESS THAN (TO_DAYS('2020-04-01')),
--     PARTITION p_2020_q2 VALUES LESS THAN (TO_DAYS('2020-07-01')),
--     PARTITION p_2020_q3 VALUES LESS THAN (TO_DAYS('2020-10-01')),
--     PARTITION p_2020_q4 VALUES LESS THAN (TO_DAYS('2021-01-01')),
--     PARTITION p_2021_q1 VALUES LESS THAN (TO_DAYS('2021-04-01')),
--     PARTITION p_2021_q2 VALUES LESS THAN (TO_DAYS('2021-07-01')),
--     PARTITION p_2021_q3 VALUES LESS THAN (TO_DAYS('2021-10-01')),
--     PARTITION p_2021_q4 VALUES LESS THAN (TO_DAYS('2022-01-01')),
--     PARTITION p_2022_q1 VALUES LESS THAN (TO_DAYS('2022-04-01')),
--     PARTITION p_2022_q2 VALUES LESS THAN (TO_DAYS('2022-07-01')),
--     PARTITION p_2022_q3 VALUES LESS THAN (TO_DAYS('2022-10-01')),
--     PARTITION p_2022_q4 VALUES LESS THAN (TO_DAYS('2023-01-01')),
--     PARTITION p_2023_q1 VALUES LESS THAN (TO_DAYS('2023-04-01')),
--     PARTITION p_2023_q2 VALUES LESS THAN (TO_DAYS('2023-07-01')),
--     PARTITION p_2023_q3 VALUES LESS THAN (TO_DAYS('2023-10-01')),
--     PARTITION p_2023_q4 VALUES LESS THAN (TO_DAYS('2024-01-01')),
--     PARTITION p_2024_q1 VALUES LESS THAN (TO_DAYS('2024-04-01')),
--     PARTITION p_2024_q2 VALUES LESS THAN (TO_DAYS('2024-07-01')),
--     PARTITION p_2024_q3 VALUES LESS THAN (TO_DAYS('2024-10-01')),
--     PARTITION p_2024_q4 VALUES LESS THAN (TO_DAYS('2025-01-01')),
--     PARTITION p_2025_q1 VALUES LESS THAN (TO_DAYS('2025-04-01')),
--     PARTITION p_2025_q2 VALUES LESS THAN (TO_DAYS('2025-07-01')),
--     PARTITION p_2025_q3 VALUES LESS THAN (TO_DAYS('2025-10-01')),
--     PARTITION p_2025_q4 VALUES LESS THAN (TO_DAYS('2026-01-01')),
--     PARTITION p_future VALUES LESS THAN MAXVALUE
-- );

CREATE TABLE instance_logs (
    id BIGINT NOT NULL AUTO_INCREMENT,
    instance_id BIGINT NOT NULL,
    app_id BIGINT NOT NULL,
    log_type ENUM('app', 'system', 'deployment', 'build', 'task', 'audit') NOT NULL,
    log_level ENUM('debug', 'info', 'warn', 'error', 'fatal') DEFAULT 'info',
    message TEXT NOT NULL,
    source VARCHAR(255),
    metadata JSON,
    timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (id),
    KEY idx_logs_instance_id_timestamp (instance_id, timestamp),
    KEY idx_logs_app_id (app_id),
    KEY idx_logs_log_type (log_type),
    KEY idx_logs_log_level (log_level),
    KEY idx_logs_timestamp (timestamp)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci 
ROW_FORMAT=COMPRESSED KEY_BLOCK_SIZE=8;
-- PARTITION BY RANGE (TO_DAYS(timestamp)) (
--     PARTITION p_oldest VALUES LESS THAN (TO_DAYS('2020-01-01')),
--     PARTITION p_2020_q1 VALUES LESS THAN (TO_DAYS('2020-04-01')),
--     PARTITION p_2020_q2 VALUES LESS THAN (TO_DAYS('2020-07-01')),
--     PARTITION p_2020_q3 VALUES LESS THAN (TO_DAYS('2020-10-01')),
--     PARTITION p_2020_q4 VALUES LESS THAN (TO_DAYS('2021-01-01')),
--     PARTITION p_2021_q1 VALUES LESS THAN (TO_DAYS('2021-04-01')),
--     PARTITION p_2021_q2 VALUES LESS THAN (TO_DAYS('2021-07-01')),
--     PARTITION p_2021_q3 VALUES LESS THAN (TO_DAYS('2021-10-01')),
--     PARTITION p_2021_q4 VALUES LESS THAN (TO_DAYS('2022-01-01')),
--     PARTITION p_2022_q1 VALUES LESS THAN (TO_DAYS('2022-04-01')),
--     PARTITION p_2022_q2 VALUES LESS THAN (TO_DAYS('2022-07-01')),
--     PARTITION p_2022_q3 VALUES LESS THAN (TO_DAYS('2022-10-01')),
--     PARTITION p_2022_q4 VALUES LESS THAN (TO_DAYS('2023-01-01')),
--     PARTITION p_2023_q1 VALUES LESS THAN (TO_DAYS('2023-04-01')),
--     PARTITION p_2023_q2 VALUES LESS THAN (TO_DAYS('2023-07-01')),
--     PARTITION p_2023_q3 VALUES LESS THAN (TO_DAYS('2023-10-01')),
--     PARTITION p_2023_q4 VALUES LESS THAN (TO_DAYS('2024-01-01')),
--     PARTITION p_2024_q1 VALUES LESS THAN (TO_DAYS('2024-04-01')),
--     PARTITION p_2024_q2 VALUES LESS THAN (TO_DAYS('2024-07-01')),
--     PARTITION p_2024_q3 VALUES LESS THAN (TO_DAYS('2024-10-01')),
--     PARTITION p_2024_q4 VALUES LESS THAN (TO_DAYS('2025-01-01')),
--     PARTITION p_2025_q1 VALUES LESS THAN (TO_DAYS('2025-04-01')),
--     PARTITION p_2025_q2 VALUES LESS THAN (TO_DAYS('2025-07-01')),
--     PARTITION p_2025_q3 VALUES LESS THAN (TO_DAYS('2025-10-01')),
--     PARTITION p_2025_q4 VALUES LESS THAN (TO_DAYS('2026-01-01')),
--     PARTITION p_future VALUES LESS THAN MAXVALUE
-- );

CREATE TABLE audit_logs (
    id BIGINT NOT NULL AUTO_INCREMENT,
    user_id BIGINT,
    org_id BIGINT,
    app_id BIGINT,
    action VARCHAR(255) NOT NULL,
    resource_type VARCHAR(255) NOT NULL,
    resource_id VARCHAR(255),
    before_state JSON,
    after_state JSON,
    details JSON,
    ip_address VARCHAR(45),
    user_agent TEXT,
    request_id VARCHAR(255),
    status ENUM('success', 'failure', 'warning') DEFAULT 'success',
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (id),
    KEY idx_audit_logs_created_at (created_at),
    KEY idx_audit_logs_user_id (user_id),
    KEY idx_audit_logs_org_id (org_id),
    KEY idx_audit_logs_app_id (app_id),
    KEY idx_audit_logs_action (action),
    KEY idx_audit_logs_resource_type (resource_type),
    KEY idx_audit_logs_status (status)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
-- ROW_FORMAT=COMPRESSED KEY_BLOCK_SIZE=8
-- PARTITION BY RANGE (TO_DAYS(created_at)) (
--     PARTITION p_oldest VALUES LESS THAN (TO_DAYS('2020-01-01')),
--     PARTITION p_2020_q1 VALUES LESS THAN (TO_DAYS('2020-04-01')),
--     PARTITION p_2020_q2 VALUES LESS THAN (TO_DAYS('2020-07-01')),
--     PARTITION p_2020_q3 VALUES LESS THAN (TO_DAYS('2020-10-01')),
--     PARTITION p_2020_q4 VALUES LESS THAN (TO_DAYS('2021-01-01')),
--     PARTITION p_2021_q1 VALUES LESS THAN (TO_DAYS('2021-04-01')),
--     PARTITION p_2021_q2 VALUES LESS THAN (TO_DAYS('2021-07-01')),
--     PARTITION p_2021_q3 VALUES LESS THAN (TO_DAYS('2021-10-01')),
--     PARTITION p_2021_q4 VALUES LESS THAN (TO_DAYS('2022-01-01')),
--     PARTITION p_2022_q1 VALUES LESS THAN (TO_DAYS('2022-04-01')),
--     PARTITION p_2022_q2 VALUES LESS THAN (TO_DAYS('2022-07-01')),
--     PARTITION p_2022_q3 VALUES LESS THAN (TO_DAYS('2022-10-01')),
--     PARTITION p_2022_q4 VALUES LESS THAN (TO_DAYS('2023-01-01')),
--     PARTITION p_2023_q1 VALUES LESS THAN (TO_DAYS('2023-04-01')),
--     PARTITION p_2023_q2 VALUES LESS THAN (TO_DAYS('2023-07-01')),
--     PARTITION p_2023_q3 VALUES LESS THAN (TO_DAYS('2023-10-01')),
--     PARTITION p_2023_q4 VALUES LESS THAN (TO_DAYS('2024-01-01')),
--     PARTITION p_2024_q1 VALUES LESS THAN (TO_DAYS('2024-04-01')),
--     PARTITION p_2024_q2 VALUES LESS THAN (TO_DAYS('2024-07-01')),
--     PARTITION p_2024_q3 VALUES LESS THAN (TO_DAYS('2024-10-01')),
--     PARTITION p_2024_q4 VALUES LESS THAN (TO_DAYS('2025-01-01')),
--     PARTITION p_2025_q1 VALUES LESS THAN (TO_DAYS('2025-04-01')),
--     PARTITION p_2025_q2 VALUES LESS THAN (TO_DAYS('2025-07-01')),
--     PARTITION p_2025_q3 VALUES LESS THAN (TO_DAYS('2025-10-01')),
--     PARTITION p_2025_q4 VALUES LESS THAN (TO_DAYS('2026-01-01')),
--     PARTITION p_future VALUES LESS THAN MAXVALUE
-- );

CREATE TABLE notifications (
    id BIGINT NOT NULL AUTO_INCREMENT,
    user_id BIGINT,
    org_id BIGINT,
    app_id BIGINT,
    notification_type ENUM('info', 'warning', 'error', 'success') DEFAULT 'info',
    message TEXT NOT NULL,
    read_status TINYINT(1) DEFAULT 0,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (id),
    KEY idx_notifications_user_id (user_id),
    KEY idx_notifications_org_id (org_id),
    KEY idx_notifications_app_id (app_id),
    KEY idx_notifications_created_at (created_at)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE backups (
    id BIGINT PRIMARY KEY AUTO_INCREMENT,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_by VARCHAR(255) NOT NULL,
    backup_type ENUM('PLATFORM', 'APPLICATION', 'PARTIAL') NOT NULL,
    status ENUM('CREATING', 'AVAILABLE', 'RESTORING', 'FAILED', 'DELETED') NOT NULL DEFAULT 'CREATING',
    
    -- Core metadata (always required)
    format_version VARCHAR(50) NOT NULL,
    source_environment VARCHAR(255) NOT NULL,
    encryption_method VARCHAR(100),
    encryption_key_id BIGINT,
    size_bytes BIGINT,
    
    -- Platform backup components (optional for app backups)
    has_system_core BOOLEAN DEFAULT FALSE,
    has_directors BOOLEAN DEFAULT FALSE,
    has_orchestrators BOOLEAN DEFAULT FALSE,
    has_network_config BOOLEAN DEFAULT FALSE,
    
    -- Application backup components
    has_app_definitions BOOLEAN DEFAULT FALSE,
    has_volume_data BOOLEAN DEFAULT FALSE,
    
    -- Partial backup filters (stored as JSON strings in MySQL)
    included_apps TEXT,
    included_services TEXT,
    
    -- Recovery metadata
    last_validated_at TIMESTAMP NULL,
    last_restored_at TIMESTAMP NULL,
    restore_target_environment VARCHAR(255),
    restore_status VARCHAR(50),
    
    -- Storage information
    storage_location TEXT NOT NULL,
    manifest_path TEXT NOT NULL,
    
    -- Additional metadata as JSON
    metadata JSON
);


-- Notifications for individual users
CREATE TABLE user_notifications (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    user_id BIGINT NOT NULL,
    org_id BIGINT,
    app_id BIGINT,
    notification_type VARCHAR(50) NOT NULL,
    message TEXT NOT NULL,
    read_status BOOLEAN DEFAULT FALSE,
    importance VARCHAR(20) DEFAULT 'normal',
    action_url VARCHAR(255),
    action_label VARCHAR(100),
    created_at DATETIME NOT NULL,
    expires_at DATETIME,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (org_id) REFERENCES orgs(id) ON DELETE CASCADE,
    FOREIGN KEY (app_id) REFERENCES apps(id) ON DELETE CASCADE,
    INDEX idx_user_notifications_user_id (user_id),
    INDEX idx_user_notifications_org_id (org_id),
    INDEX idx_user_notifications_app_id (app_id),
    INDEX idx_user_notifications_created_at (created_at),
    INDEX idx_user_notifications_type (notification_type)
);

-- Notifications for roles
CREATE TABLE role_notifications (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    role_id BIGINT NOT NULL,
    org_id BIGINT,
    app_id BIGINT,
    notification_type VARCHAR(50) NOT NULL,
    message TEXT NOT NULL,
    importance VARCHAR(20) DEFAULT 'normal',
    action_url VARCHAR(255),
    action_label VARCHAR(100),
    created_at DATETIME NOT NULL,
    expires_at DATETIME,
    FOREIGN KEY (role_id) REFERENCES roles(id) ON DELETE CASCADE,
    FOREIGN KEY (org_id) REFERENCES orgs(id) ON DELETE CASCADE,
    FOREIGN KEY (app_id) REFERENCES apps(id) ON DELETE CASCADE,
    INDEX idx_role_notifications_role_id (role_id),
    INDEX idx_role_notifications_org_id (org_id),
    INDEX idx_role_notifications_app_id (app_id),
    INDEX idx_role_notifications_created_at (created_at),
    INDEX idx_role_notifications_type (notification_type)
);

-- User notification acknowledgments
CREATE TABLE notification_acknowledgments (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    user_id BIGINT NOT NULL,
    notification_id BIGINT,
    role_notification_id BIGINT,
    acknowledged_at DATETIME NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (notification_id) REFERENCES user_notifications(id) ON DELETE CASCADE,
    FOREIGN KEY (role_notification_id) REFERENCES role_notifications(id) ON DELETE CASCADE,
    INDEX idx_notification_acks_user_id (user_id),
    INDEX idx_notification_acks_notification_id (notification_id),
    INDEX idx_notification_acks_role_notification_id (role_notification_id),
    CONSTRAINT ck_notification_reference CHECK (
        (notification_id IS NULL AND role_notification_id IS NOT NULL) OR
        (notification_id IS NOT NULL AND role_notification_id IS NULL)
    )
);

-- System alerts table
CREATE TABLE alerts (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    alert_type VARCHAR(50) NOT NULL,
    severity ENUM('critical', 'warning', 'info') NOT NULL,
    service VARCHAR(100) NOT NULL,
    message TEXT NOT NULL,
    timestamp DATETIME NOT NULL,
    status ENUM('active', 'acknowledged', 'resolved', 'auto_resolved') NOT NULL DEFAULT 'active',
    resolved_at DATETIME,
    resolved_by BIGINT,
    metadata JSON,
    org_id BIGINT,
    app_id BIGINT,
    instance_id BIGINT,
    region_id BIGINT,
    node_id BIGINT,
    FOREIGN KEY (resolved_by) REFERENCES users(id) ON DELETE SET NULL,
    FOREIGN KEY (org_id) REFERENCES orgs(id) ON DELETE CASCADE,
    FOREIGN KEY (app_id) REFERENCES apps(id) ON DELETE CASCADE,
    FOREIGN KEY (instance_id) REFERENCES instances(id) ON DELETE CASCADE,
    FOREIGN KEY (region_id) REFERENCES regions(id) ON DELETE CASCADE,
    FOREIGN KEY (node_id) REFERENCES workers(id) ON DELETE CASCADE,
    INDEX idx_alerts_type (alert_type),
    INDEX idx_alerts_severity (severity),
    INDEX idx_alerts_service (service),
    INDEX idx_alerts_timestamp (timestamp),
    INDEX idx_alerts_status (status),
    INDEX idx_alerts_org_id (org_id),
    INDEX idx_alerts_app_id (app_id),
    INDEX idx_alerts_instance_id (instance_id),
    INDEX idx_alerts_region_id (region_id),
    INDEX idx_alerts_node_id (node_id)
);

-- Alert acknowledgments
CREATE TABLE alert_acknowledgments (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    alert_id BIGINT NOT NULL,
    user_id BIGINT NOT NULL,
    acknowledged_at DATETIME NOT NULL,
    notes TEXT,
    FOREIGN KEY (alert_id) REFERENCES alerts(id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    INDEX idx_alert_acks_alert_id (alert_id),
    INDEX idx_alert_acks_user_id (user_id)
);

-- Alert escalations
CREATE TABLE alert_escalations (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    alert_id BIGINT NOT NULL,
    escalation_level BIGINT NOT NULL,
    escalated_at DATETIME NOT NULL,
    escalated_to JSON, -- Could be roles, users, external systems
    escalation_method VARCHAR(50) NOT NULL, -- email, sms, webhook, etc.
    response_required_by DATETIME,
    FOREIGN KEY (alert_id) REFERENCES alerts(id) ON DELETE CASCADE,
    INDEX idx_alert_escalations_alert_id (alert_id),
    INDEX idx_alert_escalations_level (escalation_level)
);

-- Alert history for auditing
CREATE TABLE alert_history (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    alert_id BIGINT NOT NULL,
    action VARCHAR(50) NOT NULL,
    performed_by BIGINT,
    performed_at DATETIME NOT NULL,
    previous_state JSON,
    new_state JSON,
    notes TEXT,
    FOREIGN KEY (alert_id) REFERENCES alerts(id) ON DELETE CASCADE,
    FOREIGN KEY (performed_by) REFERENCES users(id) ON DELETE SET NULL,
    INDEX idx_alert_history_alert_id (alert_id),
    INDEX idx_alert_history_performed_at (performed_at)
);

-- Index for faster queries
CREATE INDEX idx_backups_type ON backups(backup_type);
CREATE INDEX idx_backups_status ON backups(status);
CREATE INDEX idx_backups_created_at ON backups(created_at);

-- CREATE TABLE user_settings (
--     id BIGINT NOT NULL AUTO_INCREMENT,
--     user_id BIGINT NOT NULL,
--     setting_key VARCHAR(255) NOT NULL,
--     setting_value TEXT,
--     created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
--     updated_at DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
--     PRIMARY KEY (id),
--     UNIQUE KEY unique_user_setting (user_id, setting_key),
--     KEY idx_user_settings_user_id (user_id)
-- ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;