-- Disable foreign key checks and turn off autocommit for faster inserts
SET FOREIGN_KEY_CHECKS = 0;
SET autocommit = 0;
SET unique_checks = 0;

START TRANSACTION;

-- Insert Regions (pre-computed values)
INSERT INTO cluster.regions (name, provider, status) VALUES
('us-east-1',   'kubernetes', 'active'),
('us-west-1',   'kubernetes', 'active'),
('eu-west-1',   'kubernetes', 'active'),
('ap-south-1',  'kubernetes', 'maintenance'),
('custom-dc-1', 'custom',     'active');

-- Insert Users (pre-computed values)
INSERT INTO cluster.users (email, name, password, salt, active, last_login_at) VALUES
('admin@example.com',         'Admin User',    '$2a$12$LQVkMqLYXqxhph', 'randomsalt123', 1, DATE_SUB(NOW(), INTERVAL 1 DAY)),
('john.doe@example.com',      'John Doe',      '$2a$12$LQVkMqLYXqxhph', 'randomsalt124', 1, DATE_SUB(NOW(), INTERVAL 2 DAY)),
('jane.smith@example.com',    'Jane Smith',    '$2a$12$LQVkMqLYXqxhph', 'randomsalt125', 1, DATE_SUB(NOW(), INTERVAL 3 DAY)),
('bob.wilson@example.com',    'Bob Wilson',    '$2a$12$LQVkMqLYXqxhph', 'randomsalt126', 1, DATE_SUB(NOW(), INTERVAL 4 DAY)),
('alice.johnson@example.com', 'Alice Johnson', '$2a$12$LQVkMqLYXqxhph', 'randomsalt127', 0, DATE_SUB(NOW(), INTERVAL 4 DAY)),

-- Insert more test users

