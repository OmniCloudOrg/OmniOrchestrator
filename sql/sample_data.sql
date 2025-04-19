-- Sample Data Generation Script for Cloud Platform
-- This script uses only direct inserts (no temp tables, no stored procedures)
-- Compatible with all MySQL environments

-- Disable foreign key checks temporarily for faster inserts
SET FOREIGN_KEY_CHECKS = 0;
SET UNIQUE_CHECKS = 0;
SET SQL_MODE = '';

-- Set variables for scale (adjust as needed)
SET @USER_COUNT = 25;
SET @ORG_COUNT = 10;
SET @SPACE_COUNT = 20;
SET @APP_COUNT = 50;
SET @INSTANCE_COUNT = 100;
SET @DOMAIN_COUNT = 15;
SET @REGION_COUNT = 10;
SET @NODE_COUNT = 20;
SET @SERVICE_COUNT = 15;
SET @BUILD_COUNT = 100;
SET @DEPLOYMENT_COUNT = 150;
SET @TASK_COUNT = 75;
SET @LOG_COUNT = 200;
SET @METRIC_COUNT = 1000;

-- 1. Populate allocations
INSERT INTO allocations (name, cpu, memory, uplink, downlink, disk, price_per_hour)
VALUES
('starter-small'      , 1   , 1024   , 5    , 5    , 2048   , 0.012),
('starter-medium'     , 2   , 2048   , 10   , 10   , 4096   , 0.024),
('standard-small'     , 2   , 4096   , 25   , 25   , 10240  , 0.048),
('standard-medium'    , 4   , 8192   , 50   , 50   , 20480  , 0.096),
('standard-large'     , 8   , 16384  , 100  , 100  , 40960  , 0.192),
('performance-small'  , 16  , 32768  , 200  , 200  , 81920  , 0.384),
('performance-medium' , 32  , 65536  , 400  , 400  , 163840 , 0.768),
('performance-large'  , 64  , 131072 , 800  , 800  , 327680 , 1.536),
('premium-small'      , 96  , 196608 , 1200 , 1200 , 491520 , 2.304),
('premium-medium'     , 128 , 262144 , 1600 , 1600 , 655360 , 3.072),
('premium-large'      , 192 , 393216 , 2400 , 2400 , 983040 , 4.608);

INSERT INTO permissions (name, description, resource_type, action)
VALUES
('user.create'    , 'Create a new user'                , 'user'         , 'create'),
('user.read'      , 'View user details'                , 'user'         , 'read'),
('user.update'    , 'Update user details'              , 'user'         , 'update'),
('user.delete'    , 'Delete a user'                    , 'user'         , 'delete'),
('org.create'     , 'Create a new organization'        , 'organization' , 'create'),
('org.read'       , 'View organization details'        , 'organization' , 'read'),
('org.update'     , 'Update organization details'      , 'organization' , 'update'),
('org.delete'     , 'Delete an organization'           , 'organization' , 'delete'),
('org.billing'    , 'Manage organization billing'      , 'organization' , 'billing'),
('space.create'   , 'Create a new space'               , 'space'        , 'create'),
('space.read'     , 'View space details'               , 'space'        , 'read'),
('space.update'   , 'Update space details'             , 'space'        , 'update'),
('space.delete'   , 'Delete a space'                   , 'space'        , 'delete'),
('app.create'     , 'Create a new application'         , 'application'  , 'create'),
('app.read'       , 'View application details'         , 'application'  , 'read'),
('app.update'     , 'Update application details'       , 'application'  , 'update'),
('app.delete'     , 'Delete an application'            , 'application'  , 'delete'),
('app.deploy'     , 'Deploy an application'            , 'application'  , 'deploy'),
('app.scale'      , 'Scale an application'             , 'application'  , 'scale'),
('app.restart'    , 'Restart an application'           , 'application'  , 'restart'),
('app.logs'       , 'View application logs'            , 'application'  , 'logs'),
('service.create' , 'Create a new service'             , 'service'      , 'create'),
('service.read'   , 'View service details'             , 'service'      , 'read'),
('service.update' , 'Update service details'           , 'service'      , 'update'),
('service.delete' , 'Delete a service'                 , 'service'      , 'delete'),
('service.bind'   , 'Bind a service to an application' , 'service'      , 'bind'),
('domain.create'  , 'Create a new domain'              , 'domain'       , 'create'),
('domain.read'    , 'View domain details'              , 'domain'       , 'read'),
('domain.update'  , 'Update domain details'            , 'domain'       , 'update'),
('domain.delete'  , 'Delete a domain'                  , 'domain'       , 'delete'),
('route.create'   , 'Create a new route'               , 'route'        , 'create'),
('route.read'     , 'View route details'               , 'route'        , 'read'),
('route.update'   , 'Update route details'             , 'route'        , 'update'),
('route.delete'   , 'Delete a route'                   , 'route'        , 'delete'),
('quota.create'   , 'Create a new quota'               , 'quota'        , 'create'),
('quota.read'     , 'View quota details'               , 'quota'        , 'read'),
('quota.update'   , 'Update quota details'             , 'quota'        , 'update'),
('quota.delete'   , 'Delete a quota'                   , 'quota'        , 'delete'),
('config.create'  , 'Create configuration variables'   , 'config'       , 'create'),
('config.read'    , 'View configuration variables'     , 'config'       , 'read'),
('config.update'  , 'Update configuration variables'   , 'config'       , 'update'),
('config.delete'  , 'Delete configuration variables'   , 'config'       , 'delete');

INSERT INTO roles (name, description, is_system_role, scope)
VALUES
('system_admin'     , 'Full system administration access'         , 1 , 'global'),
('org_owner'        , 'Organization owner with full control'      , 1 , 'organization'),
('org_admin'        , 'Organization administrator'                , 1 , 'organization'),
('org_billing'      , 'Organization billing administrator'        , 1 , 'organization'),
('space_developer'  , 'Developer with app deployment permissions' , 1 , 'space'),
('space_manager'    , 'Space administrator'                       , 1 , 'space'),
('space_auditor'    , 'Read-only access to a space'               , 1 , 'space'),
('app_manager'      , 'Application administrator'                 , 1 , 'application'),
('app_developer'    , 'Application developer'                     , 1 , 'application'),
('app_viewer'       , 'Application viewer with read-only access'  , 1 , 'application'),
('service_admin'    , 'Service administrator'                     , 0 , 'global'),
('networking_admin' , 'Networking administrator'                  , 0 , 'organization'),
('security_admin'   , 'Security administrator'                    , 0 , 'organization'),
('monitoring_admin' , 'Monitoring administrator'                  , 0 , 'organization'),
('backup_operator'  , 'Backup and restore operator'               , 0 , 'organization');

INSERT INTO providers (name, display_name, provider_type, status) 
VALUES
('aws', 'Amazon Web Services', 'cloud', 'active'),
('gcp', 'Google Cloud Platform', 'cloud', 'active'),
('azure', 'Microsoft Azure', 'cloud', 'active');

INSERT INTO regions (name, display_name, provider, location, coordinates, is_public, class)
VALUES
('us-east-1'      , 'US East (N. Virginia)'    , 1 , 'Northern Virginia, USA'   , POINT(-77.47, 39.06)  , 1 , 'standard'),
('us-west-1'      , 'US West (N. California)'  , 1 , 'Northern California, USA' , POINT(-121.97, 37.35) , 1 , 'standard'),
('us-west-2'      , 'US West (Oregon)'         , 1 , 'Oregon, USA'              , POINT(-122.67, 45.52) , 1 , 'premium'),
('eu-west-1'      , 'EU West (Ireland)'        , 1 , 'Dublin, Ireland'          , POINT(-6.26, 53.34)   , 1 , 'standard'),
('us-central1'    , 'US Central (Iowa)'        , 2 , 'Iowa, USA'                , POINT(-93.63, 41.88)  , 1 , 'premium'),
('europe-west4'   , 'EU West (Netherlands)'    , 2 , 'Eemshaven, Netherlands'   , POINT(6.83, 53.44)    , 1 , 'standard'),
('eu-central-1'   , 'EU Central (Frankfurt)'   , 1 , 'Frankfurt, Germany'       , POINT(8.68, 50.11)    , 1 , 'premium'),
('ap-southeast-1' , 'Asia Pacific (Singapore)' , 1 , 'Singapore'                , POINT(103.85, 1.29)   , 1 , 'standard'),
('ap-northeast-1' , 'Asia Pacific (Tokyo)'     , 1 , 'Tokyo, Japan'             , POINT(139.69, 35.69)  , 1 , 'standard'),
('ap-southeast-2' , 'Asia Pacific (Sydney)'    , 1 , 'Sydney, Australia'        , POINT(151.21, -33.87) , 1 , 'standard');

INSERT INTO providers_regions (provider_id, region_id, status)
VALUES
(1, 1, 'active'),
(1, 2, 'active'),
(1, 3, 'active'),
(1, 4, 'active'),
(2, 5, 'active'),
(2, 6, 'active'),
(1, 7, 'active'),
(1, 8, 'active'),
(1, 9, 'active'),
(1, 10, 'active');


INSERT INTO users (email, email_verified, password, salt, password_changed_at, login_attempts, two_factor_enabled, active, status, created_at, last_login_at)
VALUES
('user1@example.com'       , 1 , SHA2('password1', 256)  , SHA2('salt1', 256)  , '2023-01-15 08:30:00' , 0 , 0 , 1 , 'active' , '2022-05-10 14:20:00' , '2024-02-28 09:15:00'),
('developer2@gmail.com'    , 1 , SHA2('password2', 256)  , SHA2('salt2', 256)  , '2023-02-20 10:45:00' , 1 , 1 , 1 , 'active' , '2022-06-12 16:40:00' , '2024-02-29 11:30:00'),
('admin3@company.com'      , 1 , SHA2('password3', 256)  , SHA2('salt3', 256)  , '2023-03-25 12:15:00' , 0 , 1 , 1 , 'active' , '2022-07-15 09:30:00' , '2024-03-01 08:45:00'),
('test4@techops.co'        , 1 , SHA2('password4', 256)  , SHA2('salt4', 256)  , '2023-04-10 14:20:00' , 0 , 0 , 1 , 'active' , '2022-08-20 11:10:00' , '2024-03-02 10:20:00'),
('user5@yahoo.com'         , 1 , SHA2('password5', 256)  , SHA2('salt5', 256)  , '2023-05-15 09:30:00' , 2 , 0 , 1 , 'active' , '2022-09-25 15:45:00' , '2024-03-03 14:30:00'),
('developer6@hotmail.com'  , 1 , SHA2('password6', 256)  , SHA2('salt6', 256)  , '2023-06-20 11:45:00' , 0 , 1 , 1 , 'active' , '2022-10-30 10:20:00' , '2024-03-04 09:15:00'),
('admin7@cloudplatform.io' , 1 , SHA2('password7', 256)  , SHA2('salt7', 256)  , '2023-07-25 13:15:00' , 0 , 1 , 1 , 'active' , '2022-11-05 08:30:00' , '2024-03-05 11:45:00'),
('test8@outlook.com'       , 1 , SHA2('password8', 256)  , SHA2('salt8', 256)  , '2023-08-10 15:30:00' , 1 , 0 , 1 , 'active' , '2022-12-10 13:40:00' , '2024-03-06 15:20:00'),
('user9@protonmail.com'    , 1 , SHA2('password9', 256)  , SHA2('salt9', 256)  , '2023-09-15 10:45:00' , 0 , 0 , 1 , 'active' , '2023-01-15 12:15:00' , '2024-03-07 08:30:00'),
('developer10@icloud.com'  , 1 , SHA2('password10', 256) , SHA2('salt10', 256) , '2023-10-20 12:15:00' , 0 , 1 , 1 , 'active' , '2023-02-20 14:30:00' , '2024-03-08 10:45:00');

INSERT INTO users (email, email_verified, password, salt, password_changed_at, login_attempts, two_factor_enabled, active, status, created_at, last_login_at)
VALUES
('user11@example.com'       , 1 , SHA2('password11', 256) , SHA2('salt11', 256) , '2023-01-16 08:30:00' , 0 , 0 , 1 , 'active'   , '2022-05-11 14:20:00' , '2024-02-28 09:15:00'),
('developer12@gmail.com'    , 1 , SHA2('password12', 256) , SHA2('salt12', 256) , '2023-02-21 10:45:00' , 1 , 1 , 1 , 'active'   , '2022-06-13 16:40:00' , '2024-02-29 11:30:00'),
('admin13@company.com'      , 1 , SHA2('password13', 256) , SHA2('salt13', 256) , '2023-03-26 12:15:00' , 0 , 1 , 1 , 'active'   , '2022-07-16 09:30:00' , '2024-03-01 08:45:00'),
('test14@techops.co'        , 1 , SHA2('password14', 256) , SHA2('salt14', 256) , '2023-04-11 14:20:00' , 0 , 0 , 1 , 'active'   , '2022-08-21 11:10:00' , '2024-03-02 10:20:00'),
('user15@yahoo.com'         , 1 , SHA2('password15', 256) , SHA2('salt15', 256) , '2023-05-16 09:30:00' , 2 , 0 , 1 , 'active'   , '2022-09-26 15:45:00' , '2024-03-03 14:30:00'),
('developer16@hotmail.com'  , 1 , SHA2('password16', 256) , SHA2('salt16', 256) , '2023-06-21 11:45:00' , 0 , 1 , 1 , 'active'   , '2022-10-31 10:20:00' , '2024-03-04 09:15:00'),
('admin17@cloudplatform.io' , 1 , SHA2('password17', 256) , SHA2('salt17', 256) , '2023-07-26 13:15:00' , 0 , 1 , 1 , 'active'   , '2022-11-06 08:30:00' , '2024-03-05 11:45:00'),
('test18@outlook.com'       , 1 , SHA2('password18', 256) , SHA2('salt18', 256) , '2023-08-11 15:30:00' , 1 , 0 , 1 , 'active'   , '2022-12-11 13:40:00' , '2024-03-06 15:20:00'),
('user19@protonmail.com'    , 1 , SHA2('password19', 256) , SHA2('salt19', 256) , '2023-09-16 10:45:00' , 0 , 0 , 1 , 'active'   , '2023-01-16 12:15:00' , '2024-03-07 08:30:00'),
('developer20@icloud.com'   , 1 , SHA2('password20', 256) , SHA2('salt20', 256) , '2023-10-21 12:15:00' , 0 , 1 , 1 , 'active'   , '2023-02-21 14:30:00' , '2024-03-08 10:45:00'),
('user21@example.com'       , 0 , SHA2('password21', 256) , SHA2('salt21', 256) , '2023-11-15 08:30:00' , 3 , 0 , 1 , 'pending'  , '2022-05-12 14:20:00' , NULL),
('developer22@gmail.com'    , 1 , SHA2('password22', 256) , SHA2('salt22', 256) , '2023-12-20 10:45:00' , 1 , 1 , 0 , 'suspended' , '2022-06-14 16:40:00' , '2023-12-29 11:30:00'),
('admin23@company.com'      , 1 , SHA2('password23', 256) , SHA2('salt23', 256) , '2024-01-25 12:15:00' , 0 , 1 , 1 , 'active'   , '2022-07-17 09:30:00' , '2024-03-01 08:45:00'),
('test24@techops.co'        , 1 , SHA2('password24', 256) , SHA2('salt24', 256) , '2024-02-10 14:20:00' , 0 , 0 , 1 , 'active'   , '2022-08-22 11:10:00' , '2024-03-02 10:20:00'),
('user25@yahoo.com'         , 1 , SHA2('password25', 256) , SHA2('salt25', 256) , '2024-03-15 09:30:00' , 0 , 0 , 1 , 'active'   , '2022-09-27 15:45:00' , '2024-03-03 14:30:00');

INSERT INTO users (id, email, email_verified, password, salt, active, status, created_at)
VALUES
(@USER_COUNT + 1 , 'admin@cloudplatform.io'   , 1 , SHA2('admin_password_hash', 256)   , SHA2('admin_salt', 256)   , 1 , 'active' , '2022-01-01 00:00:00'),
(@USER_COUNT + 2 , 'support@cloudplatform.io' , 1 , SHA2('support_password_hash', 256) , SHA2('support_salt', 256) , 1 , 'active' , '2022-01-01 00:00:00'),
(@USER_COUNT + 3 , 'billing@cloudplatform.io' , 1 , SHA2('billing_password_hash', 256) , SHA2('billing_salt', 256) , 1 , 'active' , '2022-01-01 00:00:00');

INSERT INTO user_meta (user_id, timezone, language, theme, notification_preferences, profile_image, dashboard_layout, onboarding_completed)
VALUES
(1 , 'UTC'              , 'en' , 'light'  , JSON_OBJECT('email', 1, 'push', 1, 'deployment', 1, 'billing', 1, 'marketing', 0) , 'https://example.com/profile/1.jpg' , JSON_OBJECT('widgets', JSON_ARRAY('deployments', 'metrics', 'instances')) , 1),
(2 , 'America/New_York' , 'en' , 'dark'   , JSON_OBJECT('email', 1, 'push', 0, 'deployment', 1, 'billing', 1, 'marketing', 0) , 'https://example.com/profile/2.jpg' , NULL                                                                      , 1),
(3 , 'Europe/London'    , 'en' , 'system' , JSON_OBJECT('email', 1, 'push', 1, 'deployment', 1, 'billing', 1, 'marketing', 1) , 'https://example.com/profile/3.jpg' , NULL                                                                      , 1),
(4 , 'Asia/Tokyo'       , 'ja' , 'light'  , JSON_OBJECT('email', 1, 'push', 0, 'deployment', 1, 'billing', 1, 'marketing', 0) , NULL                                , NULL                                                                      , 1),
(5 , 'Australia/Sydney' , 'en' , 'dark'   , JSON_OBJECT('email', 1, 'push', 1, 'deployment', 1, 'billing', 1, 'marketing', 0) , 'https://example.com/profile/5.jpg' , JSON_OBJECT('widgets', JSON_ARRAY('deployments', 'metrics'))              , 1);
INSERT INTO user_pii (user_id, first_name, last_name, full_name, identity_verified)
VALUES
(1 , 'John'    , 'Smith'    , 'John Smith'       , 1),
(2 , 'Jane'    , 'Johnson'  , 'Jane Johnson'     , 1),
(3 , 'Michael' , 'Williams' , 'Michael Williams' , 1),
(4 , 'Sarah'   , 'Jones'    , 'Sarah Jones'      , 0),
(5 , 'David'   , 'Brown'    , 'David Brown'      , 1);

