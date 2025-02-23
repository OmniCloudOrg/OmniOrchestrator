-- Create base numbers table first with all values
CREATE TABLE IF NOT EXISTS numbers AS 
SELECT 1 as n UNION ALL 
SELECT 2 UNION ALL 
SELECT 3 UNION ALL 
SELECT 4 UNION ALL 
SELECT 5 UNION ALL 
SELECT 6 UNION ALL 
SELECT 7 UNION ALL 
SELECT 8 UNION ALL 
SELECT 9 UNION ALL 
SELECT 10;

-- Create expanded numbers table without aliases
CREATE TABLE IF NOT EXISTS numbers_100 AS 
SELECT n1.n + (10 * (n2.n - 1)) as n
FROM numbers n1, numbers n2;

-- Create larger numbers table
CREATE TABLE IF NOT EXISTS numbers_1000 AS 
SELECT n1.n + (100 * (n2.n - 1)) as n
FROM numbers_100 n1, numbers n2;

-- Common app names and tech stacks for realistic data
CREATE TABLE IF NOT EXISTS app_prefixes AS
SELECT 'web' as prefix UNION ALL 
SELECT 'api' UNION ALL 
SELECT 'worker' UNION ALL 
SELECT 'batch' UNION ALL 
SELECT 'cron' UNION ALL 
SELECT 'auth' UNION ALL 
SELECT 'payment' UNION ALL 
SELECT 'search' UNION ALL 
SELECT 'admin' UNION ALL 
SELECT 'mobile';

CREATE TABLE IF NOT EXISTS tech_stacks AS
SELECT 'nodejs' as stack, 'node:16-alpine' as image UNION ALL 
SELECT 'python', 'python:3.9-slim' UNION ALL 
SELECT 'ruby', 'ruby:3.1-alpine' UNION ALL 
SELECT 'java', 'openjdk:17-slim' UNION ALL 
SELECT 'golang', 'golang:1.18-alpine';

-- Regions with realistic cloud provider names
INSERT INTO regions (name, provider, status) VALUES 
('us-east-1', 'kubernetes', 'active'),
('us-west-2', 'kubernetes', 'active'),
('eu-west-1', 'kubernetes', 'active'),
('ap-southeast-1', 'kubernetes', 'active'),
('ca-central-1', 'kubernetes', 'maintenance'),
('us-central1', 'docker', 'active'),
('europe-west4', 'docker', 'active'),
('asia-east1', 'docker', 'active'),
('au-southeast1', 'docker', 'maintenance'),
('custom-dc-1', 'custom', 'active');

-- Users with realistic patterns and varied activity
INSERT INTO users (email, name, password, salt, active, last_login_at)
SELECT 
    CASE 
        WHEN n <= 50 THEN CONCAT('admin', n, '@company', (n % 20) + 1, '.com')
        WHEN n <= 200 THEN CONCAT('dev', n, '@company', (n % 20) + 1, '.com')
        ELSE CONCAT('user', n, '@company', (n % 20) + 1, '.com')
    END,
    CASE 
        WHEN n <= 50 THEN CONCAT('Admin User ', n)
        WHEN n <= 200 THEN CONCAT('Developer ', n)
        ELSE CONCAT('Standard User ', n)
    END,
    SHA2(CONCAT('secure_pwd_', n), 256),
    UUID(),
    CASE WHEN n % 10 = 0 THEN 0 ELSE 1 END,
    CASE 
        WHEN n % 10 = 0 THEN NULL
        ELSE DATE_SUB(NOW(), INTERVAL (n % 72) HOUR)
    END
FROM numbers_1000
LIMIT 1000;

-- Roles with comprehensive permissions
INSERT INTO roles (name, description) VALUES 
('SuperAdmin', 'Full system access and management capabilities'),
('OrgAdmin', 'Organization-wide administration and oversight'),
('Developer', 'Application development and deployment access'),
('DevOps', 'Infrastructure and operations management'),
('Security', 'Security monitoring and audit access'),
('Support', 'Customer support and basic troubleshooting'),
('Analyst', 'Metrics and logs viewer'),
('ReadOnly', 'Read-only access to non-sensitive resources');

