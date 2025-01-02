-- Disable foreign key checks temporarily for faster inserts
PRAGMA foreign_keys = OFF;

-- Insert Regions
INSERT INTO regions (name, provider, status) VALUES
('us-east-1', 'kubernetes', 'active'),
('us-west-1', 'kubernetes', 'active'),
('eu-west-1', 'kubernetes', 'active'),
('ap-south-1', 'kubernetes', 'maintenance'),
('custom-dc-1', 'custom', 'active');

-- Insert Users
INSERT INTO users (email, name, password, active, last_login_at) VALUES
('admin@example.com', 'Admin User', '$2a$12$LQVkMqLYXqxhph', 1, DATETIME('now', '-1 day')),
('john.doe@example.com', 'John Doe', '$2a$12$LQVkMqLYXqxhph', 1, DATETIME('now', '-2 days')),
('jane.smith@example.com', 'Jane Smith', '$2a$12$LQVkMqLYXqxhph', 1, DATETIME('now', '-3 days')),
('bob.wilson@example.com', 'Bob Wilson', '$2a$12$LQVkMqLYXqxhph', 1, DATETIME('now', '-4 days')),
('alice.johnson@example.com', 'Alice Johnson', '$2a$12$LQVkMqLYXqxhph', 0, NULL);

-- Insert Roles
INSERT INTO roles (name, description) VALUES
('super_admin', 'Full system access'),
('org_admin', 'Organization administrator'),
('developer', 'Application developer'),
('viewer', 'Read-only access');

-- Insert Permissions
INSERT INTO permissions (name, description, resource_type) VALUES
('app.create', 'Create applications', 'app'),
('app.delete', 'Delete applications', 'app'),
('app.deploy', 'Deploy applications', 'app'),
('app.view', 'View applications', 'app'),
('org.manage', 'Manage organization settings', 'org'),
('user.manage', 'Manage users', 'user');

-- Link Permissions to Roles
INSERT INTO permissions_role (permissions_id, role_id) 
SELECT p.id, r.id 
FROM permissions p, roles r 
WHERE r.name = 'super_admin';

INSERT INTO permissions_role (permissions_id, role_id)
SELECT p.id, r.id
FROM permissions p, roles r
WHERE r.name = 'org_admin' 
AND p.name IN ('app.create', 'app.deploy', 'app.view', 'org.manage');

-- Link Users to Roles
INSERT INTO role_user (user_id, role_id)
SELECT u.id, r.id
FROM users u, roles r
WHERE u.email = 'admin@example.com' AND r.name = 'super_admin';

-- Insert Organizations
INSERT INTO orgs (name) VALUES
('Acme Corp'),
('TechStart Inc'),
('DevOps Masters'),
('Cloud Native Labs');

-- Insert Organization Members
INSERT INTO orgmember (org_id, user_id, role)
SELECT o.id, u.id, 'owner'
FROM orgs o, users u
WHERE u.email = 'admin@example.com';

INSERT INTO orgmember (org_id, user_id, role)
SELECT o.id, u.id, 'admin'
FROM orgs o, users u
WHERE u.email = 'john.doe@example.com' AND o.name = 'TechStart Inc';

-- Insert Applications
INSERT INTO apps (name, org_id, git_repo, git_branch, buildpack_url, region_id) 
SELECT 
    'app-' || o.name || '-' || seq.value,
    o.id,
    'https://github.com/org/' || LOWER(REPLACE(o.name, ' ', '-')) || '-app-' || seq.value,
    CASE WHEN seq.value % 2 = 0 THEN 'main' ELSE 'develop' END,
    CASE WHEN seq.value % 3 = 0 THEN 'https://buildpack.example.com/node' ELSE 'https://buildpack.example.com/python' END,
    (SELECT id FROM regions ORDER BY RANDOM() LIMIT 1)
FROM orgs o, 
(WITH RECURSIVE sequence(value) AS (
    SELECT 1
    UNION ALL
    SELECT value + 1 FROM sequence WHERE value < 3
)
SELECT value FROM sequence) seq;

-- Insert Instances
INSERT INTO instances (app_id, instance_type, status, container_id, pod_name, node_name)
SELECT 
    a.id,
    CASE (ABS(RANDOM()) % 3)
        WHEN 0 THEN 'web'
        WHEN 1 THEN 'worker'
        ELSE 'scheduler'
    END,
    'running',
    'cont-' || LOWER(HEX(RANDOMBLOB(4))),
    'pod-' || LOWER(HEX(RANDOMBLOB(4))),
    'node-' || (ABS(RANDOM()) % 5 + 1)
FROM apps a
CROSS JOIN (SELECT 1 AS instance_num UNION SELECT 2 UNION SELECT 3) i;