('user1@example.com',  'User One',          '$2a$12$LQVkMqLYXqxhph', 'salt128', 1, DATE_SUB(NOW(), INTERVAL 5 DAY)),
('user2@example.com',  'User Two',          '$2a$12$LQVkMqLYXqxhph', 'salt129', 1, DATE_SUB(NOW(), INTERVAL 6 DAY)),
('user3@example.com',  'User Three',        '$2a$12$LQVkMqLYXqxhph', 'salt130', 1, DATE_SUB(NOW(), INTERVAL 7 DAY)),
('user4@example.com',  'User Four',         '$2a$12$LQVkMqLYXqxhph', 'salt131', 1, DATE_SUB(NOW(), INTERVAL 8 DAY)),
('user5@example.com',  'User Five',         '$2a$12$LQVkMqLYXqxhph', 'salt132', 1, DATE_SUB(NOW(), INTERVAL 9 DAY)),
('user6@example.com',  'User Six',          '$2a$12$LQVkMqLYXqxhph', 'salt133', 1, DATE_SUB(NOW(), INTERVAL 10 DAY)),
('user7@example.com',  'User Seven',        '$2a$12$LQVkMqLYXqxhph', 'salt134', 1, DATE_SUB(NOW(), INTERVAL 11 DAY)),
('user8@example.com',  'User Eight',        '$2a$12$LQVkMqLYXqxhph', 'salt135', 1, DATE_SUB(NOW(), INTERVAL 12 DAY)),
('user9@example.com',  'User Nine',         '$2a$12$LQVkMqLYXqxhph', 'salt136', 1, DATE_SUB(NOW(), INTERVAL 13 DAY)),
('user10@example.com', 'User Ten',          '$2a$12$LQVkMqLYXqxhph', 'salt137', 1, DATE_SUB(NOW(), INTERVAL 14 DAY)),
('user11@example.com', 'User Eleven',       '$2a$12$LQVkMqLYXqxhph', 'salt138', 1, DATE_SUB(NOW(), INTERVAL 15 DAY)),
('user12@example.com', 'User Twelve',       '$2a$12$LQVkMqLYXqxhph', 'salt139', 1, DATE_SUB(NOW(), INTERVAL 16 DAY)),
('user13@example.com', 'User Thirteen',     '$2a$12$LQVkMqLYXqxhph', 'salt140', 1, DATE_SUB(NOW(), INTERVAL 17 DAY)),
('user14@example.com', 'User Fourteen',     '$2a$12$LQVkMqLYXqxhph', 'salt141', 1, DATE_SUB(NOW(), INTERVAL 18 DAY)),
('user15@example.com', 'User Fifteen',      '$2a$12$LQVkMqLYXqxhph', 'salt142', 1, DATE_SUB(NOW(), INTERVAL 19 DAY)),
('user16@example.com', 'User Sixteen',      '$2a$12$LQVkMqLYXqxhph', 'salt143', 1, DATE_SUB(NOW(), INTERVAL 20 DAY)),
('user17@example.com', 'User Seventeen',    '$2a$12$LQVkMqLYXqxhph', 'salt144', 1, DATE_SUB(NOW(), INTERVAL 21 DAY)),
('user18@example.com', 'User Eighteen',     '$2a$12$LQVkMqLYXqxhph', 'salt145', 1, DATE_SUB(NOW(), INTERVAL 22 DAY)),
('user19@example.com', 'User Nineteen',     '$2a$12$LQVkMqLYXqxhph', 'salt146', 1, DATE_SUB(NOW(), INTERVAL 23 DAY)),
('user20@example.com', 'User Twenty',       '$2a$12$LQVkMqLYXqxhph', 'salt147', 1, DATE_SUB(NOW(), INTERVAL 24 DAY)),
('user21@example.com', 'User Twenty One',   '$2a$12$LQVkMqLYXqxhph', 'salt148', 1, DATE_SUB(NOW(), INTERVAL 25 DAY)),
('user22@example.com', 'User Twenty Two',   '$2a$12$LQVkMqLYXqxhph', 'salt149', 1, DATE_SUB(NOW(), INTERVAL 26 DAY)),
('user23@example.com', 'User Twenty Three', '$2a$12$LQVkMqLYXqxhph', 'salt150', 1, DATE_SUB(NOW(), INTERVAL 27 DAY)),
('user24@example.com', 'User Twenty Four',  '$2a$12$LQVkMqLYXqxhph', 'salt151', 1, DATE_SUB(NOW(), INTERVAL 28 DAY)),
('user25@example.com', 'User Twenty Five',  '$2a$12$LQVkMqLYXqxhph', 'salt152', 1, DATE_SUB(NOW(), INTERVAL 29 DAY)),
('user26@example.com', 'User Twenty Six',   '$2a$12$LQVkMqLYXqxhph', 'salt153', 1, DATE_SUB(NOW(), INTERVAL 30 DAY)),
('user27@example.com', 'User Twenty Seven', '$2a$12$LQVkMqLYXqxhph', 'salt154', 1, DATE_SUB(NOW(), INTERVAL 31 DAY)),
('user28@example.com', 'User Twenty Eight', '$2a$12$LQVkMqLYXqxhph', 'salt155', 1, DATE_SUB(NOW(), INTERVAL 32 DAY)),
('user29@example.com', 'User Twenty Nine',  '$2a$12$LQVkMqLYXqxhph', 'salt156', 1, DATE_SUB(NOW(), INTERVAL 33 DAY)),
('user30@example.com', 'User Thirty',       '$2a$12$LQVkMqLYXqxhph', 'salt157', 1, DATE_SUB(NOW(), INTERVAL 34 DAY)),
('user31@example.com', 'User Thirty One',   '$2a$12$LQVkMqLYXqxhph', 'salt158', 1, DATE_SUB(NOW(), INTERVAL 35 DAY)),
('user32@example.com', 'User Thirty Two',   '$2a$12$LQVkMqLYXqxhph', 'salt159', 1, DATE_SUB(NOW(), INTERVAL 36 DAY)),
('user33@example.com', 'User Thirty Three', '$2a$12$LQVkMqLYXqxhph', 'salt160', 1, DATE_SUB(NOW(), INTERVAL 37 DAY)),
('user34@example.com', 'User Thirty Four',  '$2a$12$LQVkMqLYXqxhph', 'salt161', 1, DATE_SUB(NOW(), INTERVAL 38 DAY)),
('user35@example.com', 'User Thirty Five',  '$2a$12$LQVkMqLYXqxhph', 'salt162', 1, DATE_SUB(NOW(), INTERVAL 39 DAY)),
('user36@example.com', 'User Thirty Six',   '$2a$12$LQVkMqLYXqxhph', 'salt163', 1, DATE_SUB(NOW(), INTERVAL 40 DAY)),
('user37@example.com', 'User Thirty Seven', '$2a$12$LQVkMqLYXqxhph', 'salt164', 1, DATE_SUB(NOW(), INTERVAL 41 DAY)),
('user38@example.com', 'User Thirty Eight', '$2a$12$LQVkMqLYXqxhph', 'salt165', 1, DATE_SUB(NOW(), INTERVAL 42 DAY)),
('user39@example.com', 'User Thirty Nine',  '$2a$12$LQVkMqLYXqxhph', 'salt166', 1, DATE_SUB(NOW(), INTERVAL 43 DAY)),
('user40@example.com', 'User Forty',        '$2a$12$LQVkMqLYXqxhph', 'salt167', 1, DATE_SUB(NOW(), INTERVAL 44 DAY)),
('user41@example.com', 'User Forty One',    '$2a$12$LQVkMqLYXqxhph', 'salt168', 1, DATE_SUB(NOW(), INTERVAL 45 DAY)),
('user42@example.com', 'User Forty Two',    '$2a$12$LQVkMqLYXqxhph', 'salt169', 1, DATE_SUB(NOW(), INTERVAL 46 DAY)),
('user43@example.com', 'User Forty Three',  '$2a$12$LQVkMqLYXqxhph', 'salt170', 1, DATE_SUB(NOW(), INTERVAL 47 DAY)),
('user44@example.com', 'User Forty Four',   '$2a$12$LQVkMqLYXqxhph', 'salt171', 1, DATE_SUB(NOW(), INTERVAL 48 DAY)),
('user45@example.com', 'User Forty Five',   '$2a$12$LQVkMqLYXqxhph', 'salt172', 1, DATE_SUB(NOW(), INTERVAL 49 DAY)),
('user46@example.com', 'User Forty Six',    '$2a$12$LQVkMqLYXqxhph', 'salt173', 1, DATE_SUB(NOW(), INTERVAL 50 DAY)),
('user47@example.com', 'User Forty Seven',  '$2a$12$LQVkMqLYXqxhph', 'salt174', 1, DATE_SUB(NOW(), INTERVAL 51 DAY)),
('user48@example.com', 'User Forty Eight',  '$2a$12$LQVkMqLYXqxhph', 'salt175', 1, DATE_SUB(NOW(), INTERVAL 52 DAY)),
('user49@example.com', 'User Forty Nine',   '$2a$12$LQVkMqLYXqxhph', 'salt176', 1, DATE_SUB(NOW(), INTERVAL 53 DAY)),
('user50@example.com', 'User Fifty',        '$2a$12$LQVkMqLYXqxhph', 'salt177', 1, DATE_SUB(NOW(), INTERVAL 54 DAY));