-- Comprehensive set of permissions
INSERT INTO permissions (name, description, resource_type) VALUES 
('app.create', 'Create new applications', 'app'),
('app.delete', 'Delete applications', 'app'),
('app.update', 'Update application settings', 'app'),
('app.deploy', 'Deploy applications', 'app'),
('app.restart', 'Restart applications', 'app'),
('app.scale', 'Scale application instances', 'app'),
('app.logs.view', 'View application logs', 'logs'),
('app.metrics.view', 'View application metrics', 'metrics'),
('config.manage', 'Manage configuration variables', 'config'),
('domain.manage', 'Manage custom domains', 'domain'),
('build.manage', 'Manage application builds', 'build'),
('instance.manage', 'Manage application instances', 'instance'),
('org.manage', 'Manage organization settings', 'org'),
('user.manage', 'Manage user access', 'user'),
('apikey.manage', 'Manage API keys', 'apikey'),
('billing.view', 'View billing information', 'billing'),
('billing.manage', 'Manage billing settings', 'billing'),
('audit.view', 'View audit logs', 'audit');

-- Permission-role relationships with realistic access patterns
INSERT INTO permissions_role (permissions_id, role_id)
SELECT 
    p.id,
    r.id
FROM permissions p
CROSS JOIN roles r
WHERE 
    (r.name = 'SuperAdmin') OR
    (r.name = 'OrgAdmin' AND p.name NOT LIKE 'system%') OR
    (r.name = 'Developer' AND p.name IN ('app.create', 'app.deploy', 'app.restart', 'app.logs.view', 'app.metrics.view', 'config.manage', 'domain.manage', 'build.manage')) OR
    (r.name = 'DevOps' AND p.name IN ('app.scale', 'instance.manage', 'app.logs.view', 'app.metrics.view', 'config.manage')) OR
    (r.name = 'Security' AND p.name LIKE '%.view') OR
    (r.name = 'Support' AND p.name LIKE '%.view') OR
    (r.name = 'Analyst' AND p.name IN ('app.metrics.view', 'app.logs.view', 'audit.view')) OR
    (r.name = 'ReadOnly' AND p.name LIKE '%.view');

-- Organizations with realistic naming and structure
INSERT INTO orgs (name)
SELECT 
    CASE 
        WHEN n <= 20 THEN CONCAT('Enterprise-', n)
        WHEN n <= 50 THEN CONCAT('Startup-', n)
        ELSE CONCAT('Team-', n)
    END
FROM numbers_100
LIMIT 100;

-- Role assignments to users
INSERT INTO role_user (user_id, role_id)
SELECT 
    u.id,
    r.id
FROM users u
CROSS JOIN roles r
WHERE 
    (u.id <= 50 AND r.name IN ('SuperAdmin', 'OrgAdmin')) OR
    (u.id <= 200 AND r.name IN ('Developer', 'DevOps')) OR
    (u.id > 200 AND r.name IN ('ReadOnly', 'Support', 'Analyst'))
LIMIT 2000;

-- Organization members with realistic role distribution
INSERT INTO orgmember (org_id, user_id, role)
SELECT DISTINCT
    o.id,
    u.id,
    CASE 
        WHEN u.id <= 50 THEN 'owner'
        WHEN u.id <= 200 THEN 'admin'
        ELSE 'member'
    END
FROM orgs o
CROSS JOIN users u
WHERE (o.id + u.id) % 5 = 0
LIMIT 2000;

-- API keys with realistic naming and distribution
INSERT INTO api_keys (org_id, name, key_hash)
SELECT 
    o.id,
    CASE 
        WHEN n = 1 THEN 'Production Deploy Key'
        WHEN n = 2 THEN 'Staging Deploy Key'
        WHEN n = 3 THEN 'CI/CD Integration'
        WHEN n = 4 THEN 'Monitoring Integration'
        ELSE CONCAT('Integration Key ', n)
    END,
    SHA2(UUID(), 256)
FROM orgs o
CROSS JOIN (SELECT n FROM numbers WHERE n <= 5) numbers;