-- Insert Domains
INSERT INTO domains (app_id, name, ssl_enabled)
SELECT 
    id,
    LOWER(REPLACE(name, ' ', '-')) || '.example.com',
    1
FROM apps;

-- Insert Builds
INSERT INTO builds (app_id, source_version, status, started_at, completed_at)
SELECT 
    a.id,
    LOWER(HEX(RANDOMBLOB(20))),
    CASE (ABS(RANDOM()) % 4)
        WHEN 0 THEN 'pending'
        WHEN 1 THEN 'building'
        WHEN 2 THEN 'succeeded'
        ELSE 'failed'
    END,
    DATETIME('now', '-' || (ABS(RANDOM()) % 30) || ' days'),
    DATETIME('now', '-' || (ABS(RANDOM()) % 30) || ' days')
FROM apps a
CROSS JOIN (SELECT 1 AS build_num UNION SELECT 2 UNION SELECT 3) b;

-- Insert Deployments
INSERT INTO deployments (app_id, build_id, status, started_at, completed_at)
SELECT 
    b.app_id,
    b.id,
    CASE (ABS(RANDOM()) % 5)
        WHEN 0 THEN 'pending'
        WHEN 1 THEN 'in_progress'
        WHEN 2 THEN 'succeeded'
        WHEN 3 THEN 'failed'
        ELSE 'rolled_back'
    END,
    b.started_at,
    b.completed_at
FROM builds b
WHERE b.status = 'succeeded';

-- Insert Config Vars
INSERT INTO config_vars (app_id, key, value, is_secret)
SELECT 
    a.id,
    'ENV_VAR_' || seq.value,
    CASE 
        WHEN seq.value % 2 = 0 THEN 'production'
        ELSE 'development'
    END,
    seq.value % 2
FROM apps a
CROSS JOIN (
    WITH RECURSIVE sequence(value) AS (
        SELECT 1
        UNION ALL
        SELECT value + 1 FROM sequence WHERE value < 5
    )
    SELECT value FROM sequence
) seq;

-- Insert Metrics (last 24 hours of data)
INSERT INTO metrics (instance_id, metric_name, metric_value, timestamp)
SELECT 
    i.id,
    m.metric_name,
    CASE m.metric_name
        WHEN 'cpu_usage' THEN ABS(RANDOM() % 100)
        WHEN 'memory_usage' THEN ABS(RANDOM() % 1024)
        WHEN 'request_count' THEN ABS(RANDOM() % 1000)
    END,
    DATETIME('now', '-' || (seq.hour) || ' hours', '-' || (ABS(RANDOM()) % 60) || ' minutes')
FROM instances i
CROSS JOIN (
    SELECT 'cpu_usage' AS metric_name
    UNION SELECT 'memory_usage'
    UNION SELECT 'request_count'
) m
CROSS JOIN (
    WITH RECURSIVE sequence(hour) AS (
        SELECT 0
        UNION ALL
        SELECT hour + 1 FROM sequence WHERE hour < 24
    )
    SELECT hour FROM sequence
) seq
WHERE i.status = 'running';

-- Insert Instance Logs
INSERT INTO instance_logs (instance_id, log_type, message)
SELECT 
    i.id,
    CASE (ABS(RANDOM()) % 3)
        WHEN 0 THEN 'app'
        WHEN 1 THEN 'system'
        ELSE 'deployment'
    END,
    CASE (ABS(RANDOM()) % 3)
        WHEN 0 THEN 'Application started successfully'
        WHEN 1 THEN 'Health check passed'
        ELSE 'Deployment completed'
    END
FROM instances i
CROSS JOIN (SELECT 1 UNION SELECT 2 UNION SELECT 3) l;

-- Insert API Keys
INSERT INTO api_keys (org_id, name, key_hash)
SELECT 
    id,
    'Production API Key',
    LOWER(HEX(RANDOMBLOB(32)))
FROM orgs;

-- Insert Audit Logs
INSERT INTO audit_logs (user_id, org_id, action, resource_type, resource_id)
SELECT 
    u.id,
    o.id,
    CASE (ABS(RANDOM()) % 4)
        WHEN 0 THEN 'created'
        WHEN 1 THEN 'updated'
        WHEN 2 THEN 'deleted'
        ELSE 'accessed'
    END,
    CASE (ABS(RANDOM()) % 3)
        WHEN 0 THEN 'app'
        WHEN 1 THEN 'deployment'
        ELSE 'config'
    END,
    ABS(RANDOM()) % 1000
FROM users u
CROSS JOIN orgs o
CROSS JOIN (SELECT 1 UNION SELECT 2 UNION SELECT 3) l;

-- Re-enable foreign key checks
PRAGMA foreign_keys = ON;