-- Insert Roles and store IDs in variables
INSERT INTO cluster.roles (name, description) VALUES
('super_admin', 'Full system access'),
('org_admin',   'Organization administrator'),
('developer',   'Application developer'),
('viewer',      'Read-only access');

SET @super_admin_role_id = (SELECT id FROM cluster.roles WHERE name = 'super_admin');
SET @org_admin_role_id = (SELECT id FROM cluster.roles WHERE name = 'org_admin');

-- Insert Permissions with batch insert
INSERT INTO cluster.permissions (name, description, resource_type) VALUES
('org.manage',  'Manage organization settings', 'org'),
('app.create',  'Create applications',          'app'),
('app.delete',  'Delete applications',          'app'),
('app.deploy',  'Deploy applications',          'app'),
('app.view',    'View applications',            'app'),
('user.manage', 'Manage users',                 'user');

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

-- -- Insert Instances more efficiently
-- INSERT INTO cluster.instances (app_id, instance_type, status, container_id, node_name)
-- SELECT 
--     a.id,
--     ELT(1 + MOD(a.id + n.num, 3), 'web', 'worker', 'scheduler'),
--     'running',
--     CONCAT('cont-', LPAD(HEX(a.id * 3 + n.num), 8, '0')),
--     CONCAT('pod-', LPAD(HEX(a.id * 3 + n.num), 8, '0')),
--     CONCAT('node-', 1 + MOD(a.id + n.num, 5))
-- FROM cluster.apps a
-- CROSS JOIN (
--     SELECT 1 as num UNION ALL SELECT 2 UNION ALL SELECT 3
-- ) n;