-- Apps with realistic naming and configuration
INSERT INTO apps (name, org_id, git_repo, git_branch, container_image_url, region_id, maintenance_mode)
SELECT 
    CONCAT(p.prefix, '-', 
        CASE 
            WHEN o.id <= 20 THEN 'prod'
            WHEN o.id <= 50 THEN 'staging'
            ELSE 'dev'
        END,
        '-', t.stack, '-', numbers.n
    ),
    o.id,
    CONCAT('github.com/org', o.id, '/', p.prefix, '-', t.stack),
    CASE 
        WHEN o.id <= 20 THEN 'main'
        WHEN o.id <= 50 THEN 'staging'
        ELSE 'develop'
    END,
    CONCAT('registry.example.com/', t.image),
    1 + MOD(o.id + numbers.n, 8),
    CASE WHEN numbers.n % 20 = 0 THEN 1 ELSE 0 END
FROM orgs o
CROSS JOIN (SELECT n FROM numbers WHERE n <= 5) numbers
CROSS JOIN app_prefixes p
CROSS JOIN tech_stacks t
WHERE (o.id + numbers.n) % 3 = 0
LIMIT 1000;

-- Instances with realistic scaling patterns
INSERT INTO instances (app_id, instance_type, status, container_id, node_name, instance_status)
SELECT 
    a.id,
    CASE 
        WHEN a.name LIKE '%prod%' THEN 
            CASE MOD(numbers.n, 3)
                WHEN 0 THEN 't3.medium'
                WHEN 1 THEN 't3.large'
                ELSE 't3.xlarge'
            END
        ELSE 't3.small'
    END,
    CASE 
        WHEN numbers.n % 10 = 0 THEN 'failed'
        WHEN numbers.n % 20 = 0 THEN 'stopped'
        ELSE 'running'
    END,
    CONCAT('cont-', SHA2(CONCAT(a.id, numbers.n), 256), numbers.n),
    CONCAT('node-', 1 + MOD(a.id + numbers.n, 50)),
    CASE 
        WHEN numbers.n % 10 = 0 THEN 'failed'
        WHEN numbers.n % 20 = 0 THEN 'stopped'
        ELSE 'running'
    END
FROM apps a
CROSS JOIN (SELECT n FROM numbers WHERE n <= 5) numbers
WHERE a.maintenance_mode = 0;

-- Domains with realistic patterns
INSERT INTO domains (app_id, name, ssl_enabled)
SELECT 
    a.id,
    CASE 
        WHEN a.name LIKE '%prod%' THEN CONCAT(REPLACE(a.name, 'prod-', ''), '-', a.id, '.example.com')
        WHEN a.name LIKE '%staging%' THEN CONCAT(REPLACE(a.name, 'staging-', ''), '-', a.id, '.staging.example.com')
        ELSE CONCAT(a.name, '-', a.id, '.dev.example.com')
    END,
    1
FROM apps a
WHERE a.id % 2 = 0;

-- Builds with realistic success/failure patterns
INSERT INTO builds (app_id, source_version, status, started_at, completed_at)
SELECT 
    a.id,
    CONCAT(
        SUBSTRING(SHA2(CONCAT(a.id, numbers.n), 256), 1, 7),
        CASE 
            WHEN numbers.n % 5 = 0 THEN ' (hotfix)'
            WHEN numbers.n % 3 = 0 THEN ' (feature)'
            ELSE ''
        END
    ),
    CASE 
        WHEN numbers.n % 10 = 0 THEN 'failed'
        WHEN numbers.n % 20 = 0 THEN 'building'
        ELSE 'succeeded'
    END,
    DATE_SUB(NOW(), INTERVAL (numbers.n * 2) HOUR),
    CASE 
        WHEN numbers.n % 20 = 0 THEN NULL
        ELSE DATE_SUB(NOW(), INTERVAL ((numbers.n * 2) - 1) HOUR)
    END
FROM apps a
CROSS JOIN (SELECT n FROM numbers_100 WHERE n <= 20) numbers
WHERE a.maintenance_mode = 0;

-- Deployments with realistic patterns
INSERT INTO deployments (app_id, build_id, status, started_at, completed_at)
SELECT 
    b.app_id,
    b.id,
    CASE 
        WHEN b.status = 'failed' THEN 'failed'
        WHEN b.status = 'building' THEN 'pending'
        ELSE 
            CASE 
                WHEN numbers.n % 10 = 0 THEN 'failed'
                WHEN numbers.n % 20 = 0 THEN 'in_progress'
                ELSE 'deployed'
            END
    END,
    b.completed_at,
    CASE 
        WHEN b.status = 'succeeded' AND numbers.n % 20 != 0 
        THEN DATE_ADD(b.completed_at, INTERVAL 10 MINUTE)
        ELSE NULL
    END
