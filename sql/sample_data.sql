-- Disable foreign key checks temporarily for faster inserts
SET FOREIGN_KEY_CHECKS = 0;

-- Insert Regions
INSERT INTO cluster.regions (name, provider, status) VALUES
('us-east-1', 'kubernetes', 'active'),
('us-west-1', 'kubernetes', 'active'),
('eu-west-1', 'kubernetes', 'active'),
('ap-south-1', 'kubernetes', 'maintenance'),
('custom-dc-1', 'custom', 'active');

-- Insert Users
INSERT INTO cluster.users (email, name, password, salt, active, last_login_at) VALUES
('admin@example.com', 'Admin User', '$2a$12$LQVkMqLYXqxhph', 'randomsalt123', 1, DATE_SUB(NOW(), INTERVAL 1 DAY)),
('john.doe@example.com', 'John Doe', '$2a$12$LQVkMqLYXqxhph', 'randomsalt124', 1, DATE_SUB(NOW(), INTERVAL 2 DAY)),
('jane.smith@example.com', 'Jane Smith', '$2a$12$LQVkMqLYXqxhph', 'randomsalt125', 1, DATE_SUB(NOW(), INTERVAL 3 DAY)),
('bob.wilson@example.com', 'Bob Wilson', '$2a$12$LQVkMqLYXqxhph', 'randomsalt126', 1, DATE_SUB(NOW(), INTERVAL 4 DAY)),
('alice.johnson@example.com', 'Alice Johnson', '$2a$12$LQVkMqLYXqxhph', 'randomsalt127', 0, NULL);

-- Insert Roles
INSERT INTO cluster.roles (name, description) VALUES
('super_admin', 'Full system access'),
('org_admin', 'Organization administrator'), 
('developer', 'Application developer'),
('viewer', 'Read-only access');

-- Insert Permissions
INSERT INTO cluster.permissions (name, description, resource_type) VALUES
('app.create', 'Create applications', 'app'),
('app.delete', 'Delete applications', 'app'),
('app.deploy', 'Deploy applications', 'app'),
('app.view', 'View applications', 'app'),
('org.manage', 'Manage organization settings', 'org'),
('user.manage', 'Manage users', 'user');

-- Link Permissions to Roles
INSERT INTO cluster.permissions_role (permissions_id, role_id) 
SELECT p.id, r.id 
FROM cluster.permissions p, cluster.roles r 
WHERE r.name = 'super_admin';

INSERT INTO cluster.permissions_role (permissions_id, role_id)
SELECT p.id, r.id
FROM cluster.permissions p, cluster.roles r
WHERE r.name = 'org_admin' 
AND p.name IN ('app.create', 'app.deploy', 'app.view', 'org.manage');

-- Link Users to Roles
INSERT INTO cluster.role_user (user_id, role_id)
SELECT u.id, r.id
FROM cluster.users u, cluster.roles r
WHERE u.email = 'admin@example.com' AND r.name = 'super_admin';

-- Insert Organizations
INSERT INTO cluster.orgs (name) VALUES
('Acme Corp'),
('TechStart Inc'),
('DevOps Masters'),
('Cloud Native Labs');

-- Insert Organization Members
INSERT INTO cluster.orgmember (org_id, user_id, role)
SELECT o.id, u.id, 'owner'
FROM cluster.orgs o, cluster.users u
WHERE u.email = 'admin@example.com';

INSERT INTO cluster.orgmember (org_id, user_id, role)
SELECT o.id, u.id, 'admin'
FROM cluster.orgs o, cluster.users u
WHERE u.email = 'john.doe@example.com' AND o.name = 'TechStart Inc';

-- Insert Applications
INSERT INTO cluster.apps (name, org_id, git_repo, git_branch, container_image_url, region_id) 
SELECT 
   CONCAT('app-', o.name, '-', seq.value),
   o.id,
   CONCAT('https://github.com/org/', LOWER(REPLACE(o.name, ' ', '-')), '-app-', seq.value),
   CASE WHEN seq.value % 2 = 0 THEN 'main' ELSE 'develop' END,
   CASE WHEN seq.value % 3 = 0 THEN 'https://buildpack.example.com/node' ELSE 'https://buildpack.example.com/python' END,
   (SELECT id FROM cluster.regions ORDER BY RAND() LIMIT 1)
FROM cluster.orgs o,
(WITH RECURSIVE sequence AS (
   SELECT 1 AS value
   UNION ALL
   SELECT value + 1 FROM sequence WHERE value < 3
)
SELECT value FROM sequence) seq;

-- Insert Instances
INSERT INTO cluster.instances (app_id, instance_type, status, container_id, pod_name, node_name)
SELECT 
   a.id,
   CASE FLOOR(RAND() * 3)
       WHEN 0 THEN 'web'
       WHEN 1 THEN 'worker'
       ELSE 'scheduler'
   END,
   'running',
   CONCAT('cont-', LOWER(HEX(RANDOM_BYTES(4)))),
   CONCAT('pod-', LOWER(HEX(RANDOM_BYTES(4)))),
   CONCAT('node-', FLOOR(RAND() * 5 + 1))
