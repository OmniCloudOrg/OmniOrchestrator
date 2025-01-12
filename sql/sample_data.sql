-- Disable foreign key checks and turn off autocommit for faster inserts
SET FOREIGN_KEY_CHECKS = 0;
SET autocommit = 0;
SET unique_checks = 0;

START TRANSACTION;

-- Insert Regions (pre-computed values)
INSERT INTO cluster.regions (name, provider, status) VALUES
('us-east-1', 'kubernetes', 'active'),
('us-west-1', 'kubernetes', 'active'),
('eu-west-1', 'kubernetes', 'active'),
('ap-south-1', 'kubernetes', 'maintenance'),
('custom-dc-1', 'custom', 'active');

-- Insert Users (pre-computed values)
INSERT INTO cluster.users (email, name, password, salt, active, last_login_at) VALUES
('admin@example.com', 'Admin User', '$2a$12$LQVkMqLYXqxhph', 'randomsalt123', 1, DATE_SUB(NOW(), INTERVAL 1 DAY)),
('john.doe@example.com', 'John Doe', '$2a$12$LQVkMqLYXqxhph', 'randomsalt124', 1, DATE_SUB(NOW(), INTERVAL 2 DAY)),
('jane.smith@example.com', 'Jane Smith', '$2a$12$LQVkMqLYXqxhph', 'randomsalt125', 1, DATE_SUB(NOW(), INTERVAL 3 DAY)),
('bob.wilson@example.com', 'Bob Wilson', '$2a$12$LQVkMqLYXqxhph', 'randomsalt126', 1, DATE_SUB(NOW(), INTERVAL 4 DAY)),
('alice.johnson@example.com', 'Alice Johnson', '$2a$12$LQVkMqLYXqxhph', 'randomsalt127', 0, NULL);

-- Insert Roles and store IDs in variables
INSERT INTO cluster.roles (name, description) VALUES
('super_admin', 'Full system access'),
('org_admin', 'Organization administrator'),
('developer', 'Application developer'),
('viewer', 'Read-only access');

SET @super_admin_role_id = (SELECT id FROM cluster.roles WHERE name = 'super_admin');
SET @org_admin_role_id = (SELECT id FROM cluster.roles WHERE name = 'org_admin');

-- Insert Permissions with batch insert
INSERT INTO cluster.permissions (name, description, resource_type) VALUES
('app.create', 'Create applications', 'app'),
('app.delete', 'Delete applications', 'app'),
('app.deploy', 'Deploy applications', 'app'),
('app.view', 'View applications', 'app'),
('org.manage', 'Manage organization settings', 'org'),
('user.manage', 'Manage users', 'user');

-- Link Permissions to Roles more efficiently
INSERT INTO cluster.permissions_role (permissions_id, role_id)
SELECT p.id, @super_admin_role_id
FROM cluster.permissions p;

INSERT INTO cluster.permissions_role (permissions_id, role_id)
SELECT p.id, @org_admin_role_id
FROM cluster.permissions p
WHERE p.name IN ('app.create', 'app.deploy', 'app.view', 'org.manage');

-- Store admin user ID
SET @admin_user_id = (SELECT id FROM cluster.users WHERE email = 'admin@example.com');

-- Link admin to super_admin role
INSERT INTO cluster.role_user (user_id, role_id)
VALUES (@admin_user_id, @super_admin_role_id);

-- Insert Organizations
INSERT INTO cluster.orgs (name) VALUES
('Acme Corp'),
('TechStart Inc'),
('DevOps Masters'),
('Cloud Native Labs');

-- Pre-calculate org IDs
SET @first_org_id = (SELECT MIN(id) FROM cluster.orgs);
SET @last_org_id = (SELECT MAX(id) FROM cluster.orgs);

-- Insert Organization Members efficiently
INSERT INTO cluster.orgmember (org_id, user_id, role)
SELECT id, @admin_user_id, 'owner'
FROM cluster.orgs;

-- Insert Applications in bulk
INSERT INTO cluster.apps (name, org_id, git_repo, git_branch, container_image_url, region_id)
SELECT 
    CONCAT('app-', o.name, '-', numbers.n),
    o.id,
    CONCAT('https://github.com/org/', LOWER(REPLACE(o.name, ' ', '-')), '-app-', numbers.n),
    IF(numbers.n % 2 = 0, 'main', 'develop'),
    IF(numbers.n % 3 = 0, 'https://buildpack.example.com/node', 'https://buildpack.example.com/python'),
    1 + MOD(o.id + numbers.n, 5)  -- Distribute across regions deterministically
FROM cluster.orgs o
CROSS JOIN (
    SELECT 1 as n UNION ALL SELECT 2 UNION ALL SELECT 3
) numbers;

