-- Drop all tables first (in correct dependency order)
DROP TABLE IF EXISTS metrics, allocations, instance_logs, audit_logs, api_keys, 
    config_vars, deployment_logs, rollbacks, deployments, builds, tasks, 
    autoscaling_rules, health_checks, network_policies, service_bindings,
    routes, instances, domains, apps, spaces, orgmember, permissions_role, 
    role_user, permissions, roles, quotas, orgs, users, data_services, 
    nodes, regions;

-- Create independent tables first (no foreign keys)
CREATE TABLE users (
    id BIGINT NOT NULL AUTO_INCREMENT,
    email VARCHAR(255) NOT NULL,
    name VARCHAR(255) NOT NULL,
    password VARCHAR(255) NOT NULL,
    salt VARCHAR(255) NOT NULL,
    active TINYINT(1) DEFAULT 0,
    last_login_at DATETIME,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    PRIMARY KEY (id),
    UNIQUE KEY unique_email (email)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE roles (
    id BIGINT NOT NULL AUTO_INCREMENT,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (id),
    UNIQUE KEY unique_name (name)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE permissions (
    id BIGINT NOT NULL AUTO_INCREMENT,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    resource_type VARCHAR(255),
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (id),
    UNIQUE KEY unique_name (name)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE regions (
    id BIGINT NOT NULL AUTO_INCREMENT,
    name VARCHAR(255) NOT NULL,
    provider ENUM('kubernetes', 'docker', 'custom') NOT NULL,
    status ENUM('active', 'maintenance', 'offline') DEFAULT 'active',
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (id),
    UNIQUE KEY unique_name (name)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE orgs (
    id BIGINT NOT NULL AUTO_INCREMENT,
    name VARCHAR(255) NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    PRIMARY KEY (id),
    UNIQUE KEY unique_name (name)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE allocations (
    id BIGINT NOT NULL AUTO_INCREMENT,
    cpu DOUBLE NOT NULL,
    memory DOUBLE NOT NULL,
    uplink DOUBLE NOT NULL,
    downlink DOUBLE NOT NULL,
    disk DOUBLE NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (id)
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
    PRIMARY KEY (user_id, role_id),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (role_id) REFERENCES roles(id) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE orgmember (
    id BIGINT NOT NULL AUTO_INCREMENT,
    org_id BIGINT NOT NULL,
    user_id BIGINT NOT NULL,
    role ENUM('owner', 'admin', 'member') DEFAULT 'member',
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (id),
    UNIQUE KEY unique_org_user (org_id, user_id),
    KEY idx_orgmember_org_id (org_id),
    KEY idx_orgmember_user_id (user_id),
    FOREIGN KEY (org_id) REFERENCES orgs(id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE api_keys (
    id BIGINT NOT NULL AUTO_INCREMENT,
    org_id BIGINT NOT NULL,
    name VARCHAR(255) NOT NULL,
    key_hash VARCHAR(255) NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (id),
    KEY idx_api_keys_org_id (org_id),
    FOREIGN KEY (org_id) REFERENCES orgs(id) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE quotas (
    id BIGINT NOT NULL AUTO_INCREMENT,
    name VARCHAR(255) NOT NULL,
    org_id BIGINT NOT NULL,
    memory_limit BIGINT NOT NULL,
    instance_limit INT NOT NULL,
    routes_limit INT NOT NULL,
    services_limit INT NOT NULL,
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
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    PRIMARY KEY (id),
    UNIQUE KEY unique_name_org (name, org_id),
    KEY idx_spaces_org_id (org_id),
    FOREIGN KEY (org_id) REFERENCES orgs(id) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE nodes (
    id BIGINT NOT NULL AUTO_INCREMENT,
    region_id BIGINT NOT NULL,
    name VARCHAR(255) NOT NULL,
    status ENUM('active', 'provisioning', 'maintenance', 'powered_off', 'unreachable', 'degraded') DEFAULT 'active',
    cpu_total DOUBLE NOT NULL,
    cpu_available DOUBLE NOT NULL,
    memory_total DOUBLE NOT NULL,
    memory_available DOUBLE NOT NULL,
    disk_total DOUBLE NOT NULL,
    disk_available DOUBLE NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    PRIMARY KEY (id),
    KEY idx_nodes_region_id (region_id),
    KEY idx_nodes_status (status),
    FOREIGN KEY (region_id) REFERENCES regions(id) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE data_services (
    id BIGINT NOT NULL AUTO_INCREMENT,
    region_id BIGINT NOT NULL,
    name VARCHAR(255) NOT NULL,
    service_type ENUM('database', 'cache', 'message_queue', 'network_filesystem', 'cdn') NOT NULL,
    status ENUM('active', 'provisioning', 'maintenance', 'powered_off', 'unreachable', 'degraded') DEFAULT 'active',
    version VARCHAR(50),
    plan VARCHAR(50) NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    PRIMARY KEY (id),
    KEY idx_data_services_region_id (region_id),
    KEY idx_data_services_status (status),
    KEY idx_data_services_type (service_type),
    FOREIGN KEY (region_id) REFERENCES regions(id) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- Create application-related tables
CREATE TABLE apps (
    id BIGINT NOT NULL AUTO_INCREMENT,
    name VARCHAR(255) NOT NULL,
    org_id BIGINT NOT NULL,
    space_id BIGINT,
    git_repo VARCHAR(255),
    git_branch VARCHAR(255) DEFAULT 'main',
    container_image_url VARCHAR(255),
    default_allocation_id BIGINT,
    region_id BIGINT,
    instances INT DEFAULT 1,
    health_check_type ENUM('http', 'port', 'process') DEFAULT 'port',
    health_check_endpoint VARCHAR(255),
    runtime VARCHAR(255),
    restart_policy ENUM('always', 'on-failure', 'no') DEFAULT 'always',
    maintenance_mode TINYINT(1) DEFAULT 0,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    PRIMARY KEY (id),
    UNIQUE KEY unique_name_org (name, org_id),
    KEY idx_apps_name (name),
    KEY idx_apps_org_id (org_id),
    KEY idx_apps_space_id (space_id),
    KEY idx_apps_region_id (region_id),
    FOREIGN KEY (org_id) REFERENCES orgs(id) ON DELETE CASCADE,
    FOREIGN KEY (space_id) REFERENCES spaces(id),
    FOREIGN KEY (default_allocation_id) REFERENCES allocations(id),
    FOREIGN KEY (region_id) REFERENCES regions(id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE instances (
    id BIGINT NOT NULL AUTO_INCREMENT,
    app_id BIGINT NOT NULL,
    instance_type VARCHAR(255) NOT NULL,
    status ENUM('active', 'provisioning', 'maintenance', 'powered_off', 'unreachable', 'degraded') DEFAULT 'active',
    container_id VARCHAR(255),
    allocation_id BIGINT,
    node_id BIGINT,
    instance_status ENUM('running', 'stopped', 'terminated', 'failed') DEFAULT 'running',
    instance_index INT NOT NULL,
    last_health_check DATETIME,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    PRIMARY KEY (id),
    UNIQUE KEY unique_app_index (app_id, instance_index),
    KEY idx_instances_app_id (app_id),
    KEY idx_instances_status (status),
    KEY idx_instances_node_id (node_id),
    FOREIGN KEY (app_id) REFERENCES apps(id) ON DELETE CASCADE,
    FOREIGN KEY (allocation_id) REFERENCES allocations(id),
    FOREIGN KEY (node_id) REFERENCES nodes(id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE domains (
    id BIGINT NOT NULL AUTO_INCREMENT,
    name VARCHAR(255) NOT NULL,
    ssl_enabled TINYINT(1) DEFAULT 0,
    ssl_cert_path VARCHAR(255),
    ssl_key_path VARCHAR(255),
    ssl_expiry_date DATE,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    PRIMARY KEY (id),
    UNIQUE KEY unique_name (name),
    KEY idx_domains_ssl_expiry (ssl_expiry_date)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE routes (
    id BIGINT NOT NULL AUTO_INCREMENT,
    domain_id BIGINT NOT NULL,
    path VARCHAR(255) DEFAULT '',
    app_id BIGINT,
    weight INT DEFAULT 100,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    PRIMARY KEY (id),
    UNIQUE KEY unique_domain_path (domain_id, path),
    KEY idx_routes_domain_id (domain_id),
    KEY idx_routes_app_id (app_id),
    FOREIGN KEY (domain_id) REFERENCES domains(id) ON DELETE CASCADE,
    FOREIGN KEY (app_id) REFERENCES apps(id) ON DELETE SET NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE service_bindings (
    id BIGINT NOT NULL AUTO_INCREMENT,
    app_id BIGINT NOT NULL,
    service_id BIGINT NOT NULL,
    credentials TEXT,
    binding_name VARCHAR(255),
    status ENUM('creating', 'created', 'deleting', 'deleted', 'failed') DEFAULT 'creating',
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    PRIMARY KEY (id),
    UNIQUE KEY unique_app_service (app_id, service_id),
    KEY idx_service_bindings_app_id (app_id),
    KEY idx_service_bindings_service_id (service_id),
    FOREIGN KEY (app_id) REFERENCES apps(id) ON DELETE CASCADE,
    FOREIGN KEY (service_id) REFERENCES data_services(id) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE builds (
    id BIGINT NOT NULL AUTO_INCREMENT,
    app_id BIGINT NOT NULL,
    source_version VARCHAR(255),
    status ENUM('pending', 'building', 'succeeded', 'failed') DEFAULT 'pending',
    build_pack_used VARCHAR(255),
    log_url VARCHAR(255),
    started_at DATETIME,
    completed_at DATETIME,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (id),
    KEY idx_builds_app_id (app_id),
    KEY idx_builds_status (status),
    KEY idx_builds_created_at (created_at),
    FOREIGN KEY (app_id) REFERENCES apps(id) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE deployments (
    id BIGINT NOT NULL AUTO_INCREMENT,
    app_id BIGINT NOT NULL,
    build_id BIGINT NOT NULL,
    status ENUM('pending', 'in_progress', 'deployed', 'failed', 'canceled') DEFAULT 'pending',
    deployment_strategy ENUM('rolling', 'blue-green', 'canary', 'recreate') DEFAULT 'rolling',
    started_at DATETIME,
    completed_at DATETIME,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (id),
    KEY idx_deployments_app_id (app_id),
    KEY idx_deployments_build_id (build_id),
    KEY idx_deployments_status (status),
    KEY idx_deployments_created_at (created_at),
    FOREIGN KEY (app_id) REFERENCES apps(id) ON DELETE CASCADE,
    FOREIGN KEY (build_id) REFERENCES builds(id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE rollbacks (
    id BIGINT NOT NULL AUTO_INCREMENT,
    app_id BIGINT NOT NULL,
    from_deployment_id BIGINT NOT NULL,
    to_deployment_id BIGINT NOT NULL,
    status ENUM('pending', 'in_progress', 'completed', 'failed') DEFAULT 'pending',
    reason TEXT,
    started_at DATETIME,
    completed_at DATETIME,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (id),
    KEY idx_rollbacks_app_id (app_id),
    KEY idx_rollbacks_status (status),
    KEY idx_rollbacks_created_at (created_at),
    FOREIGN KEY (app_id) REFERENCES apps(id) ON DELETE CASCADE,
    FOREIGN KEY (from_deployment_id) REFERENCES deployments(id),
    FOREIGN KEY (to_deployment_id) REFERENCES deployments(id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE deployment_logs (
    id BIGINT NOT NULL AUTO_INCREMENT,
    deployment_id BIGINT NOT NULL,
    log_type ENUM('app', 'system', 'deployment') NOT NULL,
    message TEXT NOT NULL,
    timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (id),
    KEY idx_deployment_logs_deployment_id (deployment_id),
    KEY idx_deployment_logs_timestamp (timestamp),
    FOREIGN KEY (deployment_id) REFERENCES deployments(id) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE config_vars (
    id BIGINT NOT NULL AUTO_INCREMENT,
    app_id BIGINT NOT NULL,
    `key` VARCHAR(255) NOT NULL,
    value TEXT,
    is_secret TINYINT(1) DEFAULT 0,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    PRIMARY KEY (id),
    UNIQUE KEY unique_app_key (app_id, `key`),
    KEY idx_config_vars_app_id (app_id),
    FOREIGN KEY (app_id) REFERENCES apps(id) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE tasks (
    id BIGINT NOT NULL AUTO_INCREMENT,
    app_id BIGINT NOT NULL,
    command TEXT NOT NULL,
    name VARCHAR(255),
    status ENUM('pending', 'running', 'succeeded', 'failed', 'canceled') DEFAULT 'pending',
    memory_in_mb INT,
    disk_in_mb INT,
    result TEXT,
    started_at DATETIME,
    completed_at DATETIME,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (id),
    KEY idx_tasks_app_id (app_id),
    KEY idx_tasks_status (status),
    KEY idx_tasks_created_at (created_at),
    FOREIGN KEY (app_id) REFERENCES apps(id) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE health_checks (
    id BIGINT NOT NULL AUTO_INCREMENT,
    app_id BIGINT NOT NULL,
    type ENUM('http', 'port', 'process') NOT NULL DEFAULT 'port',
    endpoint VARCHAR(255),
    timeout INT NOT NULL DEFAULT 60,
    check_interval INT NOT NULL DEFAULT 10,
    healthy_threshold INT NOT NULL DEFAULT 3,
    unhealthy_threshold INT NOT NULL DEFAULT 3,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    PRIMARY KEY (id),
    KEY idx_health_checks_app_id (app_id),
    FOREIGN KEY (app_id) REFERENCES apps(id) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE autoscaling_rules (
    id BIGINT NOT NULL AUTO_INCREMENT,
    app_id BIGINT NOT NULL,
    min_instances INT NOT NULL DEFAULT 1,
    max_instances INT NOT NULL DEFAULT 1,
    metric_type ENUM('cpu', 'memory', 'http_throughput', 'http_latency', 'queue_depth') NOT NULL,
    threshold_value DOUBLE NOT NULL,
    cool_down_period_seconds INT NOT NULL DEFAULT 300,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    PRIMARY KEY (id),
    KEY idx_autoscaling_rules_app_id (app_id),
    FOREIGN KEY (app_id) REFERENCES apps(id) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE network_policies (
    id BIGINT NOT NULL AUTO_INCREMENT,
    source_app_id BIGINT NOT NULL,
    destination_app_id BIGINT NOT NULL,
    protocol ENUM('tcp', 'udp', 'icmp') NOT NULL DEFAULT 'tcp',
    port_range_start INT,
    port_range_end INT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    PRIMARY KEY (id),
    KEY idx_network_policies_source (source_app_id),
    KEY idx_network_policies_destination (destination_app_id),
    FOREIGN KEY (source_app_id) REFERENCES apps(id) ON DELETE CASCADE,
    FOREIGN KEY (destination_app_id) REFERENCES apps(id) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- Create monitoring and logging tables without partitioning
CREATE TABLE metrics (
    id BIGINT NOT NULL AUTO_INCREMENT,
    instance_id BIGINT NOT NULL,
    metric_name VARCHAR(255) NOT NULL,
    metric_value DOUBLE NOT NULL,
    timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (id),
    KEY idx_metrics_instance_id_timestamp (instance_id, timestamp, metric_name),
    KEY idx_metrics_metric_name (metric_name),
    KEY idx_metrics_timestamp (timestamp)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci ROW_FORMAT=COMPRESSED KEY_BLOCK_SIZE=8;

CREATE TABLE instance_logs (
    id BIGINT NOT NULL AUTO_INCREMENT,
    instance_id BIGINT NOT NULL,
    log_type ENUM('app', 'system', 'deployment') NOT NULL,
    message TEXT NOT NULL,
    timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (id),
    KEY idx_logs_instance_id_timestamp (instance_id, timestamp),
    KEY idx_logs_log_type (log_type),
    KEY idx_logs_timestamp (timestamp)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci ROW_FORMAT=COMPRESSED KEY_BLOCK_SIZE=8;

CREATE TABLE audit_logs (
    id BIGINT NOT NULL AUTO_INCREMENT,
    user_id BIGINT,
    org_id BIGINT,
    action VARCHAR(255) NOT NULL,
    resource_type VARCHAR(255) NOT NULL,
    resource_id VARCHAR(255),
    ip_address VARCHAR(45),
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (id),
    KEY idx_audit_logs_created_at (created_at),
    KEY idx_audit_logs_user_id (user_id),
    KEY idx_audit_logs_org_id (org_id),
    KEY idx_audit_logs_action (action),
    KEY idx_audit_logs_resource_type (resource_type)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci ROW_FORMAT=COMPRESSED KEY_BLOCK_SIZE=8;

-- Remove partitioning to avoid MySQL limitations
-- ALTER TABLE metrics PARTITION BY RANGE (TO_DAYS(timestamp)) (
--    PARTITION p_2023 VALUES LESS THAN (TO_DAYS('2024-01-01')),
--    PARTITION p_2024_q1 VALUES LESS THAN (TO_DAYS('2024-04-01')),
--    PARTITION p_2024_q2 VALUES LESS THAN (TO_DAYS('2024-07-01')),
--    PARTITION p_2024_q3 VALUES LESS THAN (TO_DAYS('2024-10-01')),
--    PARTITION p_2024_q4 VALUES LESS THAN (TO_DAYS('2025-01-01')),
--    PARTITION p_future VALUES LESS THAN MAXVALUE
-- );

-- ALTER TABLE instance_logs PARTITION BY RANGE (TO_DAYS(timestamp)) (
--    PARTITION p_2023 VALUES LESS THAN (TO_DAYS('2024-01-01')),
--    PARTITION p_2024_q1 VALUES LESS THAN (TO_DAYS('2024-04-01')),
--    PARTITION p_2024_q2 VALUES LESS THAN (TO_DAYS('2024-07-01')),
--    PARTITION p_2024_q3 VALUES LESS THAN (TO_DAYS('2024-10-01')),
--    PARTITION p_2024_q4 VALUES LESS THAN (TO_DAYS('2025-01-01')),
--    PARTITION p_future VALUES LESS THAN MAXVALUE
-- );

-- ALTER TABLE audit_logs PARTITION BY RANGE (TO_DAYS(created_at)) (
--    PARTITION p_2023 VALUES LESS THAN (TO_DAYS('2024-01-01')),
--    PARTITION p_2024_q1 VALUES LESS THAN (TO_DAYS('2024-04-01')),
--    PARTITION p_2024_q2 VALUES LESS THAN (TO_DAYS('2024-07-01')),
--    PARTITION p_2024_q3 VALUES LESS THAN (TO_DAYS('2024-10-01')),
--    PARTITION p_2024_q4 VALUES LESS THAN (TO_DAYS('2025-01-01')),
--    PARTITION p_future VALUES LESS THAN MAXVALUE
-- );

-- Create procedures for partition management (auto-rotate partitions)
-- Comment out stored procedures for partitioning since we're not using partitioning
-- DELIMITER //

-- CREATE PROCEDURE create_metrics_partition(IN partition_date DATE)
-- BEGIN
--     DECLARE partition_name VARCHAR(255);
--     DECLARE partition_until DATE;
--     DECLARE partition_exists INT DEFAULT 0;
--     
--     SET partition_name = CONCAT('p_', DATE_FORMAT(partition_date, '%Y%m%d'));
--     SET partition_until = DATE_ADD(partition_date, INTERVAL 1 DAY);
--     
--     -- Check if partition already exists
--     SELECT COUNT(*) INTO partition_exists
--     FROM information_schema.partitions
--     WHERE table_schema = DATABASE()
--       AND table_name = 'metrics'
--       AND partition_name = partition_name;
--       
--     IF partition_exists = 0 THEN
--         SET @sql = CONCAT('ALTER TABLE metrics REORGANIZE PARTITION p_future INTO (
--             PARTITION ', partition_name, ' VALUES LESS THAN (TO_DAYS(''', partition_until, ''')),
--             PARTITION p_future VALUES LESS THAN MAXVALUE)');
--         
--         PREPARE stmt FROM @sql;
--         EXECUTE stmt;
--         DEALLOCATE PREPARE stmt;
--     END IF;
-- END //

-- CREATE PROCEDURE create_instance_logs_partition(IN partition_date DATE)
-- BEGIN
--     DECLARE partition_name VARCHAR(255);
--     DECLARE partition_until DATE;
--     DECLARE partition_exists INT DEFAULT 0;
--     
--     SET partition_name = CONCAT('p_', DATE_FORMAT(partition_date, '%Y%m%d'));
--     SET partition_until = DATE_ADD(partition_date, INTERVAL 1 DAY);
--     
--     -- Check if partition already exists
--     SELECT COUNT(*) INTO partition_exists
--     FROM information_schema.partitions
--     WHERE table_schema = DATABASE()
--       AND table_name = 'instance_logs'
--       AND partition_name = partition_name;
--       
--     IF partition_exists = 0 THEN
--         SET @sql = CONCAT('ALTER TABLE instance_logs REORGANIZE PARTITION p_future INTO (
--             PARTITION ', partition_name, ' VALUES LESS THAN (TO_DAYS(''', partition_until, ''')),
--             PARTITION p_future VALUES LESS THAN MAXVALUE)');
--         
--         PREPARE stmt FROM @sql;
--         EXECUTE stmt;
--         DEALLOCATE PREPARE stmt;
--     END IF;
-- END //

-- CREATE PROCEDURE create_audit_logs_partition(IN partition_date DATE)
-- BEGIN
--     DECLARE partition_name VARCHAR(255);
--     DECLARE partition_until DATE;
--     DECLARE partition_exists INT DEFAULT 0;
--     
--     SET partition_name = CONCAT('p_', DATE_FORMAT(partition_date, '%Y%m%d'));
--     SET partition_until = DATE_ADD(partition_date, INTERVAL 1 DAY);
--     
--     -- Check if partition already exists
--     SELECT COUNT(*) INTO partition_exists
--     FROM information_schema.partitions
--     WHERE table_schema = DATABASE()
--       AND table_name = 'audit_logs'
--       AND partition_name = partition_name;
--       
--     IF partition_exists = 0 THEN
--         SET @sql = CONCAT('ALTER TABLE audit_logs REORGANIZE PARTITION p_future INTO (
--             PARTITION ', partition_name, ' VALUES LESS THAN (TO_DAYS(''', partition_until, ''')),
--             PARTITION p_future VALUES LESS THAN MAXVALUE)');
--         
--         PREPARE stmt FROM @sql;
--         EXECUTE stmt;
--         DEALLOCATE PREPARE stmt;
--     END IF;
-- END //

-- CREATE PROCEDURE drop_old_metrics_partitions(IN days_to_keep INT)
-- BEGIN
--     DECLARE partition_date DATE;
--     DECLARE partition_name VARCHAR(255);
--     SET partition_date = DATE_SUB(CURRENT_DATE, INTERVAL days_to_keep DAY);
--     
--     SELECT PARTITION_NAME INTO partition_name 
--     FROM INFORMATION_SCHEMA.PARTITIONS 
--     WHERE TABLE_NAME = 'metrics' 
--     AND PARTITION_NAME != 'p_future' 
--     AND PARTITION_NAME != 'p_current'
--     AND PARTITION_DESCRIPTION < TO_DAYS(partition_date)
--     LIMIT 1;
--     
--     IF partition_name IS NOT NULL THEN
--         SET @sql = CONCAT('ALTER TABLE metrics DROP PARTITION ', partition_name);
--         PREPARE stmt FROM @sql;
--         EXECUTE stmt;
--         DEALLOCATE PREPARE stmt;
--     END IF;
-- END //

-- Add compression to large tables for better performance
ALTER TABLE metrics ROW_FORMAT=COMPRESSED KEY_BLOCK_SIZE=8;
ALTER TABLE instance_logs ROW_FORMAT=COMPRESSED KEY_BLOCK_SIZE=8;
ALTER TABLE audit_logs ROW_FORMAT=COMPRESSED KEY_BLOCK_SIZE=8;

-- Note: Stored procedures for deployments and rollbacks are removed for initial schema setup
-- They can be added later as separate migration scripts

-- Add compression to large tables for better performance
ALTER TABLE metrics ROW_FORMAT=COMPRESSED KEY_BLOCK_SIZE=8;
ALTER TABLE instance_logs ROW_FORMAT=COMPRESSED KEY_BLOCK_SIZE=8;
ALTER TABLE audit_logs ROW_FORMAT=COMPRESSED KEY_BLOCK_SIZE=8;