FROM builds b
CROSS JOIN (SELECT 1 as n) numbers
WHERE b.status = 'succeeded';

-- Config vars with realistic keys and patterns
INSERT INTO config_vars (app_id, `key`, value, is_secret)
SELECT DISTINCT
    a.id,
    CASE MOD(numbers.n, 8)
        WHEN 0 THEN 'DATABASE_URL'
        WHEN 1 THEN 'REDIS_URL'
        WHEN 2 THEN 'AWS_ACCESS_KEY'
        WHEN 3 THEN 'AWS_SECRET_KEY'
        WHEN 4 THEN 'API_KEY'
        WHEN 5 THEN 'NODE_ENV'
        WHEN 6 THEN 'LOG_LEVEL'
        ELSE 'PORT'
    END,
    CASE MOD(numbers.n, 8)
        WHEN 0 THEN CONCAT('postgres://user:pass@db-', a.id, '.example.com:5432/db')
        WHEN 1 THEN CONCAT('redis://cache-', a.id, '.example.com:6379')
        WHEN 2 THEN CONCAT('AKIA', SUBSTRING(UUID(), 1, 16))
        WHEN 3 THEN SHA2(UUID(), 256)
        WHEN 4 THEN SHA2(CONCAT('key-', a.id), 256)
        WHEN 5 THEN CASE WHEN a.name LIKE '%prod%' THEN 'production' ELSE 'development' END
        WHEN 6 THEN 'info'
        ELSE '8080'
    END,
    CASE MOD(numbers.n, 8) 
        WHEN 0 THEN 1  -- DATABASE_URL is secret
        WHEN 1 THEN 1  -- REDIS_URL is secret
        WHEN 2 THEN 1  -- AWS_ACCESS_KEY is secret
        WHEN 3 THEN 1  -- AWS_SECRET_KEY is secret
        WHEN 4 THEN 1  -- API_KEY is secret
        ELSE 0         -- Other configs are not secret
    END
FROM apps a
CROSS JOIN (SELECT n FROM numbers WHERE n <= 8) numbers
WHERE a.maintenance_mode = 0;

-- Insert realistic metrics data with patterns
INSERT INTO metrics (instance_id, metric_name, metric_value, timestamp)
SELECT 
    i.id,
    m.metric_name,
    CASE m.metric_name
        WHEN 'cpu_usage' THEN 
            CASE 
                WHEN HOUR(NOW()) BETWEEN 9 AND 17 THEN 40 + (RAND() * 40)  -- Higher during business hours
                ELSE 20 + (RAND() * 20)  -- Lower at night
            END
        WHEN 'memory_usage' THEN 
            CASE 
                WHEN i.instance_type LIKE '%large%' THEN 4096 + (RAND() * 4096)
                WHEN i.instance_type LIKE '%medium%' THEN 2048 + (RAND() * 2048)
                ELSE 1024 + (RAND() * 1024)
            END
        WHEN 'disk_usage' THEN 45 + (RAND() * 30)
        WHEN 'network_in' THEN 
            CASE 
                WHEN HOUR(NOW()) BETWEEN 9 AND 17 THEN 1000 + (RAND() * 2000)
                ELSE 500 + (RAND() * 500)
            END
        WHEN 'network_out' THEN 
            CASE 
                WHEN HOUR(NOW()) BETWEEN 9 AND 17 THEN 800 + (RAND() * 1600)
                ELSE 400 + (RAND() * 400)
            END
        WHEN 'request_count' THEN 
            CASE 
                WHEN HOUR(NOW()) BETWEEN 9 AND 17 THEN 1000 + (RAND() * 5000)
                ELSE 100 + (RAND() * 1000)
            END
        WHEN 'error_rate' THEN RAND() * 2
        ELSE RAND() * 100
    END,
    DATE_SUB(NOW(), INTERVAL (n + (RAND() * 10)) MINUTE)