-- Insert Instances more efficiently
INSERT INTO cluster.instances (app_id, instance_type, status, container_id, pod_name, node_name)
SELECT 
    a.id,
    ELT(1 + MOD(a.id + n.num, 3), 'web', 'worker', 'scheduler'),
    'running',
    CONCAT('cont-', LPAD(HEX(a.id * 3 + n.num), 8, '0')),
    CONCAT('pod-', LPAD(HEX(a.id * 3 + n.num), 8, '0')),
    CONCAT('node-', 1 + MOD(a.id + n.num, 5))
FROM cluster.apps a
CROSS JOIN (
    SELECT 1 as num UNION ALL SELECT 2 UNION ALL SELECT 3
) n;

-- Insert Domains efficiently
INSERT INTO cluster.domains (app_id, name, ssl_enabled)
SELECT 
    id,
    CONCAT(LOWER(REPLACE(name, ' ', '-')), '.example.com'),
    1
FROM cluster.apps;

-- Insert Builds with deterministic values
INSERT INTO cluster.builds (app_id, source_version, status, started_at, completed_at)
SELECT 
    a.id,
    CONCAT('commit-', LPAD(HEX(a.id * 3 + b.num), 40, '0')),
    ELT(1 + MOD(a.id + b.num, 4), 'pending', 'building', 'succeeded', 'failed'),
    DATE_SUB(NOW(), INTERVAL (a.id + b.num) DAY),
    DATE_SUB(NOW(), INTERVAL (a.id + b.num - 1) DAY)
FROM cluster.apps a
CROSS JOIN (
    SELECT 1 as num UNION ALL SELECT 2 UNION ALL SELECT 3
) b;

-- Insert Deployments for successful builds only
INSERT INTO cluster.deployments (app_id, build_id, status, started_at, completed_at)
SELECT 
    app_id,
    id,
    ELT(1 + MOD(app_id, 5), 'pending', 'in_progress', 'succeeded', 'failed', 'rolled_back'),
    started_at,
    completed_at
FROM cluster.builds
WHERE status = 'succeeded';

-- Insert Config Vars efficiently
INSERT INTO cluster.config_vars (app_id, `key`, value, is_secret)
SELECT 
    a.id,
    CONCAT('ENV_VAR_', n.num),
    IF(n.num % 2 = 0, 'production', 'development'),
    n.num % 2
FROM cluster.apps a
CROSS JOIN (
    SELECT 1 as num UNION ALL SELECT 2 UNION ALL SELECT 3 UNION ALL SELECT 4 UNION ALL SELECT 5
) n;

-- Insert Metrics more efficiently (last 24 hours, sampled)
INSERT INTO cluster.metrics (instance_id, metric_name, metric_value, timestamp)
SELECT 
    i.id,
    m.name,
    CASE m.name
        WHEN 'cpu_usage' THEN 20 + MOD(i.id * h.hour, 80)
        WHEN 'memory_usage' THEN 200 + MOD(i.id * h.hour, 824)
        ELSE 100 + MOD(i.id * h.hour, 900)
    END,
    DATE_SUB(NOW(), INTERVAL h.hour HOUR)
FROM cluster.instances i
CROSS JOIN (
    SELECT 'cpu_usage' as name UNION ALL 
    SELECT 'memory_usage' UNION ALL 
    SELECT 'request_count'
) m
CROSS JOIN (
    SELECT hr as hour FROM (
        SELECT 0 hr UNION ALL SELECT 4 UNION ALL SELECT 8 UNION ALL 
        SELECT 12 UNION ALL SELECT 16 UNION ALL SELECT 20
    ) hours
) h
WHERE i.status = 'running';

-- Insert Instance Logs with prepared messages
INSERT INTO cluster.instance_logs (instance_id, log_type, message)
SELECT 
    i.id,
    ELT(1 + MOD(i.id + l.num, 3), 'app', 'system', 'deployment'),
    ELT(1 + MOD(i.id + l.num, 3), 
        'Application started successfully',
        'Health check passed',
        'Deployment completed')
FROM cluster.instances i
CROSS JOIN (
    SELECT 1 as num UNION ALL SELECT 2 UNION ALL SELECT 3
) l;

-- Insert API Keys efficiently
INSERT INTO cluster.api_keys (org_id, name, key_hash)
SELECT 
    id,
    'Production API Key',
    CONCAT('key-', LPAD(HEX(id), 32, '0'))
FROM cluster.orgs;

-- Insert Audit Logs efficiently
INSERT INTO cluster.audit_logs (user_id, org_id, action, resource_type, resource_id)
SELECT 
    u.id,
    o.id,
    ELT(1 + MOD(u.id * o.id + l.num, 4), 'created', 'updated', 'deleted', 'accessed'),
    ELT(1 + MOD(u.id + o.id + l.num, 3), 'app', 'deployment', 'config'),
    MOD(u.id * o.id * l.num, 1000)
FROM cluster.users u
CROSS JOIN cluster.orgs o
CROSS JOIN (
    SELECT 1 as num UNION ALL SELECT 2 UNION ALL SELECT 3
) l;

COMMIT;

-- Re-enable checks
SET FOREIGN_KEY_CHECKS = 1;
SET unique_checks = 1;
SET autocommit = 1;