-- Insert Domains efficiently
INSERT INTO cluster.domains (app_id, name, ssl_enabled)
SELECT 
    id,
    CONCAT(LOWER(REPLACE(name, ' ', '-')), '.example.com'),
    1
FROM cluster.apps;

-- -- Insert Builds with deterministic values
-- INSERT INTO cluster.builds (app_id, source_version, status, started_at, completed_at)
-- SELECT 
--     a.id,
--     CONCAT('commit-', LPAD(HEX(a.id * 3 + b.num), 40, '0')),
--     ELT(1 + MOD(a.id + b.num, 4), 'pending', 'building', 'succeeded', 'failed'),
--     DATE_SUB(NOW(), INTERVAL (a.id + b.num) DAY),
--     DATE_SUB(NOW(), INTERVAL (a.id + b.num - 1) DAY)
-- FROM cluster.apps a
-- CROSS JOIN (
--     SELECT 1 as num UNION ALL SELECT 2 UNION ALL SELECT 3
-- ) b;

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

-- Insert Metrics more efficiently (1 week of data, higher frequency)
INSERT INTO cluster.metrics (instance_id, metric_name, metric_value, timestamp)
SELECT 
    i.id,
    m.name,
    CASE m.name
        WHEN 'cpu_usage' THEN 20 + MOD(i.id * MINUTE(DATE_SUB(NOW(), INTERVAL h.minute MINUTE)), 80)
        WHEN 'memory_usage' THEN 200 + MOD(i.id * MINUTE(DATE_SUB(NOW(), INTERVAL h.minute MINUTE)), 824)
        ELSE 100 + MOD(i.id * MINUTE(DATE_SUB(NOW(), INTERVAL h.minute MINUTE)), 900)
    END,
    DATE_SUB(NOW(), INTERVAL h.minute MINUTE)
FROM cluster.instances i
CROSS JOIN (
    SELECT 'cpu_usage' as name UNION ALL 
    SELECT 'memory_usage' UNION ALL 
    SELECT 'request_count'
) m
CROSS JOIN (
    SELECT a.a + b.a * 60 + c.a * 1440 as minute FROM
    (SELECT 0 as a UNION ALL SELECT 1 UNION ALL SELECT 2 UNION ALL SELECT 3) a,
    (SELECT 0 as a UNION ALL SELECT 1 UNION ALL SELECT 2 UNION ALL SELECT 3) b,
    (SELECT 0 as a UNION ALL SELECT 1 UNION ALL SELECT 2 UNION ALL SELECT 3 UNION ALL SELECT 4 UNION ALL SELECT 5 UNION ALL SELECT 6) c
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