FROM instances i
CROSS JOIN (
    SELECT n, metric_name FROM numbers_100 
    CROSS JOIN (
        SELECT 'cpu_usage' as metric_name UNION ALL 
        SELECT 'memory_usage' UNION ALL
        SELECT 'disk_usage' UNION ALL
        SELECT 'network_in' UNION ALL
        SELECT 'network_out' UNION ALL
        SELECT 'request_count' UNION ALL
        SELECT 'error_rate' UNION ALL
        SELECT 'response_time'
    ) metrics
    WHERE n <= 50
) m
WHERE i.status = 'running';

-- Insert realistic instance logs
INSERT INTO instance_logs (instance_id, log_type, message, timestamp)
SELECT 
    i.id,
    CASE MOD(n, 3)
        WHEN 0 THEN 'app'
        WHEN 1 THEN 'system'
        ELSE 'deployment'
    END,
    CASE MOD(n, 3)
        WHEN 0 THEN 
            CASE MOD(n, 5)
                WHEN 0 THEN CONCAT('Error: Failed to connect to database at ', DATE_FORMAT(NOW() - INTERVAL n MINUTE, '%H:%M:%S'))
                WHEN 1 THEN CONCAT('Warning: High memory usage detected at ', DATE_FORMAT(NOW() - INTERVAL n MINUTE, '%H:%M:%S'))
                WHEN 2 THEN 'Application startup completed successfully'
                WHEN 3 THEN CONCAT('Processing batch job #', n)
                ELSE CONCAT('Request completed in ', 100 + RAND() * 200, 'ms')
            END
        WHEN 1 THEN 
            CASE MOD(n, 5)
                WHEN 0 THEN 'Container health check passed'
                WHEN 1 THEN 'System update initiated'
                WHEN 2 THEN CONCAT('Container memory usage at ', 50 + RAND() * 30, '%')
                WHEN 3 THEN 'Network connectivity check passed'
                ELSE 'Container restart completed'
            END
        ELSE 
            CASE MOD(n, 5)
                WHEN 0 THEN 'Deployment started: Pulling new image'
                WHEN 1 THEN 'Configuration update applied'
                WHEN 2 THEN 'Rolling update in progress'
                WHEN 3 THEN 'Deployment completed successfully'
                ELSE 'Rollback initiated due to health check failure'
            END
    END,
    DATE_SUB(NOW(), INTERVAL n MINUTE)
FROM instances i
CROSS JOIN (SELECT n FROM numbers_1000 WHERE n <= 100) numbers
WHERE i.status = 'running';

-- Insert comprehensive audit logs
INSERT INTO audit_logs (user_id, org_id, action, resource_type, resource_id, created_at)
SELECT 
    CASE 
        WHEN n % 10 = 0 THEN NULL  -- Some actions via API
        ELSE (SELECT id FROM users WHERE id = 1 + MOD(n, 1000))
    END,
    o.id,
    CASE MOD(n, 12)
        WHEN 0 THEN 'create'
        WHEN 1 THEN 'update'
        WHEN 2 THEN 'delete'
        WHEN 3 THEN 'deploy'
        WHEN 4 THEN 'rollback'
        WHEN 5 THEN 'scale'
        WHEN 6 THEN 'restart'
        WHEN 7 THEN 'configure'
        WHEN 8 THEN 'enable'
        WHEN 9 THEN 'disable'
        WHEN 10 THEN 'access'
        ELSE 'view'
    END,
    CASE MOD(n, 8)
        WHEN 0 THEN 'app'
        WHEN 1 THEN 'instance'
        WHEN 2 THEN 'config'
        WHEN 3 THEN 'deployment'
        WHEN 4 THEN 'domain'
        WHEN 5 THEN 'build'
        WHEN 6 THEN 'apikey'
        ELSE 'user'
    END,
    CONCAT(1 + MOD(n, 1000)),  -- resource_id as string
    DATE_SUB(NOW(), INTERVAL n MINUTE)
FROM orgs o
CROSS JOIN (SELECT n FROM numbers_1000 WHERE n <= 1000) numbers
WHERE (o.id + numbers.n) % 3 = 0;

-- Drop temporary tables
DROP TABLE IF EXISTS numbers;
DROP TABLE IF EXISTS numbers_100;
DROP TABLE IF EXISTS numbers_1000;
DROP TABLE IF EXISTS app_prefixes;
DROP TABLE IF EXISTS tech_stacks;