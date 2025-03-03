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

-- Create quotas for organizations
INSERT INTO quotas (name, org_id, memory_limit, instance_limit, routes_limit, services_limit)
SELECT 
    CONCAT('quota-', o.id),
    o.id,
    CASE 
        WHEN o.id <= 20 THEN 102400  -- 100GB for Enterprise
        WHEN o.id <= 50 THEN 51200   -- 50GB for Startups
        ELSE 10240                    -- 10GB for Teams
    END,
    CASE 
        WHEN o.id <= 20 THEN 100     -- Enterprise can have up to 100 instances
        WHEN o.id <= 50 THEN 50      -- Startups can have up to 50 instances
        ELSE 20                       -- Teams can have up to 20 instances
    END,
    CASE 
        WHEN o.id <= 20 THEN 50      -- Enterprise can have up to 50 routes
        WHEN o.id <= 50 THEN 25      -- Startups can have up to 25 routes
        ELSE 10                       -- Teams can have up to 10 routes
    END,
    CASE 
        WHEN o.id <= 20 THEN 20      -- Enterprise can have up to 20 services
        WHEN o.id <= 50 THEN 10      -- Startups can have up to 10 services
        ELSE 5                        -- Teams can have up to 5 services
    END
FROM orgs o;

-- Create spaces within orgs
INSERT INTO spaces (org_id, name)
SELECT 
    o.id,
    s.space_name
FROM orgs o
CROSS JOIN (
    SELECT 'production' as space_name UNION ALL
    SELECT 'staging' UNION ALL
    SELECT 'development' UNION ALL
    SELECT 'testing' UNION ALL
    SELECT 'sandbox'
) s
WHERE 
    (o.id <= 20) OR                                    -- Enterprise gets all spaces
    (o.id <= 50 AND s.space_name IN ('production', 'staging', 'development')) OR  -- Startups get 3 spaces
    (o.id > 50 AND s.space_name IN ('production', 'development'))                 -- Teams get 2 spaces
;

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

-- Create allocations for apps
INSERT INTO allocations (cpu, memory, uplink, downlink, disk)
VALUES
(0.5, 512, 100, 100, 1024),     -- Small
(1.0, 1024, 250, 250, 2048),    -- Medium
(2.0, 2048, 500, 500, 4096),    -- Large
(4.0, 4096, 1000, 1000, 8192),  -- XL
(8.0, 8192, 2000, 2000, 16384); -- 2XL

-- Create nodes for instances
INSERT INTO nodes (region_id, name, status, cpu_total, cpu_available, memory_total, memory_available, disk_total, disk_available)
SELECT 
    r.id,
    CONCAT('node-', r.name, '-', numbers.n),
    CASE 
        WHEN numbers.n % 10 = 0 THEN 'maintenance'
        WHEN numbers.n % 20 = 0 THEN 'provisioning'
        ELSE 'active'
    END,
    32,  -- 32 CPU cores total
    32 - (numbers.n % 8),  -- Some cores in use
    65536,  -- 64GB memory total
    65536 - (numbers.n * 1024),  -- Some memory in use
    1048576,  -- 1TB disk total
    1048576 - (numbers.n * 10240)  -- Some disk in use
FROM regions r
CROSS JOIN (SELECT n FROM numbers WHERE n <= 5) numbers;

-- Data Services
INSERT INTO data_services (region_id, name, service_type, status, version, plan)
SELECT 
    r.id,
    CONCAT(
        CASE s.service_type 
            WHEN 'database' THEN 'db'
            WHEN 'cache' THEN 'cache'
            WHEN 'message_queue' THEN 'mq'
            WHEN 'network_filesystem' THEN 'nfs'
            ELSE 'cdn'
        END,
        '-', r.name, '-', numbers.n
    ),
    s.service_type,
    CASE 
        WHEN numbers.n % 10 = 0 THEN 'maintenance'
        WHEN numbers.n % 20 = 0 THEN 'provisioning'
        ELSE 'active'
    END,
    CASE s.service_type 
        WHEN 'database' THEN '13.4'
        WHEN 'cache' THEN '6.2'
        WHEN 'message_queue' THEN '3.9'
        WHEN 'network_filesystem' THEN '4.1'
        ELSE '2.5'
    END,
    CASE 
        WHEN numbers.n % 3 = 0 THEN 'standard'
        WHEN numbers.n % 3 = 1 THEN 'premium'
        ELSE 'basic'
    END