FROM cluster.apps a
CROSS JOIN (SELECT 1 AS instance_num UNION SELECT 2 UNION SELECT 3) i;

-- Insert Domains
INSERT INTO cluster.domains (app_id, name, ssl_enabled)
SELECT 
   id,
   CONCAT(LOWER(REPLACE(name, ' ', '-')), '.example.com'),
   1
FROM cluster.apps;

-- Insert Builds
INSERT INTO cluster.builds (app_id, source_version, status, started_at, completed_at)
SELECT 
   a.id,
   LOWER(HEX(RANDOM_BYTES(20))),
   CASE FLOOR(RAND() * 4)
       WHEN 0 THEN 'pending'
       WHEN 1 THEN 'building'
       WHEN 2 THEN 'succeeded'
       ELSE 'failed'
   END,
   DATE_SUB(NOW(), INTERVAL FLOOR(RAND() * 30) DAY),
   DATE_SUB(NOW(), INTERVAL FLOOR(RAND() * 30) DAY)
FROM cluster.apps a
CROSS JOIN (SELECT 1 AS build_num UNION SELECT 2 UNION SELECT 3) b;

-- Insert Deployments
INSERT INTO cluster.deployments (app_id, build_id, status, started_at, completed_at)
SELECT 
   b.app_id,
   b.id,
   CASE FLOOR(RAND() * 5)
       WHEN 0 THEN 'pending'
       WHEN 1 THEN 'in_progress'
       WHEN 2 THEN 'succeeded'
       WHEN 3 THEN 'failed'
       ELSE 'rolled_back'
   END,
   b.started_at,
   b.completed_at
FROM cluster.builds b
WHERE b.status = 'succeeded';

-- Insert Config Vars
INSERT INTO cluster.config_vars (app_id, `key`, value, is_secret)
SELECT 
   a.id,
   CONCAT('ENV_VAR_', seq.value),
   CASE 
       WHEN seq.value % 2 = 0 THEN 'production'
       ELSE 'development'
   END,
   seq.value % 2
FROM cluster.apps a
CROSS JOIN (
   WITH RECURSIVE sequence AS (
       SELECT 1 AS value
       UNION ALL
       SELECT value + 1 FROM sequence WHERE value < 5
   )
   SELECT value FROM sequence
) seq;

-- Insert Metrics (last 24 hours of data)
INSERT INTO cluster.metrics (instance_id, metric_name, metric_value, timestamp)
SELECT 
    i.id,
    m.metric_name,
    CASE m.metric_name
        WHEN 'cpu_usage' THEN FLOOR(RAND() * 100)
        WHEN 'memory_usage' THEN FLOOR(RAND() * 1024)
        WHEN 'request_count' THEN FLOOR(RAND() * 1000)
    END,
    DATE_SUB(DATE_SUB(NOW(), INTERVAL seq.hour HOUR), INTERVAL FLOOR(RAND() * 60) MINUTE)
FROM cluster.instances i
CROSS JOIN (
    SELECT 'cpu_usage' AS metric_name
    UNION SELECT 'memory_usage'
    UNION SELECT 'request_count'
) m
CROSS JOIN (
    WITH RECURSIVE sequence AS (
        SELECT 0 AS hour
        UNION ALL
        SELECT hour + 1 FROM sequence WHERE hour < 24
    )
    SELECT hour FROM sequence
) seq
WHERE i.status = 'running';

-- Insert Instance Logs
INSERT INTO cluster.instance_logs (instance_id, log_type, message)
SELECT 
   i.id,
   CASE FLOOR(RAND() * 3)
       WHEN 0 THEN 'app'
       WHEN 1 THEN 'system'
       ELSE 'deployment'
   END,
   CASE FLOOR(RAND() * 3)
       WHEN 0 THEN 'Application started successfully'
       WHEN 1 THEN 'Health check passed'
       ELSE 'Deployment completed'
   END
FROM cluster.instances i
CROSS JOIN (SELECT 1 UNION SELECT 2 UNION SELECT 3) l;

-- Insert API Keys
INSERT INTO cluster.api_keys (org_id, name, key_hash)
SELECT 
   id,
   'Production API Key',
   LOWER(HEX(RANDOM_BYTES(32)))
FROM cluster.orgs;

-- Insert Audit Logs
INSERT INTO cluster.audit_logs (user_id, org_id, action, resource_type, resource_id)
SELECT 
   u.id,
   o.id,
   CASE FLOOR(RAND() * 4)
       WHEN 0 THEN 'created'
       WHEN 1 THEN 'updated'
       WHEN 2 THEN 'deleted'
       ELSE 'accessed'
   END,
   CASE FLOOR(RAND() * 3)
       WHEN 0 THEN 'app'
       WHEN 1 THEN 'deployment'
       ELSE 'config'
   END,
   FLOOR(RAND() * 1000)
FROM cluster.users u
CROSS JOIN cluster.orgs o
CROSS JOIN (SELECT 1 UNION SELECT 2 UNION SELECT 3) l;

-- Re-enable foreign key checks
SET FOREIGN_KEY_CHECKS = 1;