INSERT INTO user_sessions (user_id, session_token, refresh_token, ip_address, user_agent, device_info, location_info, is_active, last_activity, expires_at)
VALUES
(1  , SHA2('session1', 256)  , SHA2('refresh1', 256)  , '192.168.1.1'  , 'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.114 Safari/537.36'               , JSON_OBJECT('type', 'desktop', 'os', 'macOS')   , JSON_OBJECT('city', 'New York', 'country', 'United States')    , 1 , NOW() - INTERVAL 1 HOUR  , NOW() + INTERVAL 2 DAY),
(2  , SHA2('session2', 256)  , SHA2('refresh2', 256)  , '192.168.1.2'  , 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.114 Safari/537.36'                     , JSON_OBJECT('type', 'desktop', 'os', 'Windows') , JSON_OBJECT('city', 'Los Angeles', 'country', 'United States') , 1 , NOW() - INTERVAL 2 HOUR  , NOW() + INTERVAL 3 DAY),
(3  , SHA2('session3', 256)  , SHA2('refresh3', 256)  , '192.168.1.3'  , 'Mozilla/5.0 (iPhone; CPU iPhone OS 14_6 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/14.0 Mobile/15E148 Safari/604.1' , JSON_OBJECT('type', 'mobile', 'os', 'iOS')      , JSON_OBJECT('city', 'Chicago', 'country', 'United States')     , 1 , NOW() - INTERVAL 3 HOUR  , NOW() + INTERVAL 1 DAY),
(5  , SHA2('session5', 256)  , SHA2('refresh5', 256)  , '192.168.1.5'  , 'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.114 Safari/537.36'               , JSON_OBJECT('type', 'desktop', 'os', 'macOS')   , JSON_OBJECT('city', 'Phoenix', 'country', 'United States')     , 1 , NOW() - INTERVAL 5 HOUR  , NOW() + INTERVAL 5 DAY),
(7  , SHA2('session7', 256)  , SHA2('refresh7', 256)  , '192.168.1.7'  , 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.114 Safari/537.36'                     , JSON_OBJECT('type', 'desktop', 'os', 'Windows') , JSON_OBJECT('city', 'San Antonio', 'country', 'United States') , 1 , NOW() - INTERVAL 7 HOUR  , NOW() + INTERVAL 7 DAY),
(10 , SHA2('session10', 256) , SHA2('refresh10', 256) , '192.168.1.10' , 'Mozilla/5.0 (iPad; CPU OS 14_6 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/14.0 Mobile/15E148 Safari/604.1'          , JSON_OBJECT('type', 'tablet', 'os', 'iOS')      , JSON_OBJECT('city', 'San Jose', 'country', 'United States')    , 1 , NOW() - INTERVAL 10 HOUR , NOW() + INTERVAL 2 DAY);

INSERT INTO orgs (name, display_name, description, logo_url, website, billing_email, status, created_at)
VALUES
('techcloud'    , 'TechCloud Inc'          , 'Organization for TechCloud Inc'          , 'https://example.com/logos/techcloud.png'    , 'https://techcloud.com'    , 'billing@techcloud.com'    , 'active' , '2022-05-10 14:20:00'),
('datastack'    , 'DataStack Solutions'    , 'Organization for DataStack Solutions'    , 'https://example.com/logos/datastack.png'    , 'https://datastack.com'    , 'billing@datastack.com'    , 'active' , '2022-06-15 09:30:00'),
('codeworks'    , 'CodeWorks LLC'          , 'Organization for CodeWorks LLC'          , 'https://example.com/logos/codeworks.png'    , 'https://codeworks.com'    , 'billing@codeworks.com'    , 'active' , '2022-07-20 11:45:00'),
('devbit'       , 'DevBit Technologies'    , 'Organization for DevBit Technologies'    , 'https://example.com/logos/devbit.png'       , 'https://devbit.com'       , 'billing@devbit.com'       , 'active' , '2022-08-25 16:30:00'),
('aiplatform'   , 'AIPlatform Corp'        , 'Organization for AIPlatform Corp'        , NULL                                         , 'https://aiplatform.com'   , 'billing@aiplatform.com'   , 'active' , '2022-09-30 10:15:00'),
('networklogic' , 'NetworkLogic Systems'   , 'Organization for NetworkLogic Systems'   , 'https://example.com/logos/networklogic.png' , 'https://networklogic.com' , 'billing@networklogic.com' , 'active' , '2022-10-05 14:45:00'),
('webstack'     , 'WebStack Digital'       , 'Organization for WebStack Digital'       , NULL                                         , 'https://webstack.com'     , 'billing@webstack.com'     , 'active' , '2022-11-10 09:20:00'),
('cloudcompute' , 'CloudCompute Group'     , 'Organization for CloudCompute Group'     , 'https://example.com/logos/cloudcompute.png' , 'https://cloudcompute.com' , 'billing@cloudcompute.com' , 'active' , '2022-12-15 11:30:00'),
('softwaretech' , 'SoftwareTech Solutions' , 'Organization for SoftwareTech Solutions' , 'https://example.com/logos/softwaretech.png' , 'https://softwaretech.com' , 'billing@softwaretech.com' , 'active' , '2023-01-20 15:45:00'),
('mobileapps'   , 'MobileApps Team'        , 'Organization for MobileApps Team'        , NULL                                         , 'https://mobileapps.com'   , 'billing@mobileapps.com'   , 'active' , '2023-02-25 10:15:00');

INSERT INTO quotas (name, org_id, memory_limit, instance_limit, routes_limit, services_limit, cpu_limit, disk_limit)
VALUES
('devbit-quota'       , 4  , 32768 , 100 , 200 , 50 , 32 , 102400),
('webstack-quota'     , 7  , 4096  , 10  , 20  , 5  , 4  , 10240),
('techcloud-quota'    , 1  , 32768 , 100 , 200 , 50 , 32 , 102400),
('datastack-quota'    , 2  , 16384 , 50  , 100 , 20 , 16 , 51200),
('codeworks-quota'    , 3  , 8192  , 20  , 40  , 10 , 8  , 20480),
('mobileapps-quota'   , 10 , 8192  , 20  , 40  , 10 , 8  , 20480),
('aiplatform-quota'   , 5  , 16384 , 50  , 100 , 20 , 16 , 51200),
('networklogic-quota' , 6  , 8192  , 20  , 40  , 10 , 8  , 20480),
('cloudcompute-quota' , 8  , 32768 , 100 , 200 , 50 , 32 , 102400),
('softwaretech-quota' , 9  , 16384 , 50  , 100 , 20 , 16 , 51200);

INSERT INTO orgs (id, name, display_name, description, plan, status, created_at)
VALUES
(@ORG_COUNT + 1 , 'cloudplatform' , 'Cloud Platform, Inc.' , 'Official Cloud Platform Organization' , 'enterprise' , 'active' , '2020-01-01 00:00:00');

INSERT INTO orgmember (org_id, user_id, role, invitation_status)
VALUES
(1  , 1 , 'owner'  , 'accepted'),
(1  , 2 , 'admin'  , 'accepted'),
(1  , 3 , 'member' , 'accepted'),
(1  , 4 , 'member' , 'accepted'),
(2  , 5 , 'owner'  , 'accepted'),
(2  , 6 , 'admin'  , 'accepted'),
(2  , 7 , 'member' , 'accepted'),
(3  , 8 , 'owner'  , 'accepted'),
(3  , 9 , 'admin'  , 'accepted'),
(5  , 1 , 'member' , 'accepted'),
(6  , 2 , 'member' , 'accepted'),
(7  , 3 , 'member' , 'accepted'),
(8  , 4 , 'member' , 'accepted'),
(9  , 5 , 'member' , 'accepted'),
(10 , 6 , 'member' , 'accepted');

-- INSERT INTO orgmember (org_id, user_id, role, invitation_status)


INSERT INTO api_keys (org_id, user_id, name, key_hash, prefix, scopes)
VALUES
(1 , 1  , 'Default API Key' , SHA2('api_key1', 256) , SUBSTRING(MD5(RAND()), 1, 8) , JSON_ARRAY('app.read', 'app.write')),
(2 , 5  , 'Default API Key' , SHA2('api_key2', 256) , SUBSTRING(MD5(RAND()), 1, 8) , JSON_ARRAY('app.read', 'app.write')),
(3 , 8  , 'Default API Key' , SHA2('api_key3', 256) , SUBSTRING(MD5(RAND()), 1, 8) , JSON_ARRAY('app.read', 'app.write')),
(4 , 11 , 'Default API Key' , SHA2('api_key4', 256) , SUBSTRING(MD5(RAND()), 1, 8) , JSON_ARRAY('app.read', 'app.write')),
(5 , 14 , 'Default API Key' , SHA2('api_key5', 256) , SUBSTRING(MD5(RAND()), 1, 8) , JSON_ARRAY('app.read', 'app.write')),
(8 , 20 , 'Default API Key' , SHA2('api_key8', 256) , SUBSTRING(MD5(RAND()), 1, 8) , JSON_ARRAY('app.read', 'app.write')),
(9 , 22 , 'Default API Key' , SHA2('api_key9', 256) , SUBSTRING(MD5(RAND()), 1, 8) , JSON_ARRAY('app.read', 'app.write'));

-- INSERT INTO api_keys (org_id, user_id, name, key_hash, prefix, scopes)


INSERT INTO spaces (org_id, name, description, status, network_isolation, created_at)
VALUES
(1  , 'production'  , 'Space for production environment'  , 'active' , 1 , '2022-05-15 10:30:00'),
(1  , 'staging'     , 'Space for staging environment'     , 'active' , 0 , '2022-05-15 11:45:00'),
(1  , 'development' , 'Space for development environment' , 'active' , 0 , '2022-05-15 14:20:00'),
(2  , 'production'  , 'Space for production environment'  , 'active' , 1 , '2022-06-20 09:15:00'),
(2  , 'staging'     , 'Space for staging environment'     , 'active' , 0 , '2022-06-20 10:30:00'),
(2  , 'development' , 'Space for development environment' , 'active' , 0 , '2022-06-20 13:45:00'),
(3  , 'production'  , 'Space for production environment'  , 'active' , 1 , '2022-07-25 11:30:00'),
(3  , 'testing'     , 'Space for testing environment'     , 'active' , 0 , '2022-07-25 14:15:00'),
(4  , 'production'  , 'Space for production environment'  , 'active' , 1 , '2022-08-30 10:45:00'),
(4  , 'staging'     , 'Space for staging environment'     , 'active' , 0 , '2022-08-30 13:30:00'),
(5  , 'production'  , 'Space for production environment'  , 'active' , 1 , '2022-10-05 09:45:00'),
(5  , 'development' , 'Space for development environment' , 'active' , 0 , '2022-10-05 11:15:00'),
(6  , 'production'  , 'Space for production environment'  , 'active' , 1 , '2022-11-10 14:30:00'),
(6  , 'staging'     , 'Space for staging environment'     , 'active' , 0 , '2022-11-10 16:45:00'),
(7  , 'development' , 'Space for development environment' , 'active' , 0 , '2022-12-15 10:15:00'),
(8  , 'production'  , 'Space for production environment'  , 'active' , 1 , '2023-01-20 13:45:00'),
(8  , 'staging'     , 'Space for staging environment'     , 'active' , 0 , '2023-01-20 15:30:00'),
(9  , 'production'  , 'Space for production environment'  , 'active' , 1 , '2023-02-25 09:30:00'),
(9  , 'development' , 'Space for development environment' , 'active' , 0 , '2023-02-25 11:45:00'),
(10 , 'development' , 'Space for development environment' , 'active' , 0 , '2023-03-01 14:20:00');

-- INSERT INTO spaces (org_id, name, description, status, created_at)


INSERT INTO workers (region_id, name, provider_id, instance_type, status, cpu_total, cpu_available, cpu_reserved, memory_total, memory_available, memory_reserved, disk_total, disk_available, disk_reserved, network_in_capacity, network_out_capacity, docker_version, labels, last_heartbeat, created_at)
VALUES
(1  , 'node-a1b2c3d4' , 'i-0123456789abcdef0' , 'c5.4xlarge' , 'active'       , 8  , 3  , 5  , 32768  , 12768 , 20000  , 512000  , 212000 , 300000  , 10000 , 10000 , '1.24.8' , JSON_OBJECT('role', 'worker', 'zone', 'zone-a')        , NOW() - INTERVAL 5 MINUTE   , '2022-01-15 10:30:00'),
(1  , 'node-e5f6g7h8' , 'i-1234567890abcdef1' , 'm5.8xlarge' , 'active'       , 16 , 6  , 10 , 65536  , 25536 , 40000  , 1024000 , 424000 , 600000  , 10000 , 10000 , '1.24.8' , JSON_OBJECT('role', 'control-plane', 'zone', 'zone-a') , NOW() - INTERVAL 3 MINUTE   , '2022-01-20 14:45:00'),
(2  , 'node-i9j0k1l2' , 'i-2345678901abcdef2' , 'r5.8xlarge' , 'active'       , 16 , 5  , 11 , 131072 , 51072 , 80000  , 1024000 , 324000 , 700000  , 10000 , 10000 , '1.25.3' , JSON_OBJECT('role', 'worker', 'zone', 'zone-b')        , NOW() - INTERVAL 8 MINUTE   , '2022-02-10 09:15:00'),
(2  , 'node-m3n4o5p6' , 'i-3456789012abcdef3' , 'c5.4xlarge' , 'active'       , 8  , 2  , 6  , 32768  , 8768  , 24000  , 512000  , 192000 , 320000  , 10000 , 10000 , '1.25.3' , JSON_OBJECT('role', 'worker', 'zone', 'zone-b')        , NOW() - INTERVAL 12 MINUTE  , '2022-02-15 11:30:00'),
(3  , 'node-q7r8s9t0' , 'i-4567890123abcdef4' , 'i3.8xlarge' , 'active'       , 32 , 12 , 20 , 131072 , 51072 , 80000  , 2048000 , 648000 , 1400000 , 10000 , 10000 , '1.25.3' , JSON_OBJECT('role', 'worker', 'zone', 'zone-c')        , NOW() - INTERVAL 7 MINUTE   , '2022-03-05 13:45:00'),
(3  , 'node-u1v2w3x4' , 'i-5678901234abcdef5' , 'm5.8xlarge' , 'active'       , 16 , 4  , 12 , 65536  , 15536 , 50000  , 1024000 , 324000 , 700000  , 10000 , 10000 , '1.26.1' , JSON_OBJECT('role', 'control-plane', 'zone', 'zone-c') , NOW() - INTERVAL 2 MINUTE   , '2022-03-10 15:20:00'),
(4  , 'node-y5z6a7b8' , 'i-6789012345abcdef6' , 'r5.8xlarge' , 'maintenance'  , 16 , 0  , 16 , 131072 , 0     , 131072 , 1024000 , 0      , 1024000 , 10000 , 10000 , '1.24.8' , JSON_OBJECT('role', 'worker', 'zone', 'zone-a')        , NOW() - INTERVAL 120 MINUTE , '2022-04-05 10:15:00'),
(4  , 'node-c9d0e1f2' , 'i-7890123456abcdef7' , 'c5.4xlarge' , 'active'       , 8  , 3  , 5  , 32768  , 12768 , 20000  , 512000  , 212000 , 300000  , 10000 , 10000 , '1.26.1' , JSON_OBJECT('role', 'worker', 'zone', 'zone-a')        , NOW() - INTERVAL 9 MINUTE   , '2022-04-10 14:30:00'),
(5  , 'node-g3h4i5j6' , 'i-8901234567abcdef8' , 'i3.8xlarge' , 'active'       , 32 , 10 , 22 , 131072 , 41072 , 90000  , 2048000 , 548000 , 1500000 , 10000 , 10000 , '1.26.1' , JSON_OBJECT('role', 'worker', 'zone', 'zone-b')        , NOW() - INTERVAL 4 MINUTE   , '2022-05-05 09:45:00'),
(5  , 'node-k7l8m9n0' , 'i-9012345678abcdef9' , 'm5.8xlarge' , 'active'       , 16 , 5  , 11 , 65536  , 25536 , 40000  , 1024000 , 424000 , 600000  , 10000 , 10000 , '1.26.1' , JSON_OBJECT('role', 'control-plane', 'zone', 'zone-b') , NOW() - INTERVAL 6 MINUTE   , '2022-05-10 11:15:00'),
(6  , 'node-s5t6u7v8' , 'i-1234567890abcdefb' , 'c5.4xlarge' , 'provisioning' , 8  , 8  , 0  , 32768  , 32768 , 0      , 512000  , 512000 , 0       , 10000 , 10000 , '1.26.1' , JSON_OBJECT('role', 'worker', 'zone', 'zone-c')        , NOW() - INTERVAL 25 MINUTE  , '2022-06-10 15:45:00'),
(6  , 'node-o1p2q3r4' , 'i-0123456789abcdefa' , 'r5.8xlarge' , 'active'       , 16 , 6  , 10 , 131072 , 51072 , 80000  , 1024000 , 424000 , 600000  , 10000 , 10000 , '1.24.8' , JSON_OBJECT('role', 'worker', 'zone', 'zone-c')        , NOW() - INTERVAL 11 MINUTE  , '2022-06-05 13:30:00'),
(7  , 'node-w9x0y1z2' , 'i-2345678901abcdefc' , 'i3.8xlarge' , 'active'       , 32 , 8  , 24 , 131072 , 31072 , 100000 , 2048000 , 448000 , 1600000 , 10000 , 10000 , '1.25.3' , JSON_OBJECT('role', 'worker', 'zone', 'zone-a')        , NOW() - INTERVAL 7 MINUTE   , '2022-07-05 10:30:00'),
(7  , 'node-a3b4c5d6' , 'i-3456789012abcdefd' , 'm5.8xlarge' , 'active'       , 16 , 4  , 12 , 65536  , 15536 , 50000  , 1024000 , 224000 , 800000  , 10000 , 10000 , '1.25.3' , JSON_OBJECT('role', 'control-plane', 'zone', 'zone-a') , NOW() - INTERVAL 5 MINUTE   , '2022-07-10 14:15:00'),
(8  , 'node-e7f8g9h0' , 'i-4567890123abcdefe' , 'r5.8xlarge' , 'active'       , 16 , 5  , 11 , 131072 , 51072 , 80000  , 1024000 , 324000 , 700000  , 10000 , 10000 , '1.26.1' , JSON_OBJECT('role', 'worker', 'zone', 'zone-b')        , NOW() - INTERVAL 8 MINUTE   , '2022-08-05 09:45:00'),
(8  , 'node-i1j2k3l4' , 'i-5678901234abcdeff' , 'c5.4xlarge' , 'degraded'      , 8  , 0  , 8  , 32768  , 0     , 32768  , 512000  , 0      , 512000  , 10000 , 10000 , '1.24.8' , JSON_OBJECT('role', 'worker', 'zone', 'zone-b')        , NOW() - INTERVAL 240 MINUTE , '2022-08-10 11:30:00'),
(9  , 'node-m5n6o7p8' , 'i-6789012345abcdefg' , 'i3.8xlarge' , 'active'       , 32 , 12 , 20 , 131072 , 51072 , 80000  , 2048000 , 648000 , 1400000 , 10000 , 10000 , '1.25.3' , JSON_OBJECT('role', 'worker', 'zone', 'zone-c')        , NOW() - INTERVAL 3 MINUTE   , '2022-09-05 13:45:00'),
(9  , 'node-q9r0s1t2' , 'i-7890123456abcdefh' , 'm5.8xlarge' , 'active'       , 16 , 6  , 10 , 65536  , 25536 , 40000  , 1024000 , 424000 , 600000  , 10000 , 10000 , '1.25.3' , JSON_OBJECT('role', 'control-plane', 'zone', 'zone-c') , NOW() - INTERVAL 4 MINUTE   , '2022-09-10 15:20:00'),
(1  , 'node-u3v4w5x6' , 'i-8901234567abcdefi' , 'r5.8xlarge' , 'active'       , 16 , 4  , 12 , 131072 , 31072 , 100000 , 1024000 , 224000 , 800000  , 10000 , 10000 , '1.26.1' , JSON_OBJECT('role', 'worker', 'zone', 'zone-a')        , NOW() - INTERVAL 6 MINUTE   , '2022-10-05 10:15:00'),
(1  , 'node-y7z8a9b0' , 'i-9012345678abcdefj' , 'c5.4xlarge' , 'active'       , 8  , 3  , 5  , 32768  , 12768 , 20000  , 512000  , 212000 , 300000  , 10000 , 10000 , '1.26.1' , JSON_OBJECT('role', 'worker', 'zone', 'zone-a')        , NOW() - INTERVAL 9 MINUTE   , '2022-10-10 14:30:00'),
(2  , 'node-g1h2i3j4' , 'i-0123456789abcdefk' , 'i3.8xlarge' , 'active'       , 32 , 10 , 22 , 131072 , 41072 , 90000  , 2048000 , 548000 , 1500000 , 10000 , 10000 , '1.26.1' , JSON_OBJECT('role', 'worker', 'zone', 'zone-b')        , NOW() - INTERVAL 2 MINUTE   , '2022-11-05 10:30:00'),
(2  , 'node-k5l6m7n8' , 'i-1234567890abcdefl' , 'm5.8xlarge' , 'active'       , 16 , 5  , 11 , 65536  , 25536 , 40000  , 1024000 , 424000 , 600000  , 10000 , 10000 , '1.26.1' , JSON_OBJECT('role', 'control-plane', 'zone', 'zone-b') , NOW() - INTERVAL 3 MINUTE   , '2022-11-10 11:15:00'),
(3  , 'node-s9t0u1v2' , 'i-2345678901abcdefm' , 'c5.4xlarge' , 'active'       , 8  , 2  , 6  , 32768  , 8768  , 24000  , 512000  , 192000 , 320000  , 10000 , 10000 , '1.26.1' , JSON_OBJECT('role', 'worker', 'zone', 'zone-c')        , NOW() - INTERVAL 7 MINUTE   , '2022-12-05 09:45:00'),
(3  , 'node-a3b4c5d6' , 'i-3456789012abcdefn' , 'r5.8xlarge' , 'active'       , 16 , 6  , 10 , 131072 , 51072 , 80000  , 1024000 , 324000 , 700000  , 10000 , 10000 , '1.26.1' , JSON_OBJECT('role', 'worker', 'zone', 'zone-c')        , NOW() - INTERVAL 4 MINUTE   , '2022-12-10 14:15:00'),
(4  , 'node-e7f8g9h0' , 'i-4567890123abcdefo' , 'i3.8xlarge' , 'active'       , 32 , 12 , 20 , 131072 , 51072 , 80000  , 2048000 , 648000 , 1400000 , 10000 , 10000 , '1.26.1' , JSON_OBJECT('role', 'worker', 'zone', 'zone-a')        , NOW() - INTERVAL 5 MINUTE   , '2023-01-05 10:30:00'),
(4  , 'node-i1j2k3l4' , 'i-5678901234abcdefp' , 'm5.8xlarge' , 'active'       , 16 , 4  , 12 , 65536  , 15536 , 50000  , 1024000 , 224000 , 800000  , 10000 , 10000 , '1.26.1' , JSON_OBJECT('role', 'control-plane', 'zone', 'zone-a') , NOW() - INTERVAL 6 MINUTE   , '2023-01-10 11:15:00'),
(5  , 'node-m5n6o7p8' , 'i-6789012345abcdefq' , 'r5.8xlarge' , 'active'       , 16 , 6  , 10 , 131072 , 51072 , 80000  , 1024000 , 324000 , 700000  , 10000 , 10000 , '1.26.1' , JSON_OBJECT('role', 'worker', 'zone', 'zone-b')        , NOW() - INTERVAL 7 MINUTE   , '2023-02-05 09:45:00'),
(5  , 'node-q9r0s1t2' , 'i-7890123456abcdefr' , 'm5.8xlarge' , 'active'       , 16 , 4  , 12 , 65536  , 15536 , 50000  , 1024000 , 224000 , 800000  , 10000 , 10000 , '1.26.1' , JSON_OBJECT('role', 'control-plane', 'zone', 'zone-b') , NOW() - INTERVAL 8 MINUTE   , '2023-02-10 11:15:00');

INSERT INTO data_services (region_id, name, display_name, service_type, service_subtype, status, version, plan, tier, is_highly_available, backup_enabled, backup_retention_days, encryption_at_rest, created_at)
VALUES
(1 , 'mysql-a1b2c3d4'         , 'MySQL Database a1b2'         , 'database'           , 'mysql'         , 'active'       , '8.0'  , 'standard' , 'premium'  , 1 , 1 , 30 , 1 , '2022-02-15 10:30:00'),
(1 , 'redis-e5f6g7h8'         , 'Redis Cache e5f6'            , 'cache'              , 'redis'         , 'active'       , '6.2'  , 'basic'    , 'standard' , 0 , 1 , 7  , 1 , '2022-02-20 14:45:00'),
(2 , 'postgres-i9j0k1l2'      , 'PostgreSQL Database i9j0'    , 'database'           , 'postgres'      , 'active'       , '15.3' , 'premium'  , 'premium'  , 1 , 1 , 30 , 1 , '2022-03-10 09:15:00'),
(2 , 'mongodb-m3n4o5p6'       , 'MongoDB Database m3n4'       , 'database'           , 'mongodb'       , 'active'       , '6.0'  , 'standard' , 'standard' , 1 , 1 , 14 , 1 , '2022-03-15 11:30:00'),
(3 , 'rabbitmq-q7r8s9t0'      , 'RabbitMQ Message Queue q7r8' , 'message_queue'      , 'rabbitmq'      , 'active'       , '3.8'  , 'standard' , 'standard' , 1 , 1 , 7  , 1 , '2022-04-05 13:45:00'),
(3 , 'kafka-u1v2w3x4'         , 'Kafka Message Queue u1v2'    , 'message_queue'      , 'kafka'         , 'active'       , '3.5'  , 'premium'  , 'premium'  , 1 , 1 , 14 , 1 , '2022-04-10 15:20:00'),
(4 , 'elasticsearch-y5z6a7b8' , 'Elasticsearch Service y5z6'  , 'search'             , 'elasticsearch' , 'active'       , '7.10' , 'standard' , 'standard' , 1 , 1 , 7  , 1 , '2022-05-05 10:15:00'),
(4 , 'mysql-c9d0e1f2'         , 'MySQL Database c9d0'         , 'database'           , 'mysql'         , 'active'       , '8.2'  , 'basic'    , 'free'     , 0 , 1 , 7  , 1 , '2022-05-10 14:30:00'),
(5 , 'postgres-g3h4i5j6'      , 'PostgreSQL Database g3h4'    , 'database'           , 'postgres'      , 'provisioning' , '15.3' , 'standard' , 'standard' , 1 , 1 , 14 , 1 , '2022-06-05 09:45:00'),
(5 , 'redis-k7l8m9n0'         , 'Redis Cache k7l8'            , 'cache'              , 'redis'         , 'active'       , '6.2'  , 'premium'  , 'premium'  , 1 , 1 , 7  , 1 , '2022-06-10 11:15:00'),
(6 , 'nfs-o1p2q3r4'           , 'Network File System o1p2'    , 'network_filesystem' , 'nfs'           , 'active'       , NULL   , 'standard' , 'standard' , 1 , 1 , 30 , 1 , '2022-07-05 13:30:00'),
(6 , 'mongodb-s5t6u7v8'       , 'MongoDB Database s5t6'       , 'database'           , 'mongodb'       , 'maintenance'  , '6.0'  , 'premium'  , 'premium'  , 1 , 1 , 30 , 1 , '2022-07-10 15:45:00'),
(7 , 'sqs-w9x0y1z2'           , 'SQS Message Queue w9x0'      , 'message_queue'      , 'sqs'           , 'active'       , NULL   , 'basic'    , 'free'     , 0 , 0 , 0  , 1 , '2022-08-05 10:30:00'),
(8 , 'mysql-a3b4c5d6'         , 'MySQL Database a3b4'         , 'database'           , 'mysql'         , 'active'       , '5.7'  , 'standard' , 'standard' , 1 , 1 , 14 , 1 , '2022-08-10 14:15:00'),
(9 , 'elasticsearch-e7f8g9h0' , 'Elasticsearch Service e7f8'  , 'search'             , 'elasticsearch' , 'active'       , '7.10' , 'premium'  , 'premium'  , 1 , 1 , 30 , 1 , '2022-09-05 09:45:00');

INSERT INTO domains (org_id, name, domain_type, ssl_enabled, ssl_expiry_date, auto_renew, verified, verification_status, created_at)
VALUES
(1    , 'techcloud.com'             , 'private' , 1 , DATE_ADD(CURRENT_DATE(), INTERVAL 180 DAY) , 1 , 1 , 'verified' , '2022-05-15 10:30:00'),
(1    , 'api.techcloud.com'         , 'private' , 1 , DATE_ADD(CURRENT_DATE(), INTERVAL 180 DAY) , 1 , 1 , 'verified' , '2022-05-15 11:45:00'),
(2    , 'datastack.com'             , 'private' , 1 , DATE_ADD(CURRENT_DATE(), INTERVAL 240 DAY) , 1 , 1 , 'verified' , '2022-06-20 09:15:00'),
(3    , 'codeworks.com'             , 'private' , 1 , DATE_ADD(CURRENT_DATE(), INTERVAL 150 DAY) , 1 , 1 , 'verified' , '2022-07-25 11:30:00'),
(3    , 'app.codeworks.com'         , 'private' , 1 , DATE_ADD(CURRENT_DATE(), INTERVAL 150 DAY) , 1 , 1 , 'verified' , '2022-07-25 14:15:00'),
(4    , 'devbit.com'                , 'private' , 1 , DATE_ADD(CURRENT_DATE(), INTERVAL 210 DAY) , 1 , 1 , 'verified' , '2022-08-30 10:45:00'),
(5    , 'aiplatform.com'            , 'private' , 1 , DATE_ADD(CURRENT_DATE(), INTERVAL 270 DAY) , 1 , 1 , 'verified' , '2022-10-05 09:45:00'),
(6    , 'networklogic.com'          , 'private' , 1 , DATE_ADD(CURRENT_DATE(), INTERVAL 120 DAY) , 1 , 1 , 'verified' , '2022-11-10 14:30:00'),
(7    , 'webstack.com'              , 'private' , 0 , NULL                                       , 1 , 1 , 'verified' , '2022-12-15 10:15:00'),
(8    , 'cloudcompute.com'          , 'private' , 1 , DATE_ADD(CURRENT_DATE(), INTERVAL 300 DAY) , 1 , 1 , 'verified' , '2023-01-20 13:45:00'),
(9    , 'softwaretech.com'          , 'private' , 1 , DATE_ADD(CURRENT_DATE(), INTERVAL 90 DAY)  , 1 , 0 , 'pending'  , '2023-02-25 09:30:00'),
(10   , 'mobileapps.com'            , 'private' , 1 , DATE_ADD(CURRENT_DATE(), INTERVAL 330 DAY) , 1 , 1 , 'verified' , '2023-03-01 14:20:00'),
(NULL , 'cloud-platform-a1b2c3.io'  , 'system'  , 1 , DATE_ADD(CURRENT_DATE(), INTERVAL 365 DAY) , 1 , 1 , 'verified' , '2022-01-01 00:00:00'),
(NULL , 'cloud-platform-d4e5f6.app' , 'system'  , 1 , DATE_ADD(CURRENT_DATE(), INTERVAL 365 DAY) , 1 , 1 , 'verified' , '2022-01-01 00:00:00'),
(NULL , 'cloud-platform-g7h8i9.dev' , 'system'  , 1 , DATE_ADD(CURRENT_DATE(), INTERVAL 365 DAY) , 1 , 1 , 'verified' , '2022-01-01 00:00:00');

INSERT INTO domains (name, domain_type, ssl_enabled, verified, verification_status, created_at)
VALUES
('cloudplatform.io'      , 'system' , 1 , 1 , 'verified' , '2020-01-01 00:00:00'),
('apps.cloudplatform.io' , 'system' , 1 , 1 , 'verified' , '2020-01-01 00:00:00'),
('api.cloudplatform.io'  , 'system' , 1 , 1 , 'verified' , '2020-01-01 00:00:00');

INSERT INTO apps (name, display_name, org_id, space_id, git_repo, git_branch, container_image_url, default_allocation_id, region_id, instances, health_check_type, health_check_endpoint, runtime, restart_policy, maintenance_mode, status, auto_scaling_enabled, labels, created_at)
VALUES
('api-backend'    , 'Api Backend'    , 1 , 1  , 'https://github.com/org1/api-backend'    , 'main'    , 'registry.example.com/org1/api-backend:latest'    , 5 , 1 , 3 , 'http'    , '/health' , 'nodejs' , 'always' , 0 , 'started'     , 1 , JSON_OBJECT('environment', 'production', 'team', 'backend')   , '2022-05-20 09:30:00'),
('web-frontend'   , 'Web Frontend'   , 1 , 1  , 'https://github.com/org1/web-frontend'   , 'main'    , 'registry.example.com/org1/web-frontend:latest'   , 4 , 1 , 3 , 'http'    , '/health' , 'nodejs' , 'always' , 0 , 'started'     , 1 , JSON_OBJECT('environment', 'production', 'team', 'frontend')  , '2022-05-20 11:45:00'),
('auth-service'   , 'Auth Service'   , 1 , 1  , 'https://github.com/org1/auth-service'   , 'main'    , 'registry.example.com/org1/auth-service:latest'   , 5 , 1 , 2 , 'http'    , '/health' , 'nodejs' , 'always' , 0 , 'started'     , 0 , JSON_OBJECT('environment', 'production', 'team', 'backend')   , '2022-05-20 14:15:00'),
('worker-jobs'    , 'Worker Jobs'    , 1 , 1  , 'https://github.com/org1/worker-jobs'    , 'main'    , 'registry.example.com/org1/worker-jobs:latest'    , 6 , 1 , 2 , 'process' , NULL      , 'nodejs' , 'always' , 0 , 'started'     , 0 , JSON_OBJECT('environment', 'production', 'team', 'backend')   , '2022-05-21 10:30:00'),
('api-staging'    , 'Api Staging'    , 1 , 2  , 'https://github.com/org1/api-backend'    , 'develop' , NULL                                              , 3 , 1 , 1 , 'http'    , '/health' , 'nodejs' , 'always' , 0 , 'started'     , 0 , JSON_OBJECT('environment', 'staging', 'team', 'backend')      , '2022-05-21 13:45:00'),
('web-staging'    , 'Web Staging'    , 1 , 2  , 'https://github.com/org1/web-frontend'   , 'develop' , NULL                                              , 3 , 1 , 1 , 'http'    , '/health' , 'nodejs' , 'always' , 0 , 'started'     , 0 , JSON_OBJECT('environment', 'staging', 'team', 'frontend')     , '2022-05-21 15:20:00'),
('api-dev'        , 'Api Dev'        , 1 , 3  , 'https://github.com/org1/api-backend'    , 'develop' , NULL                                              , 2 , 1 , 1 , 'http'    , '/health' , 'nodejs' , 'always' , 0 , 'started'     , 0 , JSON_OBJECT('environment', 'development', 'team', 'backend')  , '2022-05-22 09:15:00'),
('web-dev'        , 'Web Dev'        , 1 , 3  , 'https://github.com/org1/web-frontend'   , 'develop' , NULL                                              , 2 , 1 , 1 , 'http'    , '/health' , 'nodejs' , 'always' , 0 , 'started'     , 0 , JSON_OBJECT('environment', 'development', 'team', 'frontend') , '2022-05-22 11:30:00'),
('data-analytics' , 'Data Analytics' , 2 , 4  , 'https://github.com/org2/data-analytics' , 'main'    , 'registry.example.com/org2/data-analytics:latest' , 7 , 2 , 2 , 'http'    , '/status' , 'python' , 'always' , 0 , 'started'     , 1 , JSON_OBJECT('environment', 'production', 'team', 'data')      , '2022-06-25 10:45:00'),
('ml-processor'   , 'Ml Processor'   , 2 , 4  , 'https://github.com/org2/ml-processor'   , 'main'    , 'registry.example.com/org2/ml-processor:latest'   , 8 , 2 , 2 , 'http'    , '/status' , 'python' , 'always' , 0 , 'started'     , 0 , JSON_OBJECT('environment', 'production', 'team', 'data')      , '2022-06-25 14:30:00'),
('api-gateway'    , 'Api Gateway'    , 2 , 4  , 'https://github.com/org2/api-gateway'    , 'main'    , 'registry.example.com/org2/api-gateway:latest'    , 6 , 2 , 2 , 'http'    , '/health' , 'go'     , 'always' , 0 , 'started'     , 1 , JSON_OBJECT('environment', 'production', 'team', 'backend')   , '2022-06-26 09:15:00'),
('data-staging'   , 'Data Staging'   , 2 , 5  , 'https://github.com/org2/data-analytics' , 'develop' , NULL                                              , 4 , 2 , 1 , 'http'    , '/status' , 'python' , 'always' , 0 , 'started'     , 0 , JSON_OBJECT('environment', 'staging', 'team', 'data')         , '2022-06-26 11:45:00'),
('api-staging'    , 'Api Staging'    , 2 , 5  , 'https://github.com/org2/api-gateway'    , 'develop' , NULL                                              , 4 , 2 , 1 , 'http'    , '/health' , 'go'     , 'always' , 0 , 'started'     , 0 , JSON_OBJECT('environment', 'staging', 'team', 'backend')      , '2022-06-26 15:20:00'),
('data-dev'       , 'Data Dev'       , 2 , 6  , 'https://github.com/org2/data-analytics' , 'develop' , NULL                                              , 3 , 2 , 1 , 'http'    , '/status' , 'python' , 'always' , 0 , 'started'     , 0 , JSON_OBJECT('environment', 'development', 'team', 'data')     , '2022-06-27 10:30:00'),
('api-dev'        , 'Api Dev'        , 2 , 6  , 'https://github.com/org2/api-gateway'    , 'develop' , NULL                                              , 3 , 2 , 1 , 'http'    , '/health' , 'go'     , 'always' , 0 , 'started'     , 0 , JSON_OBJECT('environment', 'development', 'team', 'backend')  , '2022-06-27 14:15:00'),
('code-service'   , 'Code Service'   , 3 , 7  , 'https://github.com/org3/code-service'   , 'main'    , 'registry.example.com/org3/code-service:latest'   , 5 , 3 , 2 , 'http'    , '/health' , 'java'   , 'always' , 0 , 'started'     , 0 , JSON_OBJECT('environment', 'production', 'team', 'backend')   , '2022-07-30 09:45:00'),
('web-portal'     , 'Web Portal'     , 3 , 7  , 'https://github.com/org3/web-portal'     , 'main'    , 'registry.example.com/org3/web-portal:latest'     , 4 , 3 , 2 , 'http'    , '/health' , 'nodejs' , 'always' , 0 , 'started'     , 1 , JSON_OBJECT('environment', 'production', 'team', 'frontend')  , '2022-07-30 13:30:00'),
('worker-tasks'   , 'Worker Tasks'   , 3 , 7  , 'https://github.com/org3/worker-tasks'   , 'main'    , 'registry.example.com/org3/worker-tasks:latest'   , 5 , 3 , 2 , 'process' , NULL      , 'python' , 'always' , 0 , 'started'     , 0 , JSON_OBJECT('environment', 'production', 'team', 'backend')   , '2022-07-31 10:15:00'),
('code-testing'   , 'Code Testing'   , 3 , 8  , 'https://github.com/org3/code-service'   , 'develop' , NULL                                              , 3 , 3 , 1 , 'http'    , '/health' , 'java'   , 'always' , 0 , 'stopped'     , 0 , JSON_OBJECT('environment', 'testing', 'team', 'backend')      , '2022-07-31 14:45:00'),
('web-testing'    , 'Web Testing'    , 3 , 8  , 'https://github.com/org3/web-portal'     , 'develop' , NULL                                              , 3 , 3 , 1 , 'http'    , '/health' , 'nodejs' , 'always' , 0 , 'started'     , 0 , JSON_OBJECT('environment', 'testing', 'team', 'frontend')     , '2022-08-01 09:30:00'),
('dev-platform'   , 'Dev Platform'   , 4 , 9  , 'https://github.com/org4/dev-platform'   , 'main'    , 'registry.example.com/org4/dev-platform:latest'   , 6 , 4 , 3 , 'http'    , '/health' , 'nodejs' , 'always' , 0 , 'started'     , 1 , JSON_OBJECT('environment', 'production', 'team', 'platform')  , '2022-09-05 10:45:00'),
('api-service'    , 'Api Service'    , 4 , 9  , 'https://github.com/org4/api-service'    , 'main'    , 'registry.example.com/org4/api-service:latest'    , 5 , 4 , 2 , 'http'    , '/health' , 'nodejs' , 'always' , 0 , 'started'     , 1 , JSON_OBJECT('environment', 'production', 'team', 'backend')   , '2022-09-05 14:20:00'),
('mobile-backend' , 'Mobile Backend' , 4 , 9  , 'https://github.com/org4/mobile-backend' , 'main'    , 'registry.example.com/org4/mobile-backend:latest' , 5 , 4 , 2 , 'http'    , '/status' , 'java'   , 'always' , 0 , 'started'     , 0 , JSON_OBJECT('environment', 'production', 'team', 'mobile')    , '2022-09-06 09:30:00'),
('dev-staging'    , 'Dev Staging'    , 4 , 10 , 'https://github.com/org4/dev-platform'   , 'develop' , NULL                                              , 4 , 4 , 1 , 'http'    , '/health' , 'nodejs' , 'always' , 0 , 'started'     , 0 , JSON_OBJECT('environment', 'staging', 'team', 'platform')     , '2022-09-06 11:45:00'),
('api-staging'    , 'Api Staging'    , 4 , 10 , 'https://github.com/org4/api-service'    , 'develop' , NULL                                              , 4 , 4 , 1 , 'http'    , '/health' , 'nodejs' , 'always' , 1 , 'crashed' , 0 , JSON_OBJECT('environment', 'staging', 'team', 'backend')      , '2022-09-06 15:20:00'),
('ai-engine'      , 'Ai Engine'      , 5 , 11 , 'https://github.com/org5/ai-engine'      , 'main'    , 'registry.example.com/org5/ai-engine:latest'      , 9 , 5 , 3 , 'http'    , '/health' , 'python' , 'always' , 0 , 'started'     , 1 , JSON_OBJECT('environment', 'production', 'team', 'data')      , '2022-10-10 09:15:00'),
('data-processor' , 'Data Processor' , 5 , 11 , 'https://github.com/org5/data-processor' , 'main'    , 'registry.example.com/org5/data-processor:latest' , 8 , 5 , 3 , 'http'    , '/status' , 'python' , 'always' , 0 , 'started'     , 1 , JSON_OBJECT('environment', 'production', 'team', 'data')      , '2022-10-10 11:30:00'),
('web-interface'  , 'Web Interface'  , 5 , 11 , 'https://github.com/org5/web-interface'  , 'main'    , 'registry.example.com/org5/web-interface:latest'  , 6 , 5 , 2 , 'http'    , '/health' , 'nodejs' , 'always' , 0 , 'started'     , 0 , JSON_OBJECT('environment', 'production', 'team', 'frontend')  , '2022-10-11 13:45:00'),
('ai-dev'         , 'Ai Dev'         , 5 , 12 , 'https://github.com/org5/ai-engine'      , 'develop' , NULL                                              , 4 , 5 , 1 , 'http'    , '/health' , 'python' , 'always' , 0 , 'started'     , 0 , JSON_OBJECT('environment', 'development', 'team', 'data')     , '2022-10-11 15:20:00'),
('data-dev'       , 'Data Dev'       , 5 , 12 , 'https://github.com/org5/data-processor' , 'develop' , NULL                                              , 4 , 5 , 1 , 'http'    , '/status' , 'python' , 'always' , 0 , 'started'     , 0 , JSON_OBJECT('environment', 'development', 'team', 'data')     , '2022-10-12 10:30:00');

INSERT INTO apps (name, display_name, org_id, space_id, git_repo, git_branch, container_image_url, default_allocation_id, region_id, instances, health_check_type, health_check_endpoint, runtime, restart_policy, maintenance_mode, status, auto_scaling_enabled, labels, created_at)
VALUES
('network-service'    , 'Network Service'    , 6  , 13 , 'https://github.com/org6/network-service'    , 'main'    , 'registry.example.com/org6/network-service:latest'    , 5 , 6  , 2 , 'http' , '/health' , 'go'     , 'always' , 0 , 'started' , 0 , JSON_OBJECT('environment', 'production', 'team', 'platform')  , '2022-11-15 09:45:00'),
('routing-engine'     , 'Routing Engine'     , 6  , 13 , 'https://github.com/org6/routing-engine'     , 'main'    , 'registry.example.com/org6/routing-engine:latest'     , 6 , 6  , 2 , 'http' , '/status' , 'go'     , 'always' , 0 , 'started' , 1 , JSON_OBJECT('environment', 'production', 'team', 'platform')  , '2022-11-15 14:20:00'),
('admin-portal'       , 'Admin Portal'       , 6  , 13 , 'https://github.com/org6/admin-portal'       , 'main'    , 'registry.example.com/org6/admin-portal:latest'       , 4 , 6  , 1 , 'http' , '/health' , 'nodejs' , 'always' , 0 , 'started' , 0 , JSON_OBJECT('environment', 'production', 'team', 'frontend')  , '2022-11-16 10:15:00'),
('network-staging'    , 'Network Staging'    , 6  , 14 , 'https://github.com/org6/network-service'    , 'develop' , NULL                                                  , 3 , 6  , 1 , 'http' , '/health' , 'go'     , 'always' , 0 , 'started' , 0 , JSON_OBJECT('environment', 'staging', 'team', 'platform')     , '2022-11-16 13:30:00'),
('routing-staging'    , 'Routing Staging'    , 6  , 14 , 'https://github.com/org6/routing-engine'     , 'develop' , NULL                                                  , 3 , 6  , 1 , 'http' , '/status' , 'go'     , 'always' , 0 , 'started' , 0 , JSON_OBJECT('environment', 'staging', 'team', 'platform')     , '2022-11-16 15:45:00'),
('web-app'            , 'Web App'            , 7  , 15 , 'https://github.com/org7/web-app'            , 'develop' , NULL                                                  , 2 , 7  , 1 , 'http' , '/health' , 'nodejs' , 'always' , 0 , 'started' , 0 , JSON_OBJECT('environment', 'development', 'team', 'frontend') , '2022-12-20 10:30:00'),
('api-app'            , 'Api App'            , 7  , 15 , 'https://github.com/org7/api-app'            , 'develop' , NULL                                                  , 2 , 7  , 1 , 'http' , '/health' , 'nodejs' , 'always' , 0 , 'crashed' , 0 , JSON_OBJECT('environment', 'development', 'team', 'backend')  , '2022-12-20 14:15:00'),
('cloud-manager'      , 'Cloud Manager'      , 8  , 16 , 'https://github.com/org8/cloud-manager'      , 'main'    , 'registry.example.com/org8/cloud-manager:latest'      , 8 , 8  , 3 , 'http' , '/health' , 'java'   , 'always' , 0 , 'started' , 1 , JSON_OBJECT('environment', 'production', 'team', 'platform')  , '2023-01-25 09:45:00'),
('resource-api'       , 'Resource Api'       , 8  , 16 , 'https://github.com/org8/resource-api'       , 'main'    , 'registry.example.com/org8/resource-api:latest'       , 7 , 8  , 2 , 'http' , '/health' , 'java'   , 'always' , 0 , 'started' , 1 , JSON_OBJECT('environment', 'production', 'team', 'backend')   , '2023-01-25 13:30:00'),
('monitoring-service' , 'Monitoring Service' , 8  , 16 , 'https://github.com/org8/monitoring-service' , 'main'    , 'registry.example.com/org8/monitoring-service:latest' , 6 , 8  , 2 , 'http' , '/status' , 'go'     , 'always' , 0 , 'started' , 0 , JSON_OBJECT('environment', 'production', 'team', 'platform')  , '2023-01-26 10:15:00'),
('cloud-staging'      , 'Cloud Staging'      , 8  , 17 , 'https://github.com/org8/cloud-manager'      , 'develop' , NULL                                                  , 4 , 8  , 1 , 'http' , '/health' , 'java'   , 'always' , 0 , 'started' , 0 , JSON_OBJECT('environment', 'staging', 'team', 'platform')     , '2023-01-26 14:45:00'),
('resource-staging'   , 'Resource Staging'   , 8  , 17 , 'https://github.com/org8/resource-api'       , 'develop' , NULL                                                  , 4 , 8  , 1 , 'http' , '/health' , 'java'   , 'always' , 0 , 'stopped' , 0 , JSON_OBJECT('environment', 'staging', 'team', 'backend')      , '2023-01-27 09:30:00'),
('software-suite'     , 'Software Suite'     , 9  , 18 , 'https://github.com/org9/software-suite'     , 'main'    , 'registry.example.com/org9/software-suite:latest'     , 6 , 9  , 2 , 'http' , '/health' , 'ruby'   , 'always' , 0 , 'started' , 0 , JSON_OBJECT('environment', 'production', 'team', 'backend')   , '2023-03-05 10:45:00'),
('crm-system'         , 'Crm System'         , 9  , 18 , 'https://github.com/org9/crm-system'         , 'main'    , 'registry.example.com/org9/crm-system:latest'         , 5 , 9  , 2 , 'http' , '/status' , 'ruby'   , 'always' , 0 , 'started' , 1 , JSON_OBJECT('environment', 'production', 'team', 'backend')   , '2023-03-05 14:20:00'),
('customer-portal'    , 'Customer Portal'    , 9  , 18 , 'https://github.com/org9/customer-portal'    , 'main'    , 'registry.example.com/org9/customer-portal:latest'    , 4 , 9  , 2 , 'http' , '/health' , 'nodejs' , 'always' , 0 , 'started' , 0 , JSON_OBJECT('environment', 'production', 'team', 'frontend')  , '2023-03-06 09:30:00'),
('software-dev'       , 'Software Dev'       , 9  , 19 , 'https://github.com/org9/software-suite'     , 'develop' , NULL                                                  , 3 , 9  , 1 , 'http' , '/health' , 'ruby'   , 'always' , 0 , 'started' , 0 , JSON_OBJECT('environment', 'development', 'team', 'backend')  , '2023-03-06 11:45:00'),
('crm-dev'            , 'Crm Dev'            , 9  , 19 , 'https://github.com/org9/crm-system'         , 'develop' , NULL                                                  , 3 , 9  , 1 , 'http' , '/status' , 'ruby'   , 'always' , 0 , 'started' , 0 , JSON_OBJECT('environment', 'development', 'team', 'backend')  , '2023-03-06 15:20:00'),
('mobile-app'         , 'Mobile App'         , 10 , 20 , 'https://github.com/org10/mobile-app'        , 'develop' , NULL                                                  , 3 , 10 , 1 , 'http' , '/health' , 'nodejs' , 'always' , 0 , 'started' , 0 , JSON_OBJECT('environment', 'development', 'team', 'mobile')   , '2023-03-10 09:15:00'),
('api-service'        , 'Api Service'        , 10 , 20 , 'https://github.com/org10/api-service'       , 'develop' , NULL                                                  , 3 , 10 , 1 , 'http' , '/health' , 'nodejs' , 'always' , 0 , 'stopped' , 0 , JSON_OBJECT('environment', 'development', 'team', 'backend')  , '2023-03-10 11:30:00');

INSERT INTO health_checks (app_id, type, endpoint, timeout, check_interval, healthy_threshold, unhealthy_threshold)
VALUES 
(1  , 'http'    , '/health' , 30 , 10 , 2 , 3),
(2  , 'http'    , '/health' , 30 , 15 , 2 , 3),
(3  , 'http'    , '/health' , 40 , 10 , 2 , 2),
(4  , 'process' , NULL      , 60 , 20 , 1 , 3),
(5  , 'http'    , '/health' , 30 , 15 , 2 , 3),
(6  , 'http'    , '/health' , 30 , 15 , 2 , 3),
(7  , 'http'    , '/health' , 30 , 15 , 2 , 3),
(8  , 'http'    , '/health' , 30 , 15 , 2 , 3),
(9  , 'http'    , '/status' , 45 , 20 , 2 , 3),
(10 , 'http'    , '/status' , 45 , 20 , 2 , 3);

INSERT INTO autoscaling_rules (app_id, name, min_instances, max_instances, metric_type, threshold_value, threshold_unit, comparison_operator, evaluation_periods, period_seconds, cooldown_period_seconds)
VALUES
(1  , 'CPU Utilization'    , 2 , 5 , 'cpu'    , 75 , 'Percent' , 'GreaterThanOrEqualToThreshold' , 2 , 60 , 300),
(2  , 'CPU Utilization'    , 2 , 6 , 'cpu'    , 80 , 'Percent' , 'GreaterThanOrEqualToThreshold' , 2 , 60 , 300),
(9  , 'Memory Utilization' , 1 , 4 , 'memory' , 80 , 'Percent' , 'GreaterThanOrEqualToThreshold' , 2 , 60 , 300),
(22 , 'CPU Utilization'    , 1 , 5 , 'cpu'    , 75 , 'Percent' , 'GreaterThanOrEqualToThreshold' , 2 , 60 , 300),
(27 , 'CPU Utilization'    , 2 , 6 , 'cpu'    , 70 , 'Percent' , 'GreaterThanOrEqualToThreshold' , 2 , 60 , 300),
(28 , 'Memory Utilization' , 2 , 6 , 'memory' , 75 , 'Percent' , 'GreaterThanOrEqualToThreshold' , 2 , 60 , 300),
(32 , 'CPU Utilization'    , 1 , 4 , 'cpu'    , 80 , 'Percent' , 'GreaterThanOrEqualToThreshold' , 2 , 60 , 300),
(40 , 'Memory Utilization' , 1 , 5 , 'memory' , 75 , 'Percent' , 'GreaterThanOrEqualToThreshold' , 2 , 60 , 300),
(41 , 'CPU Utilization'    , 1 , 4 , 'cpu'    , 70 , 'Percent' , 'GreaterThanOrEqualToThreshold' , 2 , 60 , 300);

INSERT INTO config_vars (app_id, `key`, value, is_secret)
VALUES
(1 , 'NODE_ENV'     , 'production'                                   , 0),
(1 , 'LOG_LEVEL'    , 'info'                                         , 0),
(1 , 'APP_PORT'     , '3000'                                         , 0),
(1 , 'API_KEY'      , 'key-a1b2c3d4e5f6g7h8i9j0'                     , 1),
(1 , 'DATABASE_URL' , 'postgresql://user:password@db-host:5432/db-1' , 1),
(2 , 'NODE_ENV'     , 'production'                                   , 0),
(2 , 'LOG_LEVEL'    , 'info'                                         , 0),
(2 , 'APP_PORT'     , '3000'                                         , 0),
(2 , 'API_URL'      , 'https://api.example.com'                      , 0),
(2 , 'API_KEY'      , 'key-k1l2m3n4o5p6q7r8s9t0'                     , 1),
(3 , 'NODE_ENV'     , 'production'                                   , 0),
(3 , 'LOG_LEVEL'    , 'info'                                         , 0),
(3 , 'APP_PORT'     , '3000'                                         , 0),
(3 , 'JWT_SECRET'   , 'secret-a1b2c3d4e5f6g7h8i9j0'                  , 1),
(3 , 'DATABASE_URL' , 'postgresql://user:password@db-host:5432/db-3' , 1),
(4 , 'NODE_ENV'     , 'production'                                   , 0),
(4 , 'LOG_LEVEL'    , 'info'                                         , 0),
(4 , 'QUEUE_URL'    , 'amqp://user:password@mq-host:5672/vhost'      , 1),
(5 , 'NODE_ENV'     , 'staging'                                      , 0),
(5 , 'LOG_LEVEL'    , 'debug'                                        , 0),
(5 , 'APP_PORT'     , '3000'                                         , 0),
(5 , 'DATABASE_URL' , 'postgresql://user:password@db-host:5432/db-5' , 1);

INSERT INTO instances (app_id, instance_type, guid, status, container_id, container_ip, allocation_id, node_id, instance_index, last_health_check, health_status, cpu_usage, memory_usage, disk_usage, uptime, restart_count, start_time, created_at)
VALUES
(1 , 'nodejs' , UUID() , 'running' , 'container-a1b2c3d4e5f6' , '10.0.0.1'  , 5 , 1  , 0 , NOW() - INTERVAL 5 MINUTE , 'healthy' , 35.2 , 42.8 , 15.3 , 259200 , 0 , '2024-01-15 10:30:00' , '2024-01-15 10:30:00'),
(1 , 'nodejs' , UUID() , 'running' , 'container-f6e5d4c3b2a1' , '10.0.0.2'  , 5 , 2  , 1 , NOW() - INTERVAL 6 MINUTE , 'healthy' , 42.1 , 51.6 , 17.5 , 259200 , 0 , '2024-01-15 10:31:00' , '2024-01-15 10:31:00'),
(1 , 'nodejs' , UUID() , 'running' , 'container-1a2b3c4d5e6f' , '10.0.0.3'  , 5 , 3  , 2 , NOW() - INTERVAL 3 MINUTE , 'healthy' , 38.7 , 47.2 , 16.1 , 259200 , 1 , '2024-01-15 11:15:00' , '2024-01-15 10:32:00'),
(2 , 'nodejs' , UUID() , 'running' , 'container-6f5e4d3c2b1a' , '10.0.0.4'  , 4 , 4  , 0 , NOW() - INTERVAL 4 MINUTE , 'healthy' , 29.8 , 38.3 , 12.5 , 259100 , 0 , '2024-01-15 11:30:00' , '2024-01-15 11:30:00'),
(2 , 'nodejs' , UUID() , 'running' , 'container-b2a1f6e5d4c3' , '10.0.0.5'  , 4 , 5  , 1 , NOW() - INTERVAL 7 MINUTE , 'healthy' , 31.4 , 40.2 , 13.8 , 259100 , 0 , '2024-01-15 11:31:00' , '2024-01-15 11:31:00'),
(2 , 'nodejs' , UUID() , 'running' , 'container-5e6f1a2b3c4d' , '10.0.0.6'  , 4 , 6  , 2 , NOW() - INTERVAL 2 MINUTE , 'healthy' , 33.2 , 42.6 , 14.2 , 259000 , 0 , '2024-01-15 11:32:00' , '2024-01-15 11:32:00'),
(3 , 'nodejs' , UUID() , 'running' , 'container-3c2b1a6f5e4d' , '10.0.0.7'  , 5 , 7  , 0 , NOW() - INTERVAL 8 MINUTE , 'healthy' , 45.3 , 52.7 , 18.4 , 258900 , 0 , '2024-01-15 14:30:00' , '2024-01-15 14:30:00'),
(3 , 'nodejs' , UUID() , 'running' , 'container-d4c3b2a1f6e5' , '10.0.0.8'  , 5 , 8  , 1 , NOW() - INTERVAL 6 MINUTE , 'healthy' , 43.8 , 50.2 , 17.9 , 258900 , 0 , '2024-01-15 14:31:00' , '2024-01-15 14:31:00'),
(4 , 'nodejs' , UUID() , 'running' , 'container-4d3c2b1a6f5e' , '10.0.0.9'  , 6 , 9  , 0 , NOW() - INTERVAL 5 MINUTE , 'healthy' , 52.6 , 45.3 , 15.7 , 258800 , 1 , '2024-01-15 14:45:00' , '2024-01-15 10:45:00'),
(4 , 'nodejs' , UUID() , 'running' , 'container-e5d4c3b2a1f6' , '10.0.0.10' , 6 , 10 , 1 , NOW() - INTERVAL 3 MINUTE , 'healthy' , 48.9 , 42.1 , 14.8 , 258800 , 0 , '2024-01-15 10:46:00' , '2024-01-15 10:46:00');

INSERT INTO instances (app_id, instance_type, guid, status, container_id, container_ip, allocation_id, node_id, instance_index, last_health_check, health_status, cpu_usage, memory_usage, disk_usage, uptime, restart_count, start_time, created_at)
VALUES
(5  , 'nodejs' , UUID() , 'running' , 'container-5e4d3c2b1a6f' , '10.0.1.1'  , 3 , 11 , 0 , NOW() - INTERVAL 7 MINUTE , 'healthy' , 22.5 , 31.8 , 10.2 , 172800 , 0 , '2024-01-20 13:30:00' , '2024-01-20 13:30:00'),
(6  , 'nodejs' , UUID() , 'running' , 'container-f5e4d3c2b1a6' , '10.0.1.2'  , 3 , 12 , 0 , NOW() - INTERVAL 9 MINUTE , 'healthy' , 24.7 , 33.5 , 11.4 , 172700 , 0 , '2024-01-20 15:45:00' , '2024-01-20 15:45:00'),
(7  , 'nodejs' , UUID() , 'running' , 'container-6f5e4d3c2b1a' , '10.0.1.3'  , 2 , 13 , 0 , NOW() - INTERVAL 4 MINUTE , 'healthy' , 18.2 , 25.3 , 8.7  , 172600 , 0 , '2024-01-21 09:15:00' , '2024-01-21 09:15:00'),
(8  , 'nodejs' , UUID() , 'running' , 'container-a6f5e4d3c2b1' , '10.0.1.4'  , 2 , 14 , 0 , NOW() - INTERVAL 6 MINUTE , 'healthy' , 17.5 , 26.1 , 9.3  , 172500 , 0 , '2024-01-21 11:30:00' , '2024-01-21 11:30:00'),
(9  , 'python' , UUID() , 'running' , 'container-1a6f5e4d3c2b' , '10.0.1.5'  , 7 , 15 , 0 , NOW() - INTERVAL 5 MINUTE , 'healthy' , 65.3 , 72.8 , 24.6 , 172400 , 0 , '2024-01-25 10:45:00' , '2024-01-25 10:45:00'),
(9  , 'python' , UUID() , 'running' , 'container-b1a6f5e4d3c2' , '10.0.1.6'  , 7 , 16 , 1 , NOW() - INTERVAL 7 MINUTE , 'healthy' , 68.2 , 75.4 , 26.1 , 172400 , 0 , '2024-01-25 10:46:00' , '2024-01-25 10:46:00'),
(10 , 'python' , UUID() , 'running' , 'container-2b1a6f5e4d3c' , '10.0.1.7'  , 8 , 17 , 0 , NOW() - INTERVAL 3 MINUTE , 'healthy' , 71.5 , 78.2 , 27.3 , 172300 , 0 , '2024-01-25 14:30:00' , '2024-01-25 14:30:00'),
(10 , 'python' , UUID() , 'running' , 'container-c2b1a6f5e4d3' , '10.0.1.8'  , 8 , 18 , 1 , NOW() - INTERVAL 8 MINUTE , 'healthy' , 69.7 , 76.5 , 26.8 , 172300 , 1 , '2024-01-25 15:15:00' , '2024-01-25 14:31:00'),
(11 , 'go'     , UUID() , 'running' , 'container-3c2b1a6f5e4d' , '10.0.1.9'  , 6 , 19 , 0 , NOW() - INTERVAL 4 MINUTE , 'healthy' , 28.6 , 22.4 , 9.7  , 172200 , 0 , '2024-01-26 09:15:00' , '2024-01-26 09:15:00'),
(11 , 'go'     , UUID() , 'running' , 'container-d3c2b1a6f5e4' , '10.0.1.10' , 6 , 20 , 1 , NOW() - INTERVAL 6 MINUTE , 'healthy' , 30.1 , 24.8 , 10.3 , 172200 , 0 , '2024-01-26 09:16:00' , '2024-01-26 09:16:00');

INSERT INTO instances (app_id, instance_type, guid, status, container_id, container_ip, allocation_id, node_id, instance_index, last_health_check, health_status, cpu_usage, memory_usage, disk_usage, uptime, restart_count, start_time, created_at)
VALUES
-- Additional instances for app_id 1
(1, 'nodejs', UUID(), 'running', 'container-g7h8i9j0k1l2', '10.0.0.11', 5, 11, 3, NOW() - INTERVAL 4 MINUTE, 'healthy', 36.5, 44.2, 15.8, 259150, 0, '2024-01-15 10:33:00', '2024-01-15 10:33:00'),
(1, 'nodejs', UUID(), 'running', 'container-m3n4o5p6q7r8', '10.0.0.12', 5, 12, 4, NOW() - INTERVAL 5 MINUTE, 'healthy', 39.3, 46.8, 16.5, 259120, 0, '2024-01-15 10:34:00', '2024-01-15 10:34:00'),
(1, 'nodejs', UUID(), 'running', 'container-s9t0u1v2w3x4', '10.0.0.13', 5, 13, 5, NOW() - INTERVAL 6 MINUTE, 'healthy', 37.8, 45.5, 16.2, 259100, 0, '2024-01-15 10:35:00', '2024-01-15 10:35:00'),
(1, 'nodejs', UUID(), 'running', 'container-y5z6a7b8c9d0', '10.0.0.14', 5, 14, 6, NOW() - INTERVAL 4 MINUTE, 'healthy', 38.1, 46.2, 16.3, 259080, 1, '2024-01-15 11:20:00', '2024-01-15 10:36:00'),
(1, 'nodejs', UUID(), 'running', 'container-e1f2g3h4i5j6', '10.0.0.15', 5, 15, 7, NOW() - INTERVAL 3 MINUTE, 'healthy', 40.2, 48.4, 16.8, 259060, 0, '2024-01-15 10:37:00', '2024-01-15 10:37:00'),
(1, 'nodejs', UUID(), 'running', 'container-k7l8m9n0o1p2', '10.0.0.16', 5, 16, 8, NOW() - INTERVAL 7 MINUTE, 'healthy', 35.9, 43.7, 15.5, 259040, 0, '2024-01-15 10:38:00', '2024-01-15 10:38:00'),
(1, 'nodejs', UUID(), 'running', 'container-q3r4s5t6u7v8', '10.0.0.17', 5, 17, 9, NOW() - INTERVAL 5 MINUTE, 'healthy', 37.4, 45.1, 15.9, 259020, 0, '2024-01-15 10:39:00', '2024-01-15 10:39:00'),
(1, 'nodejs', UUID(), 'running', 'container-w9x0y1z2a3b4', '10.0.0.18', 5, 18, 10, NOW() - INTERVAL 8 MINUTE, 'healthy', 39.8, 47.5, 16.7, 259000, 0, '2024-01-15 10:40:00', '2024-01-15 10:40:00'),
(1, 'nodejs', UUID(), 'running', 'container-c5d6e7f8g9h0', '10.0.0.19', 5, 19, 11, NOW() - INTERVAL 6 MINUTE, 'degraded', 72.3, 86.9, 32.4, 258980, 2, '2024-01-15 12:25:00', '2024-01-15 10:41:00'),
(1, 'nodejs', UUID(), 'running', 'container-i1j2k3l4m5n6', '10.0.0.20', 5, 20, 12, NOW() - INTERVAL 9 MINUTE, 'healthy', 38.5, 46.3, 16.4, 258960, 0, '2024-01-15 10:42:00', '2024-01-15 10:42:00'),
(1, 'nodejs', UUID(), 'paused', 'container-o7p8q9r0s1t2', '10.0.0.21', 5, 21, 13, NOW() - INTERVAL 35 MINUTE, 'unknown', 0.0, 25.1, 15.2, 258940, 0, '2024-01-15 10:43:00', '2024-01-15 10:43:00'),
(1, 'nodejs', UUID(), 'running', 'container-u3v4w5x6y7z8', '10.0.0.22', 5, 22, 14, NOW() - INTERVAL 4 MINUTE, 'healthy', 37.2, 44.9, 16.0, 258920, 0, '2024-01-15 10:44:00', '2024-01-15 10:44:00'),
(1, 'nodejs', UUID(), 'running', 'container-a9b0c1d2e3f4', '10.0.0.23', 5, 23, 15, NOW() - INTERVAL 5 MINUTE, 'healthy', 38.9, 46.7, 16.6, 258900, 0, '2024-01-15 10:45:00', '2024-01-15 10:45:00'),
(1, 'nodejs', UUID(), 'error', 'container-g5h6i7j8k9l0', '10.0.0.24', 5, 24, 16, NOW() - INTERVAL 15 MINUTE, 'critical', 95.7, 94.3, 45.8, 258880, 3, '2024-01-15 13:10:00', '2024-01-15 10:46:00'),
(1, 'nodejs', UUID(), 'running', 'container-m1n2o3p4q5r6', '10.0.0.25', 5, 25, 17, NOW() - INTERVAL 3 MINUTE, 'healthy', 36.8, 44.5, 15.7, 258860, 0, '2024-01-15 10:47:00', '2024-01-15 10:47:00'),
(1, 'nodejs', UUID(), 'running', 'container-s7t8u9v0w1x2', '10.0.0.26', 5, 26, 18, NOW() - INTERVAL 4 MINUTE, 'healthy', 38.3, 46.1, 16.2, 258840, 0, '2024-01-15 10:48:00', '2024-01-15 10:48:00'),
(1, 'nodejs', UUID(), 'running', 'container-y3z4a5b6c7d8', '10.0.0.27', 5, 27, 19, NOW() - INTERVAL 5 MINUTE, 'healthy', 37.6, 45.3, 16.0, 258820, 0, '2024-01-15 10:49:00', '2024-01-15 10:49:00'),
(1, 'nodejs', UUID(), 'running', 'container-e9f0g1h2i3j4', '10.0.0.28', 5, 28, 20, NOW() - INTERVAL 7 MINUTE, 'healthy', 39.1, 47.0, 16.5, 258800, 0, '2024-01-15 10:50:00', '2024-01-15 10:50:00'),
(1, 'nodejs', UUID(), 'running', 'container-k5l6m7n8o9p0', '10.0.0.29', 5, 29, 21, NOW() - INTERVAL 6 MINUTE, 'healthy', 38.0, 45.8, 16.1, 258780, 0, '2024-01-15 10:51:00', '2024-01-15 10:51:00'),
(1, 'nodejs', UUID(), 'running', 'container-q1r2s3t4u5v6', '10.0.0.30', 5, 30, 22, NOW() - INTERVAL 8 MINUTE, 'healthy', 37.4, 45.1, 15.9, 258760, 1, '2024-01-15 11:52:00', '2024-01-15 10:52:00'),
(1, 'nodejs', UUID(), 'running', 'container-w7x8y9z0a1b2', '10.0.0.31', 5, 31, 23, NOW() - INTERVAL 5 MINUTE, 'healthy', 38.7, 46.6, 16.4, 258740, 0, '2024-01-15 10:53:00', '2024-01-15 10:53:00'),
(1, 'nodejs', UUID(), 'running', 'container-c3d4e5f6g7h8', '10.0.0.32', 5, 32, 24, NOW() - INTERVAL 4 MINUTE, 'healthy', 39.4, 47.4, 16.7, 258720, 0, '2024-01-15 10:54:00', '2024-01-15 10:54:00'),
(1, 'nodejs', UUID(), 'running', 'container-i9j0k1l2m3n4', '10.0.0.33', 5, 33, 25, NOW() - INTERVAL 3 MINUTE, 'healthy', 36.3, 43.9, 15.4, 258700, 0, '2024-01-15 10:55:00', '2024-01-15 10:55:00'),
(1, 'nodejs', UUID(), 'degraded', 'container-o5p6q7r8s9t0', '10.0.0.34', 5, 34, 26, NOW() - INTERVAL 12 MINUTE, 'warning', 65.8, 78.3, 25.7, 258680, 1, '2024-01-15 11:40:00', '2024-01-15 10:56:00'),
(1, 'nodejs', UUID(), 'running', 'container-u1v2w3x4y5z6', '10.0.0.35', 5, 35, 27, NOW() - INTERVAL 5 MINUTE, 'healthy', 37.9, 45.7, 16.1, 258660, 0, '2024-01-15 10:57:00', '2024-01-15 10:57:00'),
(1, 'nodejs', UUID(), 'running', 'container-a7b8c9d0e1f2', '10.0.0.36', 5, 36, 28, NOW() - INTERVAL 6 MINUTE, 'healthy', 38.5, 46.4, 16.3, 258640, 0, '2024-01-15 10:58:00', '2024-01-15 10:58:00'),
(1, 'nodejs', UUID(), 'running', 'container-g3h4i5j6k7l8', '10.0.0.37', 5, 37, 29, NOW() - INTERVAL 7 MINUTE, 'healthy', 37.1, 44.8, 15.8, 258620, 0, '2024-01-15 10:59:00', '2024-01-15 10:59:00'),
(1, 'nodejs', UUID(), 'running', 'container-m9n0o1p2q3r4', '10.0.0.38', 5, 38, 30, NOW() - INTERVAL 4 MINUTE, 'healthy', 39.0, 46.9, 16.5, 258600, 0, '2024-01-15 11:00:00', '2024-01-15 11:00:00'),
(1, 'nodejs', UUID(), 'running', 'container-s5t6u7v8w9x0', '10.0.0.39', 5, 39, 31, NOW() - INTERVAL 5 MINUTE, 'healthy', 38.2, 46.0, 16.2, 258580, 0, '2024-01-15 11:01:00', '2024-01-15 11:01:00'),
(1, 'nodejs', UUID(), 'running', 'container-y1z2a3b4c5d6', '10.0.0.40', 5, 40, 32, NOW() - INTERVAL 3 MINUTE, 'healthy', 37.5, 45.2, 15.9, 258560, 0, '2024-01-15 11:02:00', '2024-01-15 11:02:00'),
(1, 'nodejs', UUID(), 'running', 'container-e7f8g9h0i1j2', '10.0.0.41', 5, 41, 33, NOW() - INTERVAL 6 MINUTE, 'healthy', 38.8, 46.7, 16.4, 258540, 0, '2024-01-15 11:03:00', '2024-01-15 11:03:00'),
(1, 'nodejs', UUID(), 'running', 'container-k3l4m5n6o7p8', '10.0.0.42', 5, 42, 34, NOW() - INTERVAL 8 MINUTE, 'healthy', 39.5, 47.5, 16.8, 258520, 0, '2024-01-15 11:04:00', '2024-01-15 11:04:00'),
(1, 'nodejs', UUID(), 'running', 'container-q9r0s1t2u3v4', '10.0.0.43', 5, 43, 35, NOW() - INTERVAL 5 MINUTE, 'healthy', 36.7, 44.3, 15.6, 258500, 0, '2024-01-15 11:05:00', '2024-01-15 11:05:00'),
(1, 'nodejs', UUID(), 'error', 'container-w5x6y7z8a9b0', '10.0.0.44', 5, 44, 36, NOW() - INTERVAL 18 MINUTE, 'critical', 98.2, 96.1, 48.3, 258480, 2, '2024-01-15 14:15:00', '2024-01-15 11:06:00'),
(1, 'nodejs', UUID(), 'running', 'container-c1d2e3f4g5h6', '10.0.0.45', 5, 45, 37, NOW() - INTERVAL 4 MINUTE, 'healthy', 38.1, 45.9, 16.2, 258460, 0, '2024-01-15 11:07:00', '2024-01-15 11:07:00'),
(1, 'nodejs', UUID(), 'running', 'container-i7j8k9l0m1n2', '10.0.0.46', 5, 46, 38, NOW() - INTERVAL 7 MINUTE, 'healthy', 37.3, 45.0, 15.9, 258440, 0, '2024-01-15 11:08:00', '2024-01-15 11:08:00'),
(1, 'nodejs', UUID(), 'running', 'container-o3p4q5r6s7t8', '10.0.0.47', 5, 47, 39, NOW() - INTERVAL 5 MINUTE, 'healthy', 39.2, 47.1, 16.6, 258420, 0, '2024-01-15 11:09:00', '2024-01-15 11:09:00'),
(1, 'nodejs', UUID(), 'running', 'container-u9v0w1x2y3z4', '10.0.0.48', 5, 48, 40, NOW() - INTERVAL 6 MINUTE, 'healthy', 38.4, 46.2, 16.3, 258400, 0, '2024-01-15 11:10:00', '2024-01-15 11:10:00'),
(1, 'nodejs', UUID(), 'running', 'container-a5b6c7d8e9f0', '10.0.0.49', 5, 49, 41, NOW() - INTERVAL 4 MINUTE, 'healthy', 37.7, 45.4, 16.0, 258380, 1, '2024-01-15 12:05:00', '2024-01-15 11:11:00'),
(1, 'nodejs', UUID(), 'running', 'container-g1h2i3j4k5l6', '10.0.0.50', 5, 50, 42, NOW() - INTERVAL 5 MINUTE, 'healthy', 38.9, 46.8, 16.5, 258360, 0, '2024-01-15 11:12:00', '2024-01-15 11:12:00'),
(1, 'nodejs', UUID(), 'running', 'container-m7n8o9p0q1r2', '10.0.0.51', 5, 51, 43, NOW() - INTERVAL 3 MINUTE, 'healthy', 39.6, 47.6, 16.9, 258340, 0, '2024-01-15 11:13:00', '2024-01-15 11:13:00'),
(1, 'nodejs', UUID(), 'paused', 'container-s3t4u5v6w7x8', '10.0.0.52', 5, 52, 44, NOW() - INTERVAL 45 MINUTE, 'unknown', 0.0, 26.2, 15.5, 258320, 0, '2024-01-15 11:14:00', '2024-01-15 11:14:00'),
(1, 'nodejs', UUID(), 'running', 'container-y9z0a1b2c3d4', '10.0.0.53', 5, 53, 45, NOW() - INTERVAL 6 MINUTE, 'healthy', 37.0, 44.7, 15.7, 258300, 0, '2024-01-15 11:15:00', '2024-01-15 11:15:00'),
(1, 'nodejs', UUID(), 'running', 'container-e5f6g7h8i9j0', '10.0.0.54', 5, 54, 46, NOW() - INTERVAL 7 MINUTE, 'healthy', 38.3, 46.1, 16.2, 258280, 0, '2024-01-15 11:16:00', '2024-01-15 11:16:00'),
(1, 'nodejs', UUID(), 'running', 'container-k1l2m3n4o5p6', '10.0.0.55', 5, 55, 47, NOW() - INTERVAL 4 MINUTE, 'healthy', 37.6, 45.3, 16.0, 258260, 0, '2024-01-15 11:17:00', '2024-01-15 11:17:00'),
(1, 'nodejs', UUID(), 'running', 'container-q7r8s9t0u1v2', '10.0.0.56', 5, 56, 48, NOW() - INTERVAL 5 MINUTE, 'healthy', 39.4, 47.3, 16.7, 258240, 0, '2024-01-15 11:18:00', '2024-01-15 11:18:00'),
(1, 'nodejs', UUID(), 'running', 'container-w3x4y5z6a7b8', '10.0.0.57', 5, 57, 49, NOW() - INTERVAL 3 MINUTE, 'healthy', 38.6, 46.5, 16.4, 258220, 0, '2024-01-15 11:19:00', '2024-01-15 11:19:00'),
(1, 'nodejs', UUID(), 'running', 'container-c9d0e1f2g3h4', '10.0.0.58', 5, 58, 50, NOW() - INTERVAL 9 MINUTE, 'healthy', 37.8, 45.6, 16.1, 258200, 0, '2024-01-15 11:20:00', '2024-01-15 11:20:00'),
(1, 'nodejs', UUID(), 'degraded', 'container-i5j6k7l8m9n0', '10.0.0.59', 5, 59, 51, NOW() - INTERVAL 10 MINUTE, 'warning', 68.3, 79.7, 27.3, 258180, 1, '2024-01-15 11:50:00', '2024-01-15 11:21:00'),
(1, 'nodejs', UUID(), 'running', 'container-o1p2q3r4s5t6', '10.0.0.60', 5, 60, 52, NOW() - INTERVAL 5 MINUTE, 'healthy', 38.0, 45.8, 16.1, 258160, 0, '2024-01-15 11:22:00', '2024-01-15 11:22:00'),
(1, 'nodejs', UUID(), 'running', 'container-u7v8w9x0y1z2', '10.0.0.61', 5, 61, 53, NOW() - INTERVAL 4 MINUTE, 'healthy', 39.7, 47.7, 16.9, 258140, 0, '2024-01-15 11:23:00', '2024-01-15 11:23:00'),
(1, 'nodejs', UUID(), 'running', 'container-a3b4c5d6e7f8', '10.0.0.62', 5, 62, 54, NOW() - INTERVAL 6 MINUTE, 'healthy', 37.2, 44.9, 15.8, 258120, 0, '2024-01-15 11:24:00', '2024-01-15 11:24:00'),
(1, 'nodejs', UUID(), 'running', 'container-g9h0i1j2k3l4', '10.0.0.63', 5, 63, 55, NOW() - INTERVAL 7 MINUTE, 'healthy', 38.5, 46.3, 16.3, 258100, 0, '2024-01-15 11:25:00', '2024-01-15 11:25:00'),
(1, 'nodejs', UUID(), 'running', 'container-m5n6o7p8q9r0', '10.0.0.64', 5, 64, 56, NOW() - INTERVAL 5 MINUTE, 'healthy', 37.9, 45.7, 16.1, 258080, 0, '2024-01-15 11:26:00', '2024-01-15 11:26:00'),
(1, 'nodejs', UUID(), 'running', 'container-s1t2u3v4w5x6', '10.0.0.65', 5, 65, 57, NOW() - INTERVAL 4 MINUTE, 'healthy', 39.5, 47.5, 16.8, 258060, 0, '2024-01-15 11:27:00', '2024-01-15 11:27:00'),
(1, 'nodejs', UUID(), 'running', 'container-y7z8a9b0c1d2', '10.0.0.66', 5, 66, 58, NOW() - INTERVAL 5 MINUTE, 'healthy', 38.8, 46.7, 16.4, 258040, 0, '2024-01-15 11:28:00', '2024-01-15 11:28:00'),
(1, 'nodejs', UUID(), 'running', 'container-e3f4g5h6i7j8', '10.0.0.67', 5, 67, 59, NOW() - INTERVAL 8 MINUTE, 'healthy', 37.4, 45.1, 15.9, 258020, 0, '2024-01-15 11:29:00', '2024-01-15 11:29:00'),
(1, 'nodejs', UUID(), 'error', 'container-k9l0m1n2o3p4', '10.0.0.68', 5, 68, 60, NOW() - INTERVAL 20 MINUTE, 'critical', 97.1, 95.3, 47.6, 258000, 3, '2024-01-15 15:05:00', '2024-01-15 11:30:00'),
(1, 'nodejs', UUID(), 'running', 'container-q5r6s7t8u9v0', '10.0.0.69', 5, 69, 61, NOW() - INTERVAL 5 MINUTE, 'healthy', 38.2, 46.0, 16.2, 257980, 0, '2024-01-15 11:31:00', '2024-01-15 11:31:00'),
(1, 'nodejs', UUID(), 'running', 'container-w1x2y3z4a5b6', '10.0.0.70', 5, 70, 62, NOW() - INTERVAL 4 MINUTE, 'healthy', 39.8, 47.8, 17.0, 257960, 0, '2024-01-15 11:32:00', '2024-01-15 11:32:00'),
(1, 'nodejs', UUID(), 'running', 'container-c7d8e9f0g1h2', '10.0.0.71', 5, 71, 63, NOW() - INTERVAL 7 MINUTE, 'healthy', 37.5, 45.2, 15.9, 257940, 0, '2024-01-15 11:33:00', '2024-01-15 11:33:00'),
(1, 'nodejs', UUID(), 'running', 'container-i3j4k5l6m7n8', '10.0.0.72', 5, 72, 64, NOW() - INTERVAL 6 MINUTE, 'healthy', 38.7, 46.6, 16.4, 257920, 0, '2024-01-15 11:34:00', '2024-01-15 11:34:00'),
(1, 'nodejs', UUID(), 'running', 'container-o9p0q1r2s3t4', '10.0.0.73', 5, 73, 65, NOW() - INTERVAL 5 MINUTE, 'healthy', 37.1, 44.8, 15.8, 257900, 1, '2024-01-15 12:35:00', '2024-01-15 11:35:00'),
(1, 'nodejs', UUID(), 'running', 'container-u5v6w7x8y9z0', '10.0.0.74', 5, 74, 66, NOW() - INTERVAL 3 MINUTE, 'healthy', 39.6, 47.6, 16.9, 257880, 0, '2024-01-15 11:36:00', '2024-01-15 11:36:00'),
(1, 'nodejs', UUID(), 'running', 'container-a1b2c3d4e5f6', '10.0.0.75', 5, 75, 67, NOW() - INTERVAL 4 MINUTE, 'healthy', 38.9, 46.8, 16.5, 257860, 0, '2024-01-15 11:37:00', '2024-01-15 11:37:00'),
(1, 'nodejs', UUID(), 'running', 'container-g7h8i9j0k1l2', '10.0.0.76', 5, 76, 68, NOW() - INTERVAL 9 MINUTE, 'healthy', 37.6, 45.4, 16.0, 257840, 0, '2024-01-15 11:38:00', '2024-01-15 11:38:00'),
(1, 'nodejs', UUID(), 'running', 'container-m3n4o5p6q7r8', '10.0.0.77', 5, 77, 69, NOW() - INTERVAL 5 MINUTE, 'healthy', 38.4, 46.2, 16.3, 257820, 0, '2024-01-15 11:39:00', '2024-01-15 11:39:00'),
(1, 'nodejs', UUID(), 'paused', 'container-s9t0u1v2w3x4', '10.0.0.78', 5, 78, 70, NOW() - INTERVAL 38 MINUTE, 'unknown', 0.0, 24.8, 15.0, 257800, 0, '2024-01-15 11:40:00', '2024-01-15 11:40:00'),
(1, 'nodejs', UUID(), 'running', 'container-y5z6a7b8c9d0', '10.0.0.79', 5, 79, 71, NOW() - INTERVAL 4 MINUTE, 'healthy', 39.9, 47.9, 17.0, 257780, 0, '2024-01-15 11:41:00', '2024-01-15 11:41:00'),
(1, 'nodejs', UUID(), 'running', 'container-e1f2g3h4i5j6', '10.0.0.80', 5, 80, 72, NOW() - INTERVAL 6 MINUTE, 'healthy', 37.7, 45.5, 16.0, 257760, 0, '2024-01-15 11:42:00', '2024-01-15 11:42:00'),
(1, 'nodejs', UUID(), 'running', 'container-o1p2q3r4s5t6', '10.0.0.60', 5, 60, 73, NOW() - INTERVAL 5 MINUTE, 'healthy', 38.0, 45.8, 16.1, 258160, 0, '2024-01-15 11:22:00', '2024-01-15 11:22:00'),
(1, 'nodejs', UUID(), 'running', 'container-u7v8w9x0y1z2', '10.0.0.61', 5, 61, 74, NOW() - INTERVAL 4 MINUTE, 'healthy', 39.7, 47.7, 16.9, 258140, 0, '2024-01-15 11:23:00', '2024-01-15 11:23:00'),
(1, 'nodejs', UUID(), 'running', 'container-a3b4c5d6e7f8', '10.0.0.62', 5, 62, 75, NOW() - INTERVAL 6 MINUTE, 'healthy', 37.2, 44.9, 15.8, 258120, 0, '2024-01-15 11:24:00', '2024-01-15 11:24:00'),
(1, 'nodejs', UUID(), 'running', 'container-g9h0i1j2k3l4', '10.0.0.63', 5, 63, 76, NOW() - INTERVAL 7 MINUTE, 'healthy', 38.5, 46.3, 16.3, 258100, 0, '2024-01-15 11:25:00', '2024-01-15 11:25:00'),
(1, 'nodejs', UUID(), 'running', 'container-m5n6o7p8q9r0', '10.0.0.64', 5, 64, 77, NOW() - INTERVAL 5 MINUTE, 'healthy', 37.9, 45.7, 16.1, 258080, 0, '2024-01-15 11:26:00', '2024-01-15 11:26:00'),
(1, 'nodejs', UUID(), 'running', 'container-s1t2u3v4w5x6', '10.0.0.65', 5, 65, 78, NOW() - INTERVAL 4 MINUTE, 'healthy', 39.5, 47.5, 16.8, 258060, 0, '2024-01-15 11:27:00', '2024-01-15 11:27:00'),
(1, 'nodejs', UUID(), 'running', 'container-y7z8a9b0c1d2', '10.0.0.66', 5, 66, 79, NOW() - INTERVAL 5 MINUTE, 'healthy', 38.8, 46.7, 16.4, 258040, 0, '2024-01-15 11:28:00', '2024-01-15 11:28:00'),
(1, 'nodejs', UUID(), 'running', 'container-e3f4g5h6i7j8', '10.0.0.67', 5, 67, 80, NOW() - INTERVAL 8 MINUTE, 'healthy', 37.4, 45.1, 15.9, 258020, 0, '2024-01-15 11:29:00', '2024-01-15 11:29:00'),
(1, 'nodejs', UUID(), 'error', 'container-k9l0m1n2o3p4', '10.0.0.68', 5, 68, 81, NOW() - INTERVAL 20 MINUTE, 'critical', 97.1, 95.3, 47.6, 258000, 3, '2024-01-15 15:05:00', '2024-01-15 11:30:00'),
(1, 'nodejs', UUID(), 'running', 'container-q5r6s7t8u9v0', '10.0.0.69', 5, 69, 82, NOW() - INTERVAL 5 MINUTE, 'healthy', 38.2, 46.0, 16.2, 257980, 0, '2024-01-15 11:31:00', '2024-01-15 11:31:00'),
(1, 'nodejs', UUID(), 'running', 'container-w1x2y3z4a5b6', '10.0.0.70', 5, 70, 83, NOW() - INTERVAL 4 MINUTE, 'healthy', 39.8, 47.8, 17.0, 257960, 0, '2024-01-15 11:32:00', '2024-01-15 11:32:00'),
(1, 'nodejs', UUID(), 'running', 'container-c7d8e9f0g1h2', '10.0.0.71', 5, 71, 84, NOW() - INTERVAL 7 MINUTE, 'healthy', 37.5, 45.2, 15.9, 257940, 0, '2024-01-15 11:33:00', '2024-01-15 11:33:00'),
(1, 'nodejs', UUID(), 'running', 'container-i3j4k5l6m7n8', '10.0.0.72', 5, 72, 85, NOW() - INTERVAL 6 MINUTE, 'healthy', 38.7, 46.6, 16.4, 257920, 0, '2024-01-15 11:34:00', '2024-01-15 11:34:00'),
(1, 'nodejs', UUID(), 'running', 'container-o9p0q1r2s3t4', '10.0.0.73', 5, 73, 86, NOW() - INTERVAL 5 MINUTE, 'healthy', 37.1, 44.8, 15.8, 257900, 1, '2024-01-15 12:35:00', '2024-01-15 11:35:00'),
(1, 'nodejs', UUID(), 'running', 'container-u5v6w7x8y9z0', '10.0.0.74', 5, 74, 87, NOW() - INTERVAL 3 MINUTE, 'healthy', 39.6, 47.6, 16.9, 257880, 0, '2024-01-15 11:36:00', '2024-01-15 11:36:00'),
(1, 'nodejs', UUID(), 'running', 'container-a1b2c3d4e5f6', '10.0.0.75', 5, 75, 88, NOW() - INTERVAL 4 MINUTE, 'healthy', 38.9, 46.8, 16.5, 257860, 0, '2024-01-15 11:37:00', '2024-01-15 11:37:00'),
(1, 'nodejs', UUID(), 'running', 'container-g7h8i9j0k1l2', '10.0.0.76', 5, 76, 89, NOW() - INTERVAL 9 MINUTE, 'healthy', 37.6, 45.4, 16.0, 257840, 0, '2024-01-15 11:38:00', '2024-01-15 11:38:00'),
(1, 'nodejs', UUID(), 'running', 'container-m3n4o5p6q7r8', '10.0.0.77', 5, 77, 90, NOW() - INTERVAL 5 MINUTE, 'healthy', 38.4, 46.2, 16.3, 257820, 0, '2024-01-15 11:39:00', '2024-01-15 11:39:00'),
(1, 'nodejs', UUID(), 'paused', 'container-s9t0u1v2w3x4', '10.0.0.78', 5, 78, 91, NOW() - INTERVAL 38 MINUTE, 'unknown', 0.0, 24.8, 15.0, 257800, 0, '2024-01-15 11:40:00', '2024-01-15 11:40:00'),
(1, 'nodejs', UUID(), 'running', 'container-y5z6a7b8c9d0', '10.0.0.79', 5, 79, 92, NOW() - INTERVAL 4 MINUTE, 'healthy', 39.9, 47.9, 17.0, 257780, 0, '2024-01-15 11:41:00', '2024-01-15 11:41:00');

INSERT INTO routes (domain_id, host, path, app_id, weight, https_only, created_at)
VALUES
(1 , 'api'          , ''        , 1  , 100 , 1 , '2022-05-20 10:00:00'),
(1 , 'app'          , ''        , 2  , 100 , 1 , '2022-05-20 12:15:00'),
(1 , 'auth'         , ''        , 3  , 100 , 1 , '2022-05-20 14:45:00'),
(2 , NULL           , '/api/v1' , 1  , 100 , 1 , '2022-05-21 09:30:00'),
(2 , NULL           , '/auth'   , 3  , 100 , 1 , '2022-05-21 11:15:00'),
(3 , 'api-staging'  , ''        , 5  , 100 , 1 , '2022-05-22 10:00:00'),
(3 , 'app-staging'  , ''        , 6  , 100 , 1 , '2022-05-22 12:15:00'),
(4 , 'api-dev'      , ''        , 7  , 100 , 0 , '2022-05-23 09:30:00'),
(4 , 'app-dev'      , ''        , 8  , 100 , 0 , '2022-05-23 11:45:00'),
(5 , 'data'         , ''        , 9  , 100 , 1 , '2022-06-25 11:15:00'),
(5 , 'ml'           , ''        , 10 , 100 , 1 , '2022-06-25 15:00:00'),
(5 , 'api'          , ''        , 11 , 100 , 1 , '2022-06-26 09:45:00'),
(6 , 'data-staging' , ''        , 12 , 100 , 1 , '2022-06-26 12:30:00'),
(6 , 'api-staging'  , ''        , 13 , 100 , 1 , '2022-06-26 15:45:00'),
(7 , 'code'         , ''        , 16 , 100 , 1 , '2022-07-30 10:15:00');

INSERT INTO service_bindings (app_id, service_id, credentials, binding_name, status, created_at)
VALUES
(1  , 1  , JSON_OBJECT('host', 'mysql-a1b2c3d4.internal', 'port', 3306, 'database', 'db_1_1', 'username', 'user_1', 'password', 'pass_a1b2c3d4')      , 'api_backend_mysql'       , 'created' , '2022-05-20 11:00:00'),
(1  , 2  , JSON_OBJECT('host', 'redis-e5f6g7h8.internal', 'port', 6379, 'password', 'pass_e5f6g7h8')                                                  , 'api_backend_redis'       , 'created' , '2022-05-20 11:15:00'),
(2  , 2  , JSON_OBJECT('host', 'redis-e5f6g7h8.internal', 'port', 6379, 'password', 'pass_e5f6g7h8')                                                  , 'web_frontend_redis'      , 'created' , '2022-05-20 12:30:00'),
(3  , 1  , JSON_OBJECT('host', 'mysql-a1b2c3d4.internal', 'port', 3306, 'database', 'db_3_1', 'username', 'user_3', 'password', 'pass_i9j0k1l2')      , 'auth_service_mysql'      , 'created' , '2022-05-20 15:00:00'),
(4  , 5  , JSON_OBJECT('host', 'rabbitmq-q7r8s9t0.internal', 'port', 5672, 'vhost', 'vhost_4', 'username', 'user_4', 'password', 'pass_q7r8s9t0')     , 'worker_jobs_rabbitmq'    , 'created' , '2022-05-21 11:00:00'),
(9  , 3  , JSON_OBJECT('host', 'postgres-i9j0k1l2.internal', 'port', 5432, 'database', 'db_9_3', 'username', 'user_9', 'password', 'pass_m3n4o5p6')   , 'data_analytics_postgres' , 'created' , '2022-06-25 11:30:00'),
(10 , 3  , JSON_OBJECT('host', 'postgres-i9j0k1l2.internal', 'port', 5432, 'database', 'db_10_3', 'username', 'user_10', 'password', 'pass_u1v2w3x4') , 'ml_processor_postgres'   , 'created' , '2022-06-25 15:15:00'),
(11 , 2  , JSON_OBJECT('host', 'redis-e5f6g7h8.internal', 'port', 6379, 'password', 'pass_y5z6a7b8')                                                  , 'api_gateway_redis'       , 'created' , '2022-06-26 10:00:00'),
(16 , 14 , JSON_OBJECT('host', 'mysql-a3b4c5d6.internal', 'port', 3306, 'database', 'db_16_14', 'username', 'user_16', 'password', 'pass_e7f8g9h0')   , 'code_service_mysql'      , 'created' , '2022-07-30 10:45:00'),
(27 , 10 , JSON_OBJECT('host', 'redis-k7l8m9n0.internal', 'port', 6379, 'password', 'pass_i1j2k3l4')                                                  , 'ai_engine_redis'         , 'created' , '2022-10-10 10:00:00');

INSERT INTO builds (app_id, source_version, commit_sha, commit_message, author, status, build_pack_used, log_url, started_at, completed_at, build_duration, created_at)
VALUES
(1 , 'v0.1' , 'a1b2c3d4e5f6g7h8i9j0k1l2m3n4o5p6q7r8s9t0' , 'Initial backend API implementation'  , 'John Smith'       , 'succeeded' , 'nodejs-buildpack' , 'https://logs.example.com/builds/1'  , '2022-05-19 09:30:00' , '2022-05-19 09:45:00' , 900  , '2022-05-19 09:30:00'),
(1 , 'v0.2' , 'b2c3d4e5f6g7h8i9j0k1l2m3n4o5p6q7r8s9t0a1' , 'Fix authentication flow'             , 'John Smith'       , 'succeeded' , 'nodejs-buildpack' , 'https://logs.example.com/builds/2'  , '2022-05-19 14:15:00' , '2022-05-19 14:28:00' , 780  , '2022-05-19 14:15:00'),
(1 , 'v0.3' , 'c3d4e5f6g7h8i9j0k1l2m3n4o5p6q7r8s9t0a1b2' , 'Add user management endpoints'       , 'Jane Johnson'     , 'succeeded' , 'nodejs-buildpack' , 'https://logs.example.com/builds/3'  , '2022-05-20 08:45:00' , '2022-05-20 09:00:00' , 900  , '2022-05-20 08:45:00'),
(2 , 'v0.1' , 'd4e5f6g7h8i9j0k1l2m3n4o5p6q7r8s9t0a1b2c3' , 'Initial frontend implementation'     , 'Michael Williams' , 'succeeded' , 'nodejs-buildpack' , 'https://logs.example.com/builds/4'  , '2022-05-20 10:30:00' , '2022-05-20 10:48:00' , 1080 , '2022-05-20 10:30:00'),
(2 , 'v0.2' , 'e5f6g7h8i9j0k1l2m3n4o5p6q7r8s9t0a1b2c3d4' , 'Implement responsive design'         , 'Michael Williams' , 'succeeded' , 'nodejs-buildpack' , 'https://logs.example.com/builds/5'  , '2022-05-20 15:15:00' , '2022-05-20 15:32:00' , 1020 , '2022-05-20 15:15:00'),
(3 , 'v0.1' , 'f6g7h8i9j0k1l2m3n4o5p6q7r8s9t0a1b2c3d4e5' , 'Initial auth service implementation' , 'Sarah Jones'      , 'succeeded' , 'nodejs-buildpack' , 'https://logs.example.com/builds/6'  , '2022-05-20 13:00:00' , '2022-05-20 13:12:00' , 720  , '2022-05-20 13:00:00'),
(3 , 'v0.2' , 'g7h8i9j0k1l2m3n4o5p6q7r8s9t0a1b2c3d4e5f6' , 'Add JWT authentication'              , 'Sarah Jones'      , 'failed'    , 'nodejs-buildpack' , 'https://logs.example.com/builds/7'  , '2022-05-20 16:30:00' , '2022-05-20 16:35:00' , 300  , '2022-05-20 16:30:00'),
(3 , 'v0.3' , 'h8i9j0k1l2m3n4o5p6q7r8s9t0a1b2c3d4e5f6g7' , 'Fix JWT authentication'              , 'Sarah Jones'      , 'succeeded' , 'nodejs-buildpack' , 'https://logs.example.com/builds/8'  , '2022-05-20 17:15:00' , '2022-05-20 17:28:00' , 780  , '2022-05-20 17:15:00'),
(4 , 'v0.1' , 'i9j0k1l2m3n4o5p6q7r8s9t0a1b2c3d4e5f6g7h8' , 'Initial worker implementation'       , 'David Brown'      , 'succeeded' , 'nodejs-buildpack' , 'https://logs.example.com/builds/9'  , '2022-05-21 09:00:00' , '2022-05-21 09:11:00' , 660  , '2022-05-21 09:00:00'),
(5 , 'v0.1' , 'j0k1l2m3n4o5p6q7r8s9t0a1b2c3d4e5f6g7h8i9' , 'Initial staging API implementation'  , 'John Smith'       , 'succeeded' , 'nodejs-buildpack' , 'https://logs.example.com/builds/10' , '2022-05-21 12:45:00' , '2022-05-21 12:57:00' , 720  , '2022-05-21 12:45:00');

INSERT INTO deployments (app_id, build_id, version, status, deployment_strategy, staged_instances, total_instances, environment_variables, started_at, completed_at, deployment_duration, created_at, created_by)
VALUES
(1 , 3  , 'v0.3' , 'deployed' , 'rolling'  , 3 , 3 , JSON_OBJECT('NODE_ENV', 'production', 'LOG_LEVEL', 'info') , '2022-05-20 09:15:00' , '2022-05-20 09:30:00' , 900 , '2022-05-20 09:15:00' , 1),
(2 , 5  , 'v0.2' , 'deployed' , 'rolling'  , 3 , 3 , JSON_OBJECT('NODE_ENV', 'production', 'LOG_LEVEL', 'info') , '2022-05-20 15:45:00' , '2022-05-20 16:00:00' , 900 , '2022-05-20 15:45:00' , 2),
(3 , 8  , 'v0.3' , 'deployed' , 'rolling'  , 2 , 2 , JSON_OBJECT('NODE_ENV', 'production', 'LOG_LEVEL', 'info') , '2022-05-20 17:45:00' , '2022-05-20 18:00:00' , 900 , '2022-05-20 17:45:00' , 3),
(4 , 9  , 'v0.1' , 'deployed' , 'recreate' , 2 , 2 , JSON_OBJECT('NODE_ENV', 'production', 'LOG_LEVEL', 'info') , '2022-05-21 09:30:00' , '2022-05-21 09:42:00' , 720 , '2022-05-21 09:30:00' , 4),
(5 , 10 , 'v0.1' , 'deployed' , 'rolling'  , 1 , 1 , JSON_OBJECT('NODE_ENV', 'staging', 'LOG_LEVEL', 'debug')   , '2022-05-21 13:15:00' , '2022-05-21 13:25:00' , 600 , '2022-05-21 13:15:00' , 1);

INSERT INTO deployment_logs (deployment_id, log_type, log_level, message, timestamp)
VALUES
(1 , 'deployment' , 'info' , 'Starting deployment process'        , '2022-05-20 09:15:00'),
(1 , 'deployment' , 'info' , 'Preparing build artifacts'          , '2022-05-20 09:16:00'),
(1 , 'deployment' , 'info' , 'Creating container image'           , '2022-05-20 09:17:00'),
(1 , 'deployment' , 'info' , 'Pushing image to registry'          , '2022-05-20 09:20:00'),
(1 , 'deployment' , 'info' , 'Updating application configuration' , '2022-05-20 09:22:00'),
(1 , 'deployment' , 'info' , 'Starting container'                 , '2022-05-20 09:24:00'),
(1 , 'deployment' , 'info' , 'Container started successfully'     , '2022-05-20 09:25:00'),
(1 , 'deployment' , 'info' , 'Waiting for health check to pass'   , '2022-05-20 09:26:00'),
(1 , 'deployment' , 'info' , 'Health check passed'                , '2022-05-20 09:28:00'),
(1 , 'deployment' , 'info' , 'Deployment completed successfully'  , '2022-05-20 09:30:00');

INSERT INTO tasks (app_id, command, name, status, memory_in_mb, disk_in_mb, cpu, timeout_seconds, result, exit_code, node_id, started_at, completed_at, duration, created_at, created_by)
VALUES
(1  , 'node scripts/db-migrate.js'     , 'Database Migration' , 'succeeded' , 1024 , 2048  , 1.0 , 1800  , 'Task completed successfully' , 0    , 1  , '2022-05-20 08:30:00'      , '2022-05-20 08:35:00' , 300   , '2022-05-20 08:30:00'      , 1),
(2  , 'npm run build'                  , 'Build Assets'       , 'succeeded' , 1024 , 2048  , 1.0 , 1800  , 'Task completed successfully' , 0    , 2  , '2022-05-20 10:15:00'      , '2022-05-20 10:22:00' , 420   , '2022-05-20 10:15:00'      , 2),
(3  , 'node scripts/seed-data.js'      , 'Seed Data'          , 'succeeded' , 1024 , 2048  , 1.0 , 1800  , 'Task completed successfully' , 0    , 3  , '2022-05-20 13:30:00'      , '2022-05-20 13:36:00' , 360   , '2022-05-20 13:30:00'      , 3),
(4  , 'node scripts/process-queue.js'  , 'Process Queue'      , 'succeeded' , 2048 , 4096  , 1.5 , 3600  , 'Task completed successfully' , 0    , 4  , '2022-05-21 08:45:00'      , '2022-05-21 09:15:00' , 1800  , '2022-05-21 08:45:00'      , 4),
(5  , 'node scripts/cleanup.js'        , 'Cleanup Task'       , 'failed'    , 1024 , 2048  , 1.0 , 1800  , NULL                          , 1    , 5  , '2022-05-21 12:30:00'      , '2022-05-21 12:32:00' , 120   , '2022-05-21 12:30:00'      , 1),
(9  , 'python scripts/analyze-data.py' , 'Data Analysis'      , 'succeeded' , 2048 , 4096  , 1.5 , 3600  , 'Task completed successfully' , 0    , 9  , '2022-06-25 11:00:00'      , '2022-06-25 11:45:00' , 2700  , '2022-06-25 11:00:00'      , 5),
(10 , 'python scripts/train-model.py'  , 'Train Model'        , 'succeeded' , 4096 , 8192  , 2.0 , 7200  , 'Task completed successfully' , 0    , 10 , '2022-06-25 15:30:00'      , '2022-06-25 17:30:00' , 7200  , '2022-06-25 15:30:00'      , 5),
(16 , 'java -jar scripts/compile.jar'  , 'Compile Code'       , 'succeeded' , 2048 , 4096  , 1.5 , 3600  , 'Task completed successfully' , 0    , 16 , '2022-07-30 11:00:00'      , '2022-07-30 11:25:00' , 1500  , '2022-07-30 11:00:00'      , 8),
(22 , 'python scripts/process-data.py' , 'Process Data'       , 'running'   , 2048 , 4096  , 1.5 , 3600  , NULL                          , NULL , 17 , NOW() - INTERVAL 30 MINUTE , NULL                  , NULL  , NOW() - INTERVAL 30 MINUTE , 14),
(27 , 'python scripts/train-ai.py'     , 'Train AI Model'     , 'succeeded' , 8192 , 16384 , 4.0 , 14400 , 'Task completed successfully' , 0    , 18 , '2022-10-10 10:30:00'      , '2022-10-10 13:30:00' , 10800 , '2022-10-10 10:30:00'      , 14);

INSERT INTO audit_logs (user_id, org_id, app_id, action, resource_type, resource_id, before_state, after_state, details, ip_address, user_agent, request_id, status, created_at)
VALUES
(1 , 1 , 1    , 'create' , 'app'   , '1' , NULL                             , JSON_OBJECT('name', 'api-backend', 'status', 'stopped')  , JSON_OBJECT('source', 'api', 'operation_id', 'op-a1b2c3d4') , '192.168.1.1' , 'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36'          , 'req-a1b2c3d4' , 'success' , '2022-05-19 09:00:00'),
(1 , 1 , 1    , 'deploy' , 'app'   , '1' , JSON_OBJECT('status', 'stopped') , JSON_OBJECT('status', 'started')                         , JSON_OBJECT('source', 'api', 'operation_id', 'op-b2c3d4e5') , '192.168.1.1' , 'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36'          , 'req-b2c3d4e5' , 'success' , '2022-05-20 09:30:00'),
(2 , 1 , 2    , 'create' , 'app'   , '2' , NULL                             , JSON_OBJECT('name', 'web-frontend', 'status', 'stopped') , JSON_OBJECT('source', 'api', 'operation_id', 'op-c3d4e5f6') , '192.168.1.2' , 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36'                , 'req-c3d4e5f6' , 'success' , '2022-05-20 10:00:00'),
(2 , 1 , 2    , 'deploy' , 'app'   , '2' , JSON_OBJECT('status', 'stopped') , JSON_OBJECT('status', 'started')                         , JSON_OBJECT('source', 'api', 'operation_id', 'op-d4e5f6g7') , '192.168.1.2' , 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36'                , 'req-d4e5f6g7' , 'success' , '2022-05-20 16:00:00'),
(3 , 1 , 3    , 'create' , 'app'   , '3' , NULL                             , JSON_OBJECT('name', 'auth-service', 'status', 'stopped') , JSON_OBJECT('source', 'api', 'operation_id', 'op-e5f6g7h8') , '192.168.1.3' , 'Mozilla/5.0 (iPhone; CPU iPhone OS 14_6 like Mac OS X) AppleWebKit/605.1.15' , 'req-e5f6g7h8' , 'success' , '2022-05-20 13:00:00'),
(3 , 1 , 3    , 'deploy' , 'app'   , '3' , JSON_OBJECT('status', 'stopped') , JSON_OBJECT('status', 'started')                         , JSON_OBJECT('source', 'api', 'operation_id', 'op-f6g7h8i9') , '192.168.1.3' , 'Mozilla/5.0 (iPhone; CPU iPhone OS 14_6 like Mac OS X) AppleWebKit/605.1.15' , 'req-f6g7h8i9' , 'success' , '2022-05-20 18:00:00'),
(1 , 1 , NULL , 'create' , 'space' , '1' , NULL                             , JSON_OBJECT('name', 'production', 'org_id', '1')         , JSON_OBJECT('source', 'api', 'operation_id', 'op-g7h8i9j0') , '192.168.1.1' , 'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36'          , 'req-g7h8i9j0' , 'success' , '2022-05-15 10:00:00'),
(1 , 1 , NULL , 'create' , 'space' , '2' , NULL                             , JSON_OBJECT('name', 'staging', 'org_id', '1')            , JSON_OBJECT('source', 'api', 'operation_id', 'op-h8i9j0k1') , '192.168.1.1' , 'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36'          , 'req-h8i9j0k1' , 'success' , '2022-05-15 11:15:00'),
(1 , 1 , NULL , 'create' , 'space' , '3' , NULL                             , JSON_OBJECT('name', 'development', 'org_id', '1')        , JSON_OBJECT('source', 'api', 'operation_id', 'op-i9j0k1l2') , '192.168.1.1' , 'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36'          , 'req-i9j0k1l2' , 'success' , '2022-05-15 14:00:00'),
(5 , 2 , NULL , 'create' , 'org'   , '2' , NULL                             , JSON_OBJECT('name', 'datastack', 'plan', 'professional') , JSON_OBJECT('source', 'api', 'operation_id', 'op-j0k1l2m3') , '192.168.1.5' , 'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36'          , 'req-j0k1l2m3' , 'success' , '2022-06-15 09:00:00');

INSERT INTO instance_logs (instance_id, app_id, log_type, log_level, message, source, timestamp)
VALUES
(1 , 1 , 'system' , 'info' , 'Container starting'                                , 'app'   , '2024-01-15 10:30:00'),
(1 , 1 , 'system' , 'info' , 'Container started successfully'                    , 'app'   , '2024-01-15 10:30:10'),
(1 , 1 , 'app'    , 'info' , 'Initializing application...'                       , 'app'   , '2024-01-15 10:30:20'),
(1 , 1 , 'app'    , 'info' , 'Connected to database'                             , 'app'   , '2024-01-15 10:30:30'),
(1 , 1 , 'app'    , 'info' , 'Server listening on port 3000'                     , 'app'   , '2024-01-15 10:30:40'),
(1 , 1 , 'app'    , 'info' , 'GET /api/users 200 52ms'                           , 'nginx' , '2024-01-15 10:35:00'),
(1 , 1 , 'app'    , 'info' , 'POST /api/auth/login 200 157ms'                    , 'nginx' , '2024-01-15 10:37:30'),
(1 , 1 , 'app'    , 'info' , 'GET /api/products 200 83ms'                        , 'nginx' , '2024-01-15 10:41:15'),
(1 , 1 , 'app'    , 'warn' , 'Slow query detected (634ms): SELECT * FROM orders' , 'app'   , '2024-01-15 10:45:20'),
(1 , 1 , 'app'    , 'info' , 'Database query completed in 43ms'                  , 'app'   , '2024-01-15 10:50:45'),
(2 , 1 , 'system' , 'info' , 'Container starting'                                , 'app'   , '2024-01-15 10:31:00'),
(2 , 1 , 'system' , 'info' , 'Container started successfully'                    , 'app'   , '2024-01-15 10:31:10'),
(2 , 1 , 'app'    , 'info' , 'Initializing application...'                       , 'app'   , '2024-01-15 10:31:20'),
(2 , 1 , 'app'    , 'info' , 'Connected to database'                             , 'app'   , '2024-01-15 10:31:30'),
(2 , 1 , 'app'    , 'info' , 'Server listening on port 3000'                     , 'app'   , '2024-01-15 10:31:40'),
(2 , 1 , 'app'    , 'info' , 'GET /api/users 200 48ms'                           , 'nginx' , '2024-01-15 10:36:00'),
(2 , 1 , 'app'    , 'info' , 'POST /api/auth/login 200 143ms'                    , 'nginx' , '2024-01-15 10:38:30'),
(2 , 1 , 'app'    , 'info' , 'GET /api/products 200 77ms'                        , 'nginx' , '2024-01-15 10:42:15'),
(2 , 1 , 'app'    , 'info' , 'Database query completed in 38ms'                  , 'app'   , '2024-01-15 10:52:45');

-- Generate a massive number of metrics with NULL instance_id and random data
INSERT INTO metrics (app_id, metric_name, metric_value, labels, timestamp)
SELECT 
     JSON_EXTRACT(JSON_OBJECT('app_id', FLOOR(RAND() * 50 + 1)), '$.app_id') AS app_id, 
     metric_name, 
     RAND() * 100 AS metric_value, 
     JSON_OBJECT('app_id', FLOOR(RAND() * 50 + 1)) AS labels, 
     NOW() AS timestamp
FROM (
     SELECT 'cpu_utilization' AS metric_name
     UNION ALL SELECT 'memory_utilization'
     UNION ALL SELECT 'disk_utilization'
     UNION ALL SELECT 'request_count'
     UNION ALL SELECT 'request_latency'
) AS metric_names
CROSS JOIN (
     SELECT 1 AS n
     UNION ALL SELECT 2
     UNION ALL SELECT 3
     UNION ALL SELECT 4
     UNION ALL SELECT 5
     UNION ALL SELECT 6
     UNION ALL SELECT 7
     UNION ALL SELECT 8
     UNION ALL SELECT 9
     UNION ALL SELECT 10
) AS numbers
LIMIT 5000;

-- Generate additional platform metrics with NULL app_id and random data
INSERT INTO metrics (app_id, metric_name, metric_value, labels, timestamp)
SELECT 
     NULL AS app_id, 
     metric_name, 
     RAND() * 100 AS metric_value, 
     JSON_OBJECT('platform', 'global') AS labels, 
     NOW() AS timestamp
FROM (
     SELECT 'cpu_utilization' AS metric_name
     UNION ALL SELECT 'memory_utilization'
     UNION ALL SELECT 'disk_utilization'
     UNION ALL SELECT 'network_in'
     UNION ALL SELECT 'network_out'
     UNION ALL SELECT 'active_sessions'
     UNION ALL SELECT 'error_rate'
     UNION ALL SELECT 'latency'
) AS metric_names
CROSS JOIN (
     SELECT 1 AS n
     UNION ALL SELECT 2
     UNION ALL SELECT 3
     UNION ALL SELECT 4
     UNION ALL SELECT 5
     UNION ALL SELECT 6
     UNION ALL SELECT 7
     UNION ALL SELECT 8
     UNION ALL SELECT 9
     UNION ALL SELECT 10
) AS numbers
LIMIT 5000;

INSERT INTO network_policies (source_app_id, destination_app_id, protocol, port_range_start, port_range_end, description, enabled, priority, created_at, created_by)
VALUES
(1  , 3  , 'tcp' , 443 , NULL , 'Allow API backend to auth service'    , 1 , 1000 , '2022-05-21 09:00:00' , 1),
(2  , 1  , 'tcp' , 443 , NULL , 'Allow web frontend to API backend'    , 1 , 1000 , '2022-05-21 09:15:00' , 1),
(4  , 3  , 'tcp' , 443 , NULL , 'Allow worker jobs to auth service'    , 1 , 1000 , '2022-05-21 09:30:00' , 1),
(4  , 1  , 'tcp' , 443 , NULL , 'Allow worker jobs to API backend'     , 1 , 1000 , '2022-05-21 09:45:00' , 1),
(9  , 11 , 'tcp' , 443 , NULL , 'Allow data analytics to API gateway'  , 1 , 1000 , '2022-06-26 10:30:00' , 5),
(10 , 9  , 'tcp' , 443 , NULL , 'Allow ML processor to data analytics' , 1 , 1000 , '2022-06-26 10:45:00' , 5),
(10 , 11 , 'tcp' , 443 , NULL , 'Allow ML processor to API gateway'    , 1 , 1000 , '2022-06-26 11:00:00' , 5),
(16 , 17 , 'tcp' , 443 , NULL , 'Allow code service to web portal'     , 1 , 1000 , '2022-07-30 14:00:00' , 8),
(17 , 18 , 'tcp' , 443 , NULL , 'Allow web portal to worker tasks'     , 1 , 1000 , '2022-07-30 14:15:00' , 8),
(18 , 16 , 'tcp' , 443 , NULL , 'Allow worker tasks to code service'   , 1 , 1000 , '2022-07-30 14:30:00' , 8);
-- 2. Populate permissions
-- 3. Populate roles
-- 4. Populate permissions_role
-- System Admin has all permissions
SELECT p.id, r.id
FROM permissions p, roles r
WHERE r.name = 'system_admin';
-- Org Owner has all org-level permissions
SELECT p.id, r.id
FROM permissions p, roles r
WHERE r.name = 'org_owner' 
AND (p.resource_type = 'organization' 
     OR p.resource_type = 'user' 
     OR p.resource_type = 'space'
     OR p.resource_type = 'application'
     OR p.resource_type = 'service'
     OR p.resource_type = 'domain'
     OR p.resource_type = 'route'
     OR p.resource_type = 'quota'
     OR p.resource_type = 'config');
-- 5. Populate regions
-- Add more users (11-25)
-- Add admin users
-- Insert user_meta for each user
-- Admin user metadata
-- Insert user_pii for each user
-- Create some user sessions
-- 7. Populate organizations - Direct inserts
-- Create quotas for each organization
-- Add a special organization for admins
-- Add organization members
-- Add admin users to admin org
-- VALUES
-- (@ORG_COUNT + 1, @USER_COUNT + 1, 'owner', 'accepted'),
-- (@ORG_COUNT + 1, @USER_COUNT + 2, 'admin', 'accepted'),
-- (@ORG_COUNT + 1, @USER_COUNT + 3, 'admin', 'accepted');
-- Add API keys for organizations
-- Add API key for admin org
-- VALUES
-- (@ORG_COUNT + 1, @USER_COUNT + 1, 'Admin API Key', SHA2('admin_api_key_hash', 256), 'admincld', JSON_ARRAY('*'));
-- 8. Populate spaces - Direct inserts
-- Add default spaces for admin org
-- VALUES
-- (@ORG_COUNT + 1, 'production', 'Production Space', 'active', '2020-01-01 00:00:00'),
-- (@ORG_COUNT + 1, 'staging', 'Staging Space', 'active', '2020-01-01 00:00:00'),
-- (@ORG_COUNT + 1, 'development', 'Development Space', 'active', '2020-01-01 00:00:00');
-- 9. Populate nodes
-- 10. Populate data services
-- 11. Populate domains
-- Add system domains
-- 12. Populate apps
-- Continue with more apps 31-50
-- Create health checks for apps
-- Create auto-scaling rules for some apps
-- Add environment variables
-- 13. Populate instances (simplified)
-- Add more instances
-- 14. Populate routes
-- 15. Populate service bindings (simplified)
-- 16. Populate builds (simplified)
-- 17. Populate deployments (simplified)
-- Add some deployment logs
-- 18. Populate tasks (simplified)
-- 19. Populate audit logs (simplified)
-- 20. Populate instance logs (simplified)
-- 21. Populate metrics (simplified)
-- 22. Populate network policies (simplified)

-- Insert notifications
INSERT INTO notifications (user_id, org_id, app_id, notification_type, message, read_status, created_at)
VALUES
(1  , 1 , 1    , 'info'    , 'Application api-backend has been deployed successfully'             , 1 , '2024-02-28 09:15:00'),
(1  , 1 , 1    , 'warning' , 'High CPU usage detected in application api-backend'                 , 0 , '2024-02-28 10:30:00'),
(2  , 1 , 2    , 'success' , 'Application web-frontend deployment completed'                      , 1 , '2024-02-28 11:45:00'),
(3  , 1 , 3    , 'error'   , 'Application auth-service build failed'                              , 1 , '2024-02-28 13:20:00'),
(4  , 1 , 4    , 'info'    , 'New version of worker-jobs is available'                            , 0 , '2024-02-28 14:45:00'),
(5  , 2 , 9    , 'success' , 'Database backup completed for data-analytics'                       , 1 , '2024-02-28 15:30:00'),
(5  , 2 , 10   , 'warning' , 'Memory usage approaching limit in ml-processor'                     , 0 , '2024-02-28 16:15:00'),
(8  , 3 , 16   , 'info'    , 'Auto-scaling event triggered for code-service'                      , 0 , '2024-02-28 17:00:00'),
(14 , 5 , 27   , 'success' , 'AI model training completed successfully'                           , 1 , '2024-02-28 18:30:00'),
(14 , 5 , 28   , 'error'   , 'Service binding failed for data-processor'                          , 0 , '2024-02-28 19:45:00'),
(1  , 1 , 1    , 'warning' , 'SSL certificate expiring in 30 days for api-backend'                , 0 , '2024-02-29 09:00:00'),
(2  , 1 , 2    , 'info'    , 'New security update available for web-frontend'                     , 0 , '2024-02-29 10:15:00'),
(3  , 1 , 3    , 'success' , 'Database migration completed successfully for auth-service'         , 1 , '2024-02-29 11:30:00'),
(5  , 2 , 9    , 'error'   , 'Failed to connect to external service in data-analytics'            , 0 , '2024-02-29 12:45:00'),
(8  , 3 , 16   , 'warning' , 'Unusual traffic pattern detected in code-service'                   , 0 , '2024-02-29 14:00:00'),
(1  , 1 , NULL , 'info'    , 'Organization quota usage at 80%'                                    , 0 , '2024-02-29 15:15:00'),
(5  , 2 , NULL , 'warning' , 'Billing cycle ending in 3 days'                                     , 1 , '2024-02-29 16:30:00'),
(8  , 3 , NULL , 'info'    , 'New feature available: Advanced Monitoring'                         , 0 , '2024-02-29 17:45:00'),
(14 , 5 , NULL , 'success' , 'Organization backup policy updated'                                 , 1 , '2024-02-29 19:00:00'),
(1  , 1 , 1    , 'info'    , 'Performance optimization recommendations available for api-backend' , 0 , '2024-03-01 09:15:00');

INSERT INTO backups (id, name, description, created_at, created_by, backup_type, status, format_version, source_environment, encryption_method, encryption_key_id, size_bytes, has_system_core, has_directors, has_orchestrators, has_network_config, has_app_definitions, has_volume_data, included_apps, included_services, last_validated_at, storage_location, manifest_path, metadata)
VALUES
(1, 'Weekly-Platform-Backup-20250401', 'Weekly full platform backup for disaster recovery', '2025-04-01 03:15:00', 'backup-system', 'PLATFORM',    'AVAILABLE', '3.2', 'production' , 'AES-256-GCM', 101, 52947834880, TRUE,  TRUE,  TRUE,  TRUE,  TRUE,  TRUE, NULL        , NULL        , '2025-04-01 05:30:22', 's3://omnicloud-backups/platform/2025-04-01/',                  's3://omnicloud-backups/platform/2025-04-01/manifest.json', '{"retention_days": 30, "priority": "high", "verification_score": 100}'),
(2, 'CRM-App-Backup-20250405',         'Daily backup of the CRM application',               '2025-04-05 01:30:00', 'app-scheduler', 'APPLICATION', 'AVAILABLE', '3.2', 'production' , 'AES-256-GCM', 101, 8589934592,  FALSE, FALSE, FALSE, FALSE, TRUE,  TRUE, '[1,2,3]'   , NULL        , '2025-04-05 02:15:10', 's3://omnicloud-backups/applications/crm/2025-04-05/',          's3://omnicloud-backups/applications/crm/2025-04-05/manifest.json', '{"retention_days": 14, "priority": "medium", "app_version": "v2.3.1"}'),
(3, 'Partial-Backend-Backup-20250406', 'Partial backup of backend services only',           '2025-04-06 00:15:00', 'admin',         'PARTIAL',     'AVAILABLE', '3.2', 'production' , 'AES-256-GCM', 101, 21474836480, FALSE, TRUE,  TRUE,  TRUE,  TRUE,  TRUE, '[5,6,7,8]' , '[12,15,18]', '2025-04-06 01:45:33', 's3://omnicloud-backups/partial/backend/2025-04-06/',           's3://omnicloud-backups/partial/backend/2025-04-06/manifest.json', '{"retention_days": 7, "priority": "medium", "scope": "backend-services"}'),
(4, 'Database-Only-Backup-20250406',   'Database-only backup for the analytics platform',   '2025-04-06 04:30:00', 'db-scheduler',  'APPLICATION', 'AVAILABLE', '3.2', 'production' , 'AES-256-GCM', 101, 5368709120,  FALSE, FALSE, FALSE, FALSE, FALSE, TRUE, '[11]'      , NULL        , '2025-04-06 05:10:45', 's3://omnicloud-backups/applications/analytics-db/2025-04-06/', 's3://omnicloud-backups/applications/analytics-db/2025-04-06/manifest.json', '{"retention_days": 30, "priority": "high", "db_version": "PostgreSQL 16.2"}'),
(5, 'Dev-Environment-Backup-20250403', 'Full backup of development environment',            '2025-04-03 22:00:00', 'dev-team',      'PLATFORM',    'AVAILABLE', '3.2', 'development', 'AES-256-GCM', 102, 32212254720, TRUE,  TRUE,  TRUE,  TRUE,  TRUE,  TRUE, NULL        , NULL        , '2025-04-03 23:30:17', 's3://omnicloud-backups/platform/dev/2025-04-03/',              's3://omnicloud-backups/platform/dev/2025-04-03/manifest.json', '{"retention_days": 10, "priority": "low", "purpose": "pre-release-snapshot"}');

-- Re-enable foreign key checks and unique checks
SET FOREIGN_KEY_CHECKS = 1;
SET UNIQUE_CHECKS = 1;
-- Output completion message
SELECT 'Sample data generation complete.' AS Message;
SELECT CONCAT('Generated ', @USER_COUNT, ' users') AS Summary
UNION SELECT CONCAT('Generated ', @ORG_COUNT, ' organizations')
UNION SELECT CONCAT('Generated ', @SPACE_COUNT, ' spaces')
UNION SELECT CONCAT('Generated ', @APP_COUNT, ' applications')
UNION SELECT CONCAT('Generated ', @INSTANCE_COUNT, ' instances')
UNION SELECT CONCAT('Generated ', @BUILD_COUNT, ' builds')
UNION SELECT CONCAT('Generated ', @DEPLOYMENT_COUNT, ' deployments')
UNION SELECT CONCAT('Generated ', @TASK_COUNT, ' tasks')
UNION SELECT CONCAT('Generated ', @LOG_COUNT, ' audit logs');