FROM regions r
CROSS JOIN (SELECT n FROM numbers WHERE n <= 3) numbers
CROSS JOIN (
    SELECT 'database' as service_type UNION ALL
    SELECT 'cache' UNION ALL
    SELECT 'message_queue' UNION ALL
    SELECT 'network_filesystem' UNION ALL
    SELECT 'cdn'
) s;

-- Apps with realistic naming and configuration
INSERT INTO apps (name, org_id, space_id, git_repo, git_branch, container_image_url, 
                 default_allocation_id, region_id, instances, health_check_type, 
                 health_check_endpoint, runtime, restart_policy, maintenance_mode)
SELECT 
    CONCAT(p.prefix, '-', 
        CASE 
            WHEN sp.name = 'production' THEN 'prod'
            WHEN sp.name = 'staging' THEN 'staging'
            ELSE 'dev'
        END,
        '-', t.stack, '-', numbers.n
    ),
    o.id,
    sp.id,
    CONCAT('github.com/org', o.id, '/', p.prefix, '-', t.stack),
    CASE 
        WHEN sp.name = 'production' THEN 'main'
        WHEN sp.name = 'staging' THEN 'staging'
        ELSE 'develop'
    END,
    CONCAT('registry.example.com/', t.image),
    1 + (o.id % 5),  -- allocation_id
    1 + MOD(o.id + numbers.n, 8),  -- region_id
    CASE 
        WHEN sp.name = 'production' THEN 3 + (numbers.n % 3)  -- prod gets 3-5 instances
        WHEN sp.name = 'staging' THEN 2  -- staging gets 2 instances
        ELSE 1  -- dev gets 1 instance
    END,
    CASE 
        WHEN p.prefix IN ('web', 'api', 'admin', 'mobile') THEN 'http'
        ELSE 'port'
    END,
    CASE 
        WHEN p.prefix IN ('web', 'api', 'admin', 'mobile') THEN '/health'
        ELSE NULL
    END,
    t.stack,
    CASE 
        WHEN sp.name = 'production' THEN 'always'
        ELSE 'on-failure'
    END,
    CASE WHEN numbers.n % 20 = 0 THEN 1 ELSE 0 END
FROM orgs o
JOIN spaces sp ON o.id = sp.org_id
CROSS JOIN (SELECT n FROM numbers WHERE n <= 3) numbers
CROSS JOIN app_prefixes p
CROSS JOIN tech_stacks t
WHERE (o.id + numbers.n) % 3 = 0
LIMIT 1000;

-- Instances with realistic scaling patterns
INSERT INTO instances (app_id, instance_type, status, container_id, node_id, instance_status, instance_index, last_health_check)
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
        WHEN numbers.n % 10 = 0 THEN 'maintenance'
        WHEN numbers.n % 20 = 0 THEN 'provisioning'
        ELSE 'active'
    END,
    CONCAT('cont-', SUBSTRING(SHA2(CONCAT(a.id, numbers.n), 256), 1, 10)),
    (SELECT id FROM nodes WHERE id = 1 + MOD(a.id + numbers.n, 
                                    (SELECT COUNT(*) FROM nodes))),
    CASE 
        WHEN numbers.n % 10 = 0 THEN 'failed'
        WHEN numbers.n % 20 = 0 THEN 'stopped'
        ELSE 'running'
    END,
    numbers.n,  -- instance_index
    DATE_SUB(NOW(), INTERVAL numbers.n MINUTE)
FROM apps a
CROSS JOIN (SELECT n FROM numbers WHERE n <= 5) numbers
WHERE a.maintenance_mode = 0 AND numbers.n <= a.instances;

-- Domains with realistic patterns
INSERT INTO domains (name, ssl_enabled, ssl_cert_path, ssl_key_path, ssl_expiry_date)
SELECT 
    CASE 
        WHEN numbers.n = 1 THEN 'example.com'
        WHEN numbers.n = 2 THEN 'staging.example.com'
        WHEN numbers.n = 3 THEN 'dev.example.com'
        WHEN numbers.n = 4 THEN 'api.example.com'
        ELSE CONCAT('custom-', numbers.n, '.example.com')
    END,
    1,  -- ssl_enabled
    CONCAT('/etc/certs/', 
        CASE 
            WHEN numbers.n = 1 THEN 'example.com'
            WHEN numbers.n = 2 THEN 'staging.example.com'
            WHEN numbers.n = 3 THEN 'dev.example.com'
            WHEN numbers.n = 4 THEN 'api.example.com'
            ELSE CONCAT('custom-', numbers.n, '.example.com')
        END, '.crt'),
    CONCAT('/etc/certs/', 
        CASE 
            WHEN numbers.n = 1 THEN 'example.com'
            WHEN numbers.n = 2 THEN 'staging.example.com'
            WHEN numbers.n = 3 THEN 'dev.example.com'
            WHEN numbers.n = 4 THEN 'api.example.com'
            ELSE CONCAT('custom-', numbers.n, '.example.com')
        END, '.key'),
    DATE_ADD(CURRENT_DATE, INTERVAL 10 MONTH)
FROM (SELECT n FROM numbers WHERE n <= 10) numbers;

-- Routes with app mapping (ensures uniqueness)
INSERT INTO routes (domain_id, path, app_id, weight)
SELECT DISTINCT
    d.id,
    CASE 
        WHEN a.name LIKE '%api%' THEN CONCAT('/api/v', 1 + (a.id % 3), '/', a.id)
        WHEN a.name LIKE '%admin%' THEN CONCAT('/admin/', a.id)
        WHEN a.name LIKE '%web%' AND sp.name = 'production' THEN CONCAT('/', a.id)
        WHEN a.name LIKE '%web%' THEN CONCAT('/', sp.name, '/', a.id)
        ELSE CONCAT('/', SUBSTRING_INDEX(a.name, '-', 1), '/', a.id)
    END,
    a.id,
    CASE 
        WHEN sp.name = 'production' THEN 100
        WHEN sp.name = 'staging' THEN 0
        ELSE 0
    END
FROM apps a
JOIN spaces sp ON a.space_id = sp.id
JOIN domains d ON 
    (sp.name = 'production' AND d.name = 'example.com') OR
    (sp.name = 'staging' AND d.name = 'staging.example.com') OR
    (sp.name NOT IN ('production', 'staging') AND d.name = 'dev.example.com')
WHERE a.maintenance_mode = 0
LIMIT 200;

-- Builds with realistic success/failure patterns
INSERT INTO builds (app_id, source_version, status, build_pack_used, log_url, started_at, completed_at)
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
    CASE 
        WHEN a.runtime = 'nodejs' THEN 'heroku/nodejs'
        WHEN a.runtime = 'python' THEN 'heroku/python'
        WHEN a.runtime = 'ruby' THEN 'heroku/ruby'
        WHEN a.runtime = 'java' THEN 'heroku/java'
        ELSE 'heroku/go'
    END,
    CONCAT('https://logs.example.com/builds/', a.id, '-', numbers.n),
    DATE_SUB(NOW(), INTERVAL (numbers.n * 2) HOUR),
    CASE 
        WHEN numbers.n % 20 = 0 THEN NULL
        ELSE DATE_SUB(NOW(), INTERVAL ((numbers.n * 2) - 1) HOUR)
    END
FROM apps a
CROSS JOIN (SELECT n FROM numbers_100 WHERE n <= 10) numbers
WHERE a.maintenance_mode = 0;

-- Deployments with realistic patterns
INSERT INTO deployments (app_id, build_id, status, deployment_strategy, started_at, completed_at)
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
    CASE 
        WHEN sp.name = 'production' THEN 'blue-green'
        WHEN sp.name = 'staging' THEN 'canary'
        ELSE 'rolling'
    END,
    b.completed_at,
    CASE 
        WHEN b.status = 'succeeded' AND numbers.n % 20 != 0 
        THEN DATE_ADD(b.completed_at, INTERVAL 10 MINUTE)
        ELSE NULL
    END
FROM builds b
JOIN apps a ON b.app_id = a.id
JOIN spaces sp ON a.space_id = sp.id
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
        WHEN 5 THEN 
            CASE 
                WHEN sp.name = 'production' THEN 'production'
                WHEN sp.name = 'staging' THEN 'staging'
                ELSE 'development'
            END
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
JOIN spaces sp ON a.space_id = sp.id
CROSS JOIN (SELECT n FROM numbers WHERE n <= 8) numbers
WHERE a.maintenance_mode = 0;

-- Service Bindings
INSERT INTO service_bindings (app_id, service_id, credentials, binding_name, status)
SELECT 
    a.id,
    ds.id,
    CASE 
        WHEN ds.service_type = 'database' THEN 
            CONCAT('{"uri": "postgresql://user:', SHA2(CONCAT(a.id, ds.id), 256), '@', ds.name, '.internal:5432/db-', a.id, '"}')
        WHEN ds.service_type = 'cache' THEN 
            CONCAT('{"uri": "redis://default:', SHA2(CONCAT(a.id, ds.id), 256), '@', ds.name, '.internal:6379/0"}')
        WHEN ds.service_type = 'message_queue' THEN 
            CONCAT('{"uri": "amqp://user:', SHA2(CONCAT(a.id, ds.id), 256), '@', ds.name, '.internal:5672/vhost"}')
        WHEN ds.service_type = 'network_filesystem' THEN 
            CONCAT('{"mount_point": "/mnt/', ds.name, '", "options": "rw,sync"}')
        ELSE 
            CONCAT('{"api_key": "', SHA2(CONCAT(a.id, ds.id), 256), '", "endpoint": "https://', ds.name, '.cdn.internal"}')
    END,
    CONCAT(ds.service_type, '-binding-', numbers.n),
    'created'
FROM apps a
JOIN spaces sp ON a.space_id = sp.id
JOIN data_services ds ON ds.region_id = a.region_id
CROSS JOIN (SELECT 1 as n) numbers
WHERE 
    a.maintenance_mode = 0 AND
    ((sp.name = 'production' AND ds.plan = 'premium') OR
     (sp.name = 'staging' AND ds.plan = 'standard') OR
     (sp.name NOT IN ('production', 'staging') AND ds.plan = 'basic'))
LIMIT 200;

-- Health Checks
INSERT INTO health_checks (app_id, type, endpoint, timeout, check_interval, healthy_threshold, unhealthy_threshold)
SELECT 
    a.id,
    CASE 
        WHEN a.name LIKE '%web%' OR a.name LIKE '%api%' THEN 'http'
        WHEN a.name LIKE '%worker%' OR a.name LIKE '%batch%' OR a.name LIKE '%cron%' THEN 'process'
        ELSE 'port'
    END,
    CASE 
        WHEN a.name LIKE '%web%' OR a.name LIKE '%api%' THEN '/health'
        ELSE NULL
    END,
    CASE 
        WHEN sp.name = 'production' THEN 30
        ELSE 60
    END,
    CASE 
        WHEN sp.name = 'production' THEN 5
        ELSE 10
    END,
    CASE 
        WHEN sp.name = 'production' THEN 2
        ELSE 3
    END,
    CASE 
        WHEN sp.name = 'production' THEN 2
        ELSE 3
    END
FROM apps a
JOIN spaces sp ON a.space_id = sp.id
WHERE a.maintenance_mode = 0;

-- Autoscaling Rules
INSERT INTO autoscaling_rules (app_id, min_instances, max_instances, metric_type, threshold_value, cool_down_period_seconds)
SELECT 
    a.id,
    CASE 
        WHEN sp.name = 'production' THEN 2
        ELSE 1
    END,
    CASE 
        WHEN sp.name = 'production' THEN 10
        WHEN sp.name = 'staging' THEN 5
        ELSE 3
    END,
    CASE 
        WHEN a.name LIKE '%web%' OR a.name LIKE '%api%' THEN 'http_throughput'
        WHEN a.name LIKE '%worker%' THEN 'queue_depth'
        ELSE 'cpu'
    END,
    CASE 
        WHEN a.name LIKE '%web%' OR a.name LIKE '%api%' THEN 1000  -- requests per second
        WHEN a.name LIKE '%worker%' THEN 100  -- queue depth
        ELSE 80  -- CPU percentage
    END,
    CASE 
        WHEN sp.name = 'production' THEN 180
        ELSE 300
    END
FROM apps a
JOIN spaces sp ON a.space_id = sp.id
WHERE 
    a.maintenance_mode = 0 AND
    (sp.name = 'production' OR sp.name = 'staging');

-- Network Policies
INSERT INTO network_policies (source_app_id, destination_app_id, protocol, port_range_start, port_range_end)
SELECT 
    a1.id,
    a2.id,
    'tcp',
    CASE
        WHEN a2.name LIKE '%api%' THEN 80
        WHEN a2.name LIKE '%db%' THEN 5432
        WHEN a2.name LIKE '%redis%' OR a2.name LIKE '%cache%' THEN 6379
        ELSE 8080
    END,
    CASE
        WHEN a2.name LIKE '%api%' THEN 80
        WHEN a2.name LIKE '%db%' THEN 5432
        WHEN a2.name LIKE '%redis%' OR a2.name LIKE '%cache%' THEN 6379
        ELSE 8080
    END
FROM apps a1
JOIN apps a2 ON a1.id != a2.id AND a1.space_id = a2.space_id
WHERE 
    a1.maintenance_mode = 0 AND 
    a2.maintenance_mode = 0 AND
    (a1.name LIKE '%web%' OR a1.name LIKE '%api%' OR a1.name LIKE '%worker%') AND
    (a2.name LIKE '%api%' OR a2.name LIKE '%db%' OR a2.name LIKE '%cache%')
LIMIT 100;

-- Tasks
INSERT INTO tasks (app_id, command, name, status, memory_in_mb, disk_in_mb, result, started_at, completed_at)
SELECT 
    a.id,
    CASE 
        WHEN a.runtime = 'nodejs' THEN 'node task-runner.js'
        WHEN a.runtime = 'python' THEN 'python manage.py run_task'
        WHEN a.runtime = 'ruby' THEN 'bundle exec rake task:run'
        WHEN a.runtime = 'java' THEN 'java -jar task-runner.jar'
        ELSE 'go run task/main.go'
    END,
    CASE 
        WHEN numbers.n = 1 THEN 'database-migration'
        WHEN numbers.n = 2 THEN 'data-import'
        WHEN numbers.n = 3 THEN 'cleanup-job'
        ELSE CONCAT('task-', numbers.n)
    END,
    CASE 
        WHEN numbers.n % 10 = 0 THEN 'failed'
        WHEN numbers.n % 20 = 0 THEN 'running'
        ELSE 'succeeded'
    END,
    512,  -- memory_in_mb
    1024,  -- disk_in_mb
    CASE 
        WHEN numbers.n % 10 = 0 THEN 'Error: Task failed with exit code 1'
        WHEN numbers.n % 20 = 0 THEN NULL
        ELSE 'Task completed successfully'
    END,
    DATE_SUB(NOW(), INTERVAL numbers.n HOUR),
    CASE 
        WHEN numbers.n % 20 = 0 THEN NULL
        ELSE DATE_SUB(NOW(), INTERVAL (numbers.n - 1) HOUR)
    END
FROM apps a
CROSS JOIN (SELECT n FROM numbers WHERE n <= 5) numbers
WHERE a.maintenance_mode = 0;

-- Rollbacks (fixed for MySQL's only_full_group_by mode)
INSERT INTO rollbacks (app_id, from_deployment_id, to_deployment_id, status, reason, started_at, completed_at)
SELECT 
    d_new.app_id,
    d_new.id as from_deployment_id,
    d_old.id as to_deployment_id,
    'completed',
    'High error rate detected after deployment',
    DATE_SUB(NOW(), INTERVAL 12 HOUR),
    DATE_SUB(NOW(), INTERVAL 11 HOUR)
FROM deployments d_new
JOIN (
    -- For each app, get the second most recent deployment
    SELECT app_id, MAX(id) as id
    FROM deployments 
    WHERE status = 'deployed'
    GROUP BY app_id
) d_old_max ON d_new.app_id = d_old_max.app_id
JOIN deployments d_old ON d_old.id = d_old_max.id
JOIN apps a ON d_new.app_id = a.id
JOIN spaces sp ON a.space_id = sp.id
WHERE 
    sp.name = 'production' AND 
    d_new.status = 'deployed' AND
    d_new.id > d_old.id
LIMIT 10;

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
    WHERE n <= 10
) m
WHERE i.status = 'active'
LIMIT 5000;

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
CROSS JOIN (SELECT n FROM numbers_100 WHERE n <= 20) numbers
WHERE i.status = 'active'
LIMIT 1000;

-- Insert comprehensive audit logs
INSERT INTO audit_logs (user_id, org_id, action, resource_type, resource_id, ip_address, created_at)
SELECT 
    CASE 
        WHEN n % 10 = 0 THEN NULL  -- Some actions via API
        ELSE (SELECT id FROM users WHERE id = 1 + MOD(n, 1000) LIMIT 1)
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
    CONCAT('192.168.', 1 + MOD(n, 254), '.', 1 + MOD(n * 3, 254)),
    DATE_SUB(NOW(), INTERVAL n MINUTE)
FROM orgs o
CROSS JOIN (SELECT n FROM numbers_100 WHERE n <= 100) numbers
WHERE (o.id + numbers.n) % 3 = 0
LIMIT 1000;

-- Deployment logs
INSERT INTO deployment_logs (deployment_id, log_type, message, timestamp)
SELECT 
    d.id,
    CASE MOD(n, 3)
        WHEN 0 THEN 'app'
        WHEN 1 THEN 'system'
        ELSE 'deployment'
    END,
    CASE MOD(n, 10)
        WHEN 0 THEN 'Starting deployment process'
        WHEN 1 THEN 'Pulling container image'
        WHEN 2 THEN 'Container image pulled successfully'
        WHEN 3 THEN 'Starting new containers'
        WHEN 4 THEN 'Health check passed for new containers'
        WHEN 5 THEN 'Routing traffic to new containers'
        WHEN 6 THEN 'Old containers still serving requests'
        WHEN 7 THEN 'Stopping old containers'
        WHEN 8 THEN 'Deployment completed successfully'
        ELSE CASE WHEN d.status = 'failed' THEN 'Deployment failed: Health check timeout' ELSE 'All systems nominal' END
    END,
    DATE_SUB(d.started_at, INTERVAL (10 - n) MINUTE)
FROM deployments d
CROSS JOIN (SELECT n FROM numbers WHERE n <= 10) numbers
WHERE d.status IN ('deployed', 'failed')
LIMIT 500;

-- Drop temporary tables
DROP TABLE IF EXISTS numbers;
DROP TABLE IF EXISTS numbers_100;
DROP TABLE IF EXISTS numbers_1000;
DROP TABLE IF EXISTS app_prefixes;
DROP TABLE IF EXISTS tech_stacks;