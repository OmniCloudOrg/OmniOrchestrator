-- Sample Data Generation Script for OmniCloud
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
('alibaba', 'Alibaba Cloud', 'cloud', 'active'),
('atlantic', 'Atlantic.Net', 'cloud', 'maintenance'),  
('aws-ec2', 'AWS EC2', 'cloud', 'active'),
('aws-ec2-api', 'AWS EC2 API', 'cloud', 'active'),
('aws-lightsail', 'AWS Lightsail', 'cloud', 'active'),
('azure', 'Microsoft Azure', 'cloud', 'active'),
('azure-api', 'Microsoft Azure API', 'cloud', 'active'),
('buyvm', 'BuyVM', 'cloud', 'active'),
('contabo', 'Contabo', 'cloud', 'active'),
('detee', 'Detee', 'cloud', 'active'),
('digital-ocean', 'DigitalOcean', 'cloud', 'active'),
('dummy', 'Dummy Provider', 'test', 'active'),
('equinix-metal', 'Equinix Metal', 'bare-metal', 'active'),
('exoscale', 'Exoscale', 'cloud', 'active'),
('gcp', 'Google Cloud Platform', 'cloud', 'active'),
('gcp-api', 'Google Cloud Platform API', 'cloud', 'active'),
('hetzner', 'Hetzner', 'cloud', 'active'),
('huawei', 'Huawei Cloud', 'cloud', 'active'),
('hyperv', 'Microsoft Hyper-V', 'hypervisor', 'active'),
('ibm', 'IBM Cloud', 'cloud', 'active'),
('kamatera', 'Kamatera', 'cloud', 'active'),
('kvm', 'KVM', 'hypervisor', 'offline'),
('linode', 'Linode', 'cloud', 'active'),
('oracle-cloud', 'Oracle Cloud', 'cloud', 'active'),
('ovh-cloud', 'OVH Cloud', 'cloud', 'active'),
('proxmox', 'Proxmox', 'hypervisor', 'active'),
('qemu', 'QEMU', 'hypervisor', 'active'),
('rackspace', 'Rackspace', 'cloud', 'active'),
('scaleway', 'Scaleway', 'cloud', 'deprecated'),
('tencent', 'Tencent Cloud', 'cloud', 'active'),
('truenas', 'TrueNAS', 'storage', 'active'),
('upcloud', 'UpCloud', 'cloud', 'active'),
('vb-linux', 'VirtualBox Linux', 'hypervisor', 'active'),
('vb-windows', 'VirtualBox Windows', 'hypervisor', 'active'),
('vmware-esxi', 'VMware ESXi', 'hypervisor', 'active');

INSERT INTO provider_audit_logs (provider_id, action, details, created_at)
VALUES
(1, 'create', 'Added Alibaba Cloud provider with default configuration', '2022-01-15 10:00:00'),
(1, 'update', 'Updated API endpoints for improved reliability', '2022-06-20 14:30:00'),
(2, 'create', 'Added Atlantic.Net provider', '2022-01-15 10:15:00'),
(2, 'update', 'Updated maintenance schedule for planned downtime', '2022-08-15 09:00:00'),
(3, 'create', 'Added AWS EC2 provider with standard configuration', '2022-01-15 10:30:00'),
(3, 'update', 'Enhanced security group configuration', '2022-07-01 11:45:00'),
(4, 'scale', 'Automatically increased capacity due to high demand', '2022-09-10 15:20:00'),
(5, 'create', 'Added AWS Lightsail provider with basic configuration', '2022-01-15 11:00:00'),
(6, 'update', 'Updated Azure authentication mechanisms', '2022-10-05 13:30:00'),
(6, 'deploy', 'Deployed new region configurations', '2022-11-15 16:45:00'),
(7, 'create', 'Added Azure API provider integration', '2022-01-15 11:30:00'),
(8, 'update', 'Updated network configurations for improved performance', '2022-12-01 10:15:00'),
(9, 'create', 'Added Contabo provider with standard setup', '2022-01-15 12:00:00'),
(10, 'delete', 'Temporarily disabled Detee provider for maintenance', '2022-12-15 09:30:00'),
(11, 'create', 'Added DigitalOcean provider with enhanced networking', '2022-01-15 12:30:00');

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
('api-backend'    , 'Api Backend'    , 1 , 1  , 'https://github.com/org1/api-backend'    , 'main'    , 'registry.example.com/org1/api-backend:latest'    , 5 , 1 , 3 , 'http'    , '/health' , 'nodejs' , 'always' , 0 , 'started' , 1 , JSON_OBJECT('environment', 'production', 'team', 'backend')   , '2022-05-20 09:30:00'),
('web-frontend'   , 'Web Frontend'   , 1 , 1  , 'https://github.com/org1/web-frontend'   , 'main'    , 'registry.example.com/org1/web-frontend:latest'   , 4 , 1 , 3 , 'http'    , '/health' , 'nodejs' , 'always' , 0 , 'started' , 1 , JSON_OBJECT('environment', 'production', 'team', 'frontend')  , '2022-05-20 11:45:00'),
('auth-service'   , 'Auth Service'   , 1 , 1  , 'https://github.com/org1/auth-service'   , 'main'    , 'registry.example.com/org1/auth-service:latest'   , 5 , 1 , 2 , 'http'    , '/health' , 'nodejs' , 'always' , 0 , 'started' , 0 , JSON_OBJECT('environment', 'production', 'team', 'backend')   , '2022-05-20 14:15:00'),
('worker-jobs'    , 'Worker Jobs'    , 1 , 1  , 'https://github.com/org1/worker-jobs'    , 'main'    , 'registry.example.com/org1/worker-jobs:latest'    , 6 , 1 , 2 , 'process' , NULL      , 'nodejs' , 'always' , 0 , 'started' , 0 , JSON_OBJECT('environment', 'production', 'team', 'backend')   , '2022-05-21 10:30:00'),
('api-staging'    , 'Api Staging'    , 1 , 2  , 'https://github.com/org1/api-backend'    , 'develop' , NULL                                              , 3 , 1 , 1 , 'http'    , '/health' , 'nodejs' , 'always' , 0 , 'started' , 0 , JSON_OBJECT('environment', 'staging', 'team', 'backend')      , '2022-05-21 13:45:00'),
('web-staging'    , 'Web Staging'    , 1 , 2  , 'https://github.com/org1/web-frontend'   , 'develop' , NULL                                              , 3 , 1 , 1 , 'http'    , '/health' , 'nodejs' , 'always' , 0 , 'started' , 0 , JSON_OBJECT('environment', 'staging', 'team', 'frontend')     , '2022-05-21 15:20:00'),
('api-dev'        , 'Api Dev'        , 1 , 3  , 'https://github.com/org1/api-backend'    , 'develop' , NULL                                              , 2 , 1 , 1 , 'http'    , '/health' , 'nodejs' , 'always' , 0 , 'started' , 0 , JSON_OBJECT('environment', 'development', 'team', 'backend')  , '2022-05-22 09:15:00'),
('web-dev'        , 'Web Dev'        , 1 , 3  , 'https://github.com/org1/web-frontend'   , 'develop' , NULL                                              , 2 , 1 , 1 , 'http'    , '/health' , 'nodejs' , 'always' , 0 , 'started' , 0 , JSON_OBJECT('environment', 'development', 'team', 'frontend') , '2022-05-22 11:30:00'),
('data-analytics' , 'Data Analytics' , 2 , 4  , 'https://github.com/org2/data-analytics' , 'main'    , 'registry.example.com/org2/data-analytics:latest' , 7 , 2 , 2 , 'http'    , '/status' , 'python' , 'always' , 0 , 'started' , 1 , JSON_OBJECT('environment', 'production', 'team', 'data')      , '2022-06-25 10:45:00'),
('ml-processor'   , 'Ml Processor'   , 2 , 4  , 'https://github.com/org2/ml-processor'   , 'main'    , 'registry.example.com/org2/ml-processor:latest'   , 8 , 2 , 2 , 'http'    , '/status' , 'python' , 'always' , 0 , 'started' , 0 , JSON_OBJECT('environment', 'production', 'team', 'data')      , '2022-06-25 14:30:00'),
('api-gateway'    , 'Api Gateway'    , 2 , 4  , 'https://github.com/org2/api-gateway'    , 'main'    , 'registry.example.com/org2/api-gateway:latest'    , 6 , 2 , 2 , 'http'    , '/health' , 'go'     , 'always' , 0 , 'started' , 1 , JSON_OBJECT('environment', 'production', 'team', 'backend')   , '2022-06-26 09:15:00'),
('data-staging'   , 'Data Staging'   , 2 , 5  , 'https://github.com/org2/data-analytics' , 'develop' , NULL                                              , 4 , 2 , 1 , 'http'    , '/status' , 'python' , 'always' , 0 , 'started' , 0 , JSON_OBJECT('environment', 'staging', 'team', 'data')         , '2022-06-26 11:45:00'),
('api-staging'    , 'Api Staging'    , 2 , 5  , 'https://github.com/org2/api-gateway'    , 'develop' , NULL                                              , 4 , 2 , 1 , 'http'    , '/health' , 'go'     , 'always' , 0 , 'started' , 0 , JSON_OBJECT('environment', 'staging', 'team', 'backend')      , '2022-06-26 15:20:00'),
('data-dev'       , 'Data Dev'       , 2 , 6  , 'https://github.com/org2/data-analytics' , 'develop' , NULL                                              , 3 , 2 , 1 , 'http'    , '/status' , 'python' , 'always' , 0 , 'started' , 0 , JSON_OBJECT('environment', 'development', 'team', 'data')     , '2022-06-27 10:30:00'),
('api-dev'        , 'Api Dev'        , 2 , 6  , 'https://github.com/org2/api-gateway'    , 'develop' , NULL                                              , 3 , 2 , 1 , 'http'    , '/health' , 'go'     , 'always' , 0 , 'started' , 0 , JSON_OBJECT('environment', 'development', 'team', 'backend')  , '2022-06-27 14:15:00'),
('code-service'   , 'Code Service'   , 3 , 7  , 'https://github.com/org3/code-service'   , 'main'    , 'registry.example.com/org3/code-service:latest'   , 5 , 3 , 2 , 'http'    , '/health' , 'java'   , 'always' , 0 , 'started' , 0 , JSON_OBJECT('environment', 'production', 'team', 'backend')   , '2022-07-30 09:45:00'),
('web-portal'     , 'Web Portal'     , 3 , 7  , 'https://github.com/org3/web-portal'     , 'main'    , 'registry.example.com/org3/web-portal:latest'     , 4 , 3 , 2 , 'http'    , '/health' , 'nodejs' , 'always' , 0 , 'started' , 1 , JSON_OBJECT('environment', 'production', 'team', 'frontend')  , '2022-07-30 13:30:00'),
('worker-tasks'   , 'Worker Tasks'   , 3 , 7  , 'https://github.com/org3/worker-tasks'   , 'main'    , 'registry.example.com/org3/worker-tasks:latest'   , 5 , 3 , 2 , 'process' , NULL      , 'python' , 'always' , 0 , 'started' , 0 , JSON_OBJECT('environment', 'production', 'team', 'backend')   , '2022-07-31 10:15:00'),
('code-testing'   , 'Code Testing'   , 3 , 8  , 'https://github.com/org3/code-service'   , 'develop' , NULL                                              , 3 , 3 , 1 , 'http'    , '/health' , 'java'   , 'always' , 0 , 'stopped' , 0 , JSON_OBJECT('environment', 'testing', 'team', 'backend')      , '2022-07-31 14:45:00'),
('web-testing'    , 'Web Testing'    , 3 , 8  , 'https://github.com/org3/web-portal'     , 'develop' , NULL                                              , 3 , 3 , 1 , 'http'    , '/health' , 'nodejs' , 'always' , 0 , 'started' , 0 , JSON_OBJECT('environment', 'testing', 'team', 'frontend')     , '2022-08-01 09:30:00'),
('dev-platform'   , 'Dev Platform'   , 4 , 9  , 'https://github.com/org4/dev-platform'   , 'main'    , 'registry.example.com/org4/dev-platform:latest'   , 6 , 4 , 3 , 'http'    , '/health' , 'nodejs' , 'always' , 0 , 'started' , 1 , JSON_OBJECT('environment', 'production', 'team', 'platform')  , '2022-09-05 10:45:00'),
('api-service'    , 'Api Service'    , 4 , 9  , 'https://github.com/org4/api-service'    , 'main'    , 'registry.example.com/org4/api-service:latest'    , 5 , 4 , 2 , 'http'    , '/health' , 'nodejs' , 'always' , 0 , 'started' , 1 , JSON_OBJECT('environment', 'production', 'team', 'backend')   , '2022-09-05 14:20:00'),
('mobile-backend' , 'Mobile Backend' , 4 , 9  , 'https://github.com/org4/mobile-backend' , 'main'    , 'registry.example.com/org4/mobile-backend:latest' , 5 , 4 , 2 , 'http'    , '/status' , 'java'   , 'always' , 0 , 'started' , 0 , JSON_OBJECT('environment', 'production', 'team', 'mobile')    , '2022-09-06 09:30:00'),
('dev-staging'    , 'Dev Staging'    , 4 , 10 , 'https://github.com/org4/dev-platform'   , 'develop' , NULL                                              , 4 , 4 , 1 , 'http'    , '/health' , 'nodejs' , 'always' , 0 , 'started' , 0 , JSON_OBJECT('environment', 'staging', 'team', 'platform')     , '2022-09-06 11:45:00'),
('api-staging'    , 'Api Staging'    , 4 , 10 , 'https://github.com/org4/api-service'    , 'develop' , NULL                                              , 4 , 4 , 1 , 'http'    , '/health' , 'nodejs' , 'always' , 1 , 'crashed' , 0 , JSON_OBJECT('environment', 'staging', 'team', 'backend')      , '2022-09-06 15:20:00'),
('ai-engine'      , 'Ai Engine'      , 5 , 11 , 'https://github.com/org5/ai-engine'      , 'main'    , 'registry.example.com/org5/ai-engine:latest'      , 9 , 5 , 3 , 'http'    , '/health' , 'python' , 'always' , 0 , 'started' , 1 , JSON_OBJECT('environment', 'production', 'team', 'data')      , '2022-10-10 09:15:00'),
('data-processor' , 'Data Processor' , 5 , 11 , 'https://github.com/org5/data-processor' , 'main'    , 'registry.example.com/org5/data-processor:latest' , 8 , 5 , 3 , 'http'    , '/status' , 'python' , 'always' , 0 , 'started' , 1 , JSON_OBJECT('environment', 'production', 'team', 'data')      , '2022-10-10 11:30:00'),
('web-interface'  , 'Web Interface'  , 5 , 11 , 'https://github.com/org5/web-interface'  , 'main'    , 'registry.example.com/org5/web-interface:latest'  , 6 , 5 , 2 , 'http'    , '/health' , 'nodejs' , 'always' , 0 , 'started' , 0 , JSON_OBJECT('environment', 'production', 'team', 'frontend')  , '2022-10-11 13:45:00'),
('ai-dev'         , 'Ai Dev'         , 5 , 12 , 'https://github.com/org5/ai-engine'      , 'develop' , NULL                                              , 4 , 5 , 1 , 'http'    , '/health' , 'python' , 'always' , 0 , 'started' , 0 , JSON_OBJECT('environment', 'development', 'team', 'data')     , '2022-10-11 15:20:00'),
('data-dev'       , 'Data Dev'       , 5 , 12 , 'https://github.com/org5/data-processor' , 'develop' , NULL                                              , 4 , 5 , 1 , 'http'    , '/status' , 'python' , 'always' , 0 , 'started' , 0 , JSON_OBJECT('environment', 'development', 'team', 'data')     , '2022-10-12 10:30:00');

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

INSERT INTO instances (app_id, instance_type, guid, status, region_id, container_id, container_ip, allocation_id, node_id, instance_index, last_health_check, health_status, cpu_usage, memory_usage, disk_usage, uptime, restart_count, start_time, created_at)
VALUES
(1 , 'nodejs' , UUID() , 'running' , 1 , 'container-a1b2c3d4e5f6' , '10.0.0.1'  , 5 , 1  , 0 , NOW() - INTERVAL 5 MINUTE , 'healthy' , 35.2 , 42.8 , 15.3 , 259200 , 0 , '2024-01-15 10:30:00' , '2024-01-15 10:30:00'),
(1 , 'nodejs' , UUID() , 'running' , 1 , 'container-f6e5d4c3b2a1' , '10.0.0.2'  , 5 , 2  , 1 , NOW() - INTERVAL 6 MINUTE , 'healthy' , 42.1 , 51.6 , 17.5 , 259200 , 0 , '2024-01-15 10:31:00' , '2024-01-15 10:31:00'),
(1 , 'nodejs' , UUID() , 'running' , 1 , 'container-1a2b3c4d5e6f' , '10.0.0.3'  , 5 , 3  , 2 , NOW() - INTERVAL 3 MINUTE , 'healthy' , 38.7 , 47.2 , 16.1 , 259200 , 1 , '2024-01-15 11:15:00' , '2024-01-15 10:32:00'),
(2 , 'nodejs' , UUID() , 'running' , 1 , 'container-6f5e4d3c2b1a' , '10.0.0.4'  , 4 , 4  , 0 , NOW() - INTERVAL 4 MINUTE , 'healthy' , 29.8 , 38.3 , 12.5 , 259100 , 0 , '2024-01-15 11:30:00' , '2024-01-15 11:30:00'),
(2 , 'nodejs' , UUID() , 'running' , 1 , 'container-b2a1f6e5d4c3' , '10.0.0.5'  , 4 , 5  , 1 , NOW() - INTERVAL 7 MINUTE , 'healthy' , 31.4 , 40.2 , 13.8 , 259100 , 0 , '2024-01-15 11:31:00' , '2024-01-15 11:31:00'),
(2 , 'nodejs' , UUID() , 'running' , 1 , 'container-5e6f1a2b3c4d' , '10.0.0.6'  , 4 , 6  , 2 , NOW() - INTERVAL 2 MINUTE , 'healthy' , 33.2 , 42.6 , 14.2 , 259000 , 0 , '2024-01-15 11:32:00' , '2024-01-15 11:32:00'),
(3 , 'nodejs' , UUID() , 'running' , 1 , 'container-3c2b1a6f5e4d' , '10.0.0.7'  , 5 , 7  , 0 , NOW() - INTERVAL 8 MINUTE , 'healthy' , 45.3 , 52.7 , 18.4 , 258900 , 0 , '2024-01-15 14:30:00' , '2024-01-15 14:30:00'),
(3 , 'nodejs' , UUID() , 'running' , 1 , 'container-d4c3b2a1f6e5' , '10.0.0.8'  , 5 , 8  , 1 , NOW() - INTERVAL 6 MINUTE , 'healthy' , 43.8 , 50.2 , 17.9 , 258900 , 0 , '2024-01-15 14:31:00' , '2024-01-15 14:31:00'),
(4 , 'nodejs' , UUID() , 'running' , 1 , 'container-4d3c2b1a6f5e' , '10.0.0.9'  , 6 , 9  , 0 , NOW() - INTERVAL 5 MINUTE , 'healthy' , 52.6 , 45.3 , 15.7 , 258800 , 1 , '2024-01-15 14:45:00' , '2024-01-15 10:45:00'),
(4 , 'nodejs' , UUID() , 'running' , 1 , 'container-e5d4c3b2a1f6' , '10.0.0.10' , 6 , 10 , 1 , NOW() - INTERVAL 3 MINUTE , 'healthy' , 48.9 , 42.1 , 14.8 , 258800 , 0 , '2024-01-15 10:46:00' , '2024-01-15 10:46:00');

INSERT INTO instances (app_id, instance_type, guid, status, region_id, container_id, container_ip, allocation_id, node_id, instance_index, last_health_check, health_status, cpu_usage, memory_usage, disk_usage, uptime, restart_count, start_time, created_at)
VALUES
(5  , 'nodejs' , UUID() , 'running' , 2 , 'container-5e4d3c2b1a6f' , '10.0.1.1'  , 3 , 11 , 0 , NOW() - INTERVAL 7 MINUTE , 'healthy' , 22.5 , 31.8 , 10.2 , 172800 , 0 , '2024-01-20 13:30:00' , '2024-01-20 13:30:00'),
(6  , 'nodejs' , UUID() , 'running' , 2 , 'container-f5e4d3c2b1a6' , '10.0.1.2'  , 3 , 12 , 0 , NOW() - INTERVAL 9 MINUTE , 'healthy' , 24.7 , 33.5 , 11.4 , 172700 , 0 , '2024-01-20 15:45:00' , '2024-01-20 15:45:00'),
(7  , 'nodejs' , UUID() , 'running' , 2 , 'container-6f5e4d3c2b1a' , '10.0.1.3'  , 2 , 13 , 0 , NOW() - INTERVAL 4 MINUTE , 'healthy' , 18.2 , 25.3 , 8.7  , 172600 , 0 , '2024-01-21 09:15:00' , '2024-01-21 09:15:00'),
(8  , 'nodejs' , UUID() , 'running' , 2 , 'container-a6f5e4d3c2b1' , '10.0.1.4'  , 2 , 14 , 0 , NOW() - INTERVAL 6 MINUTE , 'healthy' , 17.5 , 26.1 , 9.3  , 172500 , 0 , '2024-01-21 11:30:00' , '2024-01-21 11:30:00'),
(9  , 'python' , UUID() , 'running' , 2 , 'container-1a6f5e4d3c2b' , '10.0.1.5'  , 7 , 15 , 0 , NOW() - INTERVAL 5 MINUTE , 'healthy' , 65.3 , 72.8 , 24.6 , 172400 , 0 , '2024-01-25 10:45:00' , '2024-01-25 10:45:00'),
(9  , 'python' , UUID() , 'running' , 2 , 'container-b1a6f5e4d3c2' , '10.0.1.6'  , 7 , 16 , 1 , NOW() - INTERVAL 7 MINUTE , 'healthy' , 68.2 , 75.4 , 26.1 , 172400 , 0 , '2024-01-25 10:46:00' , '2024-01-25 10:46:00'),
(10 , 'python' , UUID() , 'running' , 2 , 'container-2b1a6f5e4d3c' , '10.0.1.7'  , 8 , 17 , 0 , NOW() - INTERVAL 3 MINUTE , 'healthy' , 71.5 , 78.2 , 27.3 , 172300 , 0 , '2024-01-25 14:30:00' , '2024-01-25 14:30:00'),
(10 , 'python' , UUID() , 'running' , 2 , 'container-c2b1a6f5e4d3' , '10.0.1.8'  , 8 , 18 , 1 , NOW() - INTERVAL 8 MINUTE , 'healthy' , 69.7 , 76.5 , 26.8 , 172300 , 1 , '2024-01-25 15:15:00' , '2024-01-25 14:31:00'),
(11 , 'go'     , UUID() , 'running' , 2 , 'container-3c2b1a6f5e4d' , '10.0.1.9'  , 6 , 19 , 0 , NOW() - INTERVAL 4 MINUTE , 'healthy' , 28.6 , 22.4 , 9.7  , 172200 , 0 , '2024-01-26 09:15:00' , '2024-01-26 09:15:00'),
(11 , 'go'     , UUID() , 'running' , 2 , 'container-d3c2b1a6f5e4' , '10.0.1.10' , 6 , 20 , 1 , NOW() - INTERVAL 6 MINUTE , 'healthy' , 30.1 , 24.8 , 10.3 , 172200 , 0 , '2024-01-26 09:16:00' , '2024-01-26 09:16:00');

INSERT INTO instances (app_id, instance_type, guid, status, region_id, container_id, container_ip, allocation_id, node_id, instance_index, last_health_check, health_status, cpu_usage, memory_usage, disk_usage, uptime, restart_count, start_time, created_at)
VALUES
-- Additional instances for app_id 1
(1, 'nodejs', UUID(), 'running', 1 , 'container-g7h8i9j0k1l2', '10.0.0.11', 5, 11, 3,  NOW() - INTERVAL 4 MINUTE, 'healthy', 36.5, 44.2, 15.8, 259150, 0, '2024-01-15 10:33:00', '2024-01-15 10:33:00'),
(1, 'nodejs', UUID(), 'running', 1 , 'container-m3n4o5p6q7r8', '10.0.0.12', 5, 12, 4,  NOW() - INTERVAL 5 MINUTE, 'healthy', 39.3, 46.8, 16.5, 259120, 0, '2024-01-15 10:34:00', '2024-01-15 10:34:00'),
(1, 'nodejs', UUID(), 'running', 1 , 'container-s9t0u1v2w3x4', '10.0.0.13', 5, 13, 5,  NOW() - INTERVAL 6 MINUTE, 'healthy', 37.8, 45.5, 16.2, 259100, 0, '2024-01-15 10:35:00', '2024-01-15 10:35:00'),
(1, 'nodejs', UUID(), 'running', 1 , 'container-y5z6a7b8c9d0', '10.0.0.14', 5, 14, 6,  NOW() - INTERVAL 4 MINUTE, 'healthy', 38.1, 46.2, 16.3, 259080, 1, '2024-01-15 11:20:00', '2024-01-15 10:36:00'),
(1, 'nodejs', UUID(), 'running', 1 , 'container-e1f2g3h4i5j6', '10.0.0.15', 5, 15, 7,  NOW() - INTERVAL 3 MINUTE, 'healthy', 40.2, 48.4, 16.8, 259060, 0, '2024-01-15 10:37:00', '2024-01-15 10:37:00'),
(1, 'nodejs', UUID(), 'running', 1 , 'container-k7l8m9n0o1p2', '10.0.0.16', 5, 16, 8,  NOW() - INTERVAL 7 MINUTE, 'healthy', 35.9, 43.7, 15.5, 259040, 0, '2024-01-15 10:38:00', '2024-01-15 10:38:00'),
(1, 'nodejs', UUID(), 'running', 1 , 'container-q3r4s5t6u7v8', '10.0.0.17', 5, 17, 9,  NOW() - INTERVAL 5 MINUTE, 'healthy', 37.4, 45.1, 15.9, 259020, 0, '2024-01-15 10:39:00', '2024-01-15 10:39:00'),
(1, 'nodejs', UUID(), 'running', 1 , 'container-w9x0y1z2a3b4', '10.0.0.18', 5, 18, 10, NOW() - INTERVAL 8 MINUTE, 'healthy', 39.8, 47.5, 16.7, 259000, 0, '2024-01-15 10:40:00', '2024-01-15 10:40:00');

INSERT INTO storage_volumes (id, app_id, name, size_gb, storage_class, access_mode, status, node_id, encryption_enabled, persistence_level, write_concern, reclaim_policy, filesystem_type, storage_class_id, created_at, updated_at, snapshot_id, mount_path)
VALUES 
(1,  1,  'api-backend-data',       5,  'fast-local-ssd',     'ReadWriteOnce', 'Mounted',     1 , TRUE, 'High', 'WriteReplicated', 'Delete', 'ext4', 1, '2023-01-15 10:30:00', '2023-01-15 10:30:00', NULL, '/data'),
(2,  1,  'api-backend-logs',       2,  'standard',           'ReadWriteOnce', 'Mounted',     1 , FALSE, 'Basic', 'WriteAcknowledged', 'Delete', 'ext4', 1, '2023-01-15 10:35:00', '2023-01-15 10:35:00', NULL, '/logs'),
(3,  2,  'web-frontend-assets',    8,  'distributed',        'ReadOnlyMany',  'Mounted',     1 , FALSE, 'High', 'WriteDurable', 'Delete', 'xfs', 1, '2023-01-20 11:45:00', '2023-01-20 11:45:00', NULL, '/assets'),
(4,  3,  'auth-service-data',      4,  'fast-local-ssd',     'ReadWriteOnce', 'Mounted',     1 , TRUE, 'High', 'WriteReplicated', 'Delete', 'ext4', 1, '2023-02-05 09:15:00', '2023-02-05 09:15:00', NULL, '/data'),
(5,  4,  'worker-jobs-storage',    10, 'distributed',        'ReadWriteOnce', 'Mounted',     1 , FALSE, 'High', 'WriteDurable', 'Delete', 'ext4', 1, '2023-02-10 14:30:00', '2023-02-10 14:30:00', NULL, '/storage'),
(6,  5,  'api-staging-data',       2,  'standard',           'ReadWriteOnce', 'Mounted',     1 , FALSE, 'Basic', 'WriteAcknowledged', 'Delete', 'ext4', 1, '2023-02-15 16:20:00', '2023-02-15 16:20:00', NULL, '/data'),
(7,  7,  'api-dev-data',           1,  'local-disk',         'ReadWriteOnce', 'Mounted',     1 , FALSE, 'Basic', 'WriteAcknowledged', 'Delete', 'ext4', 1, '2023-03-01 10:00:00', '2023-03-01 10:00:00', NULL, '/data'),
(8,  9,  'data-analytics-storage', 20, 'distributed',        'ReadWriteMany', 'Mounted',     1 , TRUE, 'High', 'WriteReplicated', 'Retain', 'xfs', 1, '2023-03-15 13:45:00', '2023-03-15 13:45:00', NULL, '/storage'),
(9,  10, 'ml-processor-models',    15, 'performance-small',  'ReadWriteOnce', 'Mounted',     1 , TRUE, 'High', 'WriteReplicated', 'Retain', 'ext4', 1, '2023-03-20 09:30:00', '2023-03-20 09:30:00', NULL, '/models'),
(10, 16, 'code-service-repos',     8,  'distributed',        'ReadWriteMany', 'Mounted',     1 , FALSE, 'High', 'WriteDurable', 'Delete', 'xfs', 1, '2023-04-10 11:15:00', '2023-04-10 11:15:00', NULL, '/repos'),
(11, 5,  'api-staging-backup',     3,  'standard',           'ReadWriteOnce', 'Provisioned', 1 , TRUE, 'Enhanced', 'WriteDurable', 'Retain', 'ext4', 1, '2023-02-20 14:30:00', '2023-02-20 14:30:00', NULL, NULL),
(12, 9,  'data-analytics-archive', 30, 'distributed',        'ReadOnlyMany',  'Provisioned', 1 , TRUE, 'Maximum', 'WriteDistributed', 'Retain', 'xfs', 1, '2023-03-25 16:45:00', '2023-03-25 16:45:00', NULL, NULL),
(13, 22, 'dev-platform-data',      4,  'local-disk',         'ReadWriteOnce', 'Deleting',    1 , FALSE, 'Basic', 'WriteAcknowledged', 'Delete', 'ext4', 1, '2023-04-15 10:30:00', '2023-05-10 09:15:00', NULL, NULL),
(14, 27, 'ai-engine-models',       25, 'performance-medium', 'ReadWriteOnce', 'Mounted',     1 , TRUE, 'High', 'WriteReplicated', 'Retain', 'xfs', 1, '2023-04-20 13:45:00', '2023-04-20 13:45:00', NULL, '/models'),
(15, 28, 'data-processor-cache',   12, 'local-ssd',          'ReadWriteOnce', 'Mounted',     1 , FALSE, 'Basic', 'WriteAcknowledged', 'Delete', 'ext4', 1, '2023-04-25 15:20:00', '2023-04-25 15:20:00', NULL, '/cache'),
(16, 1,  'api-backend-snapshots',  7,  'standard',           'ReadWriteOnce', 'Provisioned', 1 , TRUE, 'High', 'WriteDurable', 'Retain', 'ext4', 1, '2023-05-05 11:30:00', '2023-05-05 11:30:00', 1, NULL),
(17, 2,  'web-frontend-backups',   6,  'standard',           'ReadWriteOnce', 'Provisioned', 1 , TRUE, 'Enhanced', 'WriteDurable', 'Retain', 'ext4', 1, '2023-05-10 14:15:00', '2023-05-10 14:15:00', 3, NULL),
(18, 10, 'ml-processor-training',  18, 'performance-small',  'ReadWriteMany', 'Provisioned', 1 , TRUE, 'High', 'WriteReplicated', 'Delete', 'xfs', 1, '2023-05-15 09:45:00', '2023-05-15 09:45:00', NULL, NULL),
(19, 3,  'auth-service-certs',     1,  'local-disk',         'ReadOnlyMany',  'Mounted',     1 , TRUE, 'High', 'WriteReplicated', 'Delete', 'ext4', 1, '2023-05-20 13:00:00', '2023-05-20 13:00:00', NULL, '/certs'),
(20, 20, 'unused-volume',          5,  'standard',           'ReadWriteOnce', 'Deleted',     1 , FALSE, 'Basic', 'WriteAcknowledged', 'Delete', 'ext4', 1, '2023-05-25 10:30:00', '2023-06-01 15:45:00', NULL, NULL);

INSERT INTO storage_snapshots (id, volume_id, name, size_gb, created_at, status, description, retention_date)
VALUES
(1, 1, 'api-backend-data-daily-20230504', 5, '2023-05-04 01:00:00', 'Available', 'Daily backup of API backend data', '2023-05-11 01:00:00'),
(2, 4, 'auth-service-data-daily-20230504', 4, '2023-05-04 01:15:00', 'Available', 'Daily backup of Auth service data', '2023-05-11 01:15:00'),
(3, 3, 'web-frontend-assets-weekly-20230507', 8, '2023-05-07 02:00:00', 'Available', 'Weekly backup of web frontend assets', '2023-06-07 02:00:00'),
(4, 8, 'data-analytics-storage-daily-20230504', 20, '2023-05-04 01:30:00', 'Available', 'Daily backup of data analytics storage', '2023-05-11 01:30:00'),
(5, 9, 'ml-processor-models-daily-20230504', 15, '2023-05-04 01:45:00', 'Available', 'Daily backup of ML processor models', '2023-05-11 01:45:00'),
(6, 14, 'ai-engine-models-daily-20230504', 25, '2023-05-04 02:00:00', 'Available', 'Daily backup of AI engine models', '2023-05-11 02:00:00'),
(7, 1, 'api-backend-data-daily-20230505', 5, '2023-05-05 01:00:00', 'Available', 'Daily backup of API backend data', '2023-05-12 01:00:00'),
(8, 4, 'auth-service-data-daily-20230505', 4, '2023-05-05 01:15:00', 'Available', 'Daily backup of Auth service data', '2023-05-12 01:15:00'),
(9, 8, 'data-analytics-storage-daily-20230505', 20, '2023-05-05 01:30:00', 'Available', 'Daily backup of data analytics storage', '2023-05-12 01:30:00'),
(10, 9, 'ml-processor-models-daily-20230505', 15, '2023-05-05 01:45:00', 'Available', 'Daily backup of ML processor models', '2023-05-12 01:45:00');

INSERT INTO storage_migrations (id, source_volume_id, destination_volume_id, migration_type, status, progress_percent, started_at, completed_at, is_online, error_message, created_by)
VALUES
(1, 2, 16, 'StorageClass', 'Completed', 100, '2023-05-01 09:00:00', '2023-05-01 09:30:00', TRUE, NULL, 'system'),
(2, 6, 11, 'StorageClass', 'Completed', 100, '2023-02-20 13:00:00', '2023-02-20 14:00:00', TRUE, NULL, 'admin'),
(3, 7, 13, 'Node', 'Failed', 65, '2023-04-10 15:00:00', '2023-04-10 15:45:00', TRUE, 'Connection timeout during final sync', 'admin'),
(4, 8, 12, 'Zone', 'Completed', 100, '2023-03-25 14:00:00', '2023-03-25 16:30:00', FALSE, NULL, 'admin'),
(5, 9, 18, 'StorageClass', 'Completed', 100, '2023-05-15 08:00:00', '2023-05-15 09:30:00', FALSE, NULL, 'system');

INSERT INTO storage_qos_policies (id, name, max_iops, max_throughput_mbps, burst_iops, burst_duration_seconds, latency_target_ms, created_at, updated_at)
VALUES
(1, 'high-performance', 5000, 500, 10000, 300, 5, '2023-01-01 00:00:00', '2023-01-01 00:00:00'),
(2, 'standard', 1000, 100, 2000, 300, 10, '2023-01-01 00:00:00', '2023-01-01 00:00:00'),
(3, 'economy', 500, 50, 1000, 300, 20, '2023-01-01 00:00:00', '2023-01-01 00:00:00'),
(4, 'database-optimized', 3000, 300, 6000, 300, 5, '2023-01-01 00:00:00', '2023-01-01 00:00:00'),
(5, 'analytics-optimized', 2000, 400, 4000, 600, 15, '2023-01-01 00:00:00', '2023-01-01 00:00:00');

INSERT INTO volume_qos_policy_assignments (volume_id, policy_id, assigned_at)
VALUES
(1, 1, '2023-01-15 10:30:00'),
(3, 2, '2023-01-20 11:45:00'),
(4, 1, '2023-02-05 09:15:00'),
(5, 2, '2023-02-10 14:30:00'),
(8, 5, '2023-03-15 13:45:00'),
(9, 5, '2023-03-20 09:30:00'),
(14, 1, '2023-04-20 13:45:00'),
(15, 3, '2023-04-25 15:20:00');

INSERT INTO storage_classes (name, provisioner, reclaim_policy, volume_binding_mode, allow_volume_expansion, storage_type, default_filesystem, created_at, updated_at)
VALUES
('standard', 'omnicloud.io/distributed', 'Delete', 'Immediate', TRUE, 'distributed', 'ext4', '2023-01-01 00:00:00', '2023-01-01 00:00:00'),
('fast-local-ssd', 'omnicloud.io/local-ssd', 'Delete', 'WaitForFirstConsumer', TRUE, 'local-resilient', 'ext4', '2023-01-01 00:00:00', '2023-01-01 00:00:00'),
('local-disk', 'omnicloud.io/local-disk', 'Delete', 'WaitForFirstConsumer', FALSE, 'local-disk', 'ext4', '2023-01-01 00:00:00', '2023-01-01 00:00:00'),
('distributed', 'omnicloud.io/distributed', 'Delete', 'Immediate', TRUE, 'distributed', 'xfs', '2023-01-01 00:00:00', '2023-01-01 00:00:00'),
('performance-small', 'omnicloud.io/performance', 'Delete', 'Immediate', TRUE, 'distributed', 'ext4', '2023-01-01 00:00:00', '2023-01-01 00:00:00'),
('performance-medium', 'omnicloud.io/performance', 'Delete', 'Immediate', TRUE, 'distributed', 'xfs', '2023-01-01 00:00:00', '2023-01-01 00:00:00'),
('performance-large', 'omnicloud.io/performance', 'Delete', 'Immediate', TRUE, 'distributed', 'xfs', '2023-01-01 00:00:00', '2023-01-01 00:00:00'),
('geo-replicated', 'omnicloud.io/geo-replicated', 'Retain', 'Immediate', TRUE, 'geo-replicated', 'ext4', '2023-01-01 00:00:00', '2023-01-01 00:00:00'),
('local-ssd', 'omnicloud.io/local-ssd', 'Delete', 'WaitForFirstConsumer', TRUE, 'local-resilient', 'ext4', '2023-01-01 00:00:00', '2023-01-01 00:00:00'),
('archive', 'omnicloud.io/archive', 'Retain', 'Immediate', FALSE, 'distributed', 'ext4', '2023-01-01 00:00:00', '2023-01-01 00:00:00');

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
     JSON_EXTRACT(JSON_OBJECT('app_id', FLOOR(RAND() * 25 + 1)), '$.app_id') AS app_id, 
     metric_name, 
     RAND() * 25 AS metric_value, 
     JSON_OBJECT('app_id', FLOOR(RAND() * 25 + 1)) AS labels, 
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
     RAND() * 25 AS metric_value, 
     JSON_OBJECT('platform', 'global') AS labels, 
     time_point AS timestamp
FROM (
     SELECT 'cpu_utilization' AS metric_name
     UNION ALL SELECT 'memory_utilization'
     UNION ALL SELECT 'disk_utilization'
     UNION ALL SELECT 'network_in'
     UNION ALL SELECT 'network_out'
     UNION ALL SELECT 'active_sessions'
     UNION ALL SELECT 'error_rate'
     UNION ALL SELECT 'latency'
     UNION ALL SELECT 'request_count'
     UNION ALL SELECT 'queue_depth'
     UNION ALL SELECT 'cache_hit_ratio'
     UNION ALL SELECT 'database_connections'
) AS metric_names
CROSS JOIN (
     SELECT DATE_ADD(NOW(), INTERVAL - (n * 10) SECOND) AS time_point
     FROM (
         WITH RECURSIVE time_series AS (
             SELECT 1 AS n
             UNION ALL
             SELECT n + 1 FROM time_series WHERE n < 500
         )
         SELECT n FROM time_series
     ) AS time_points
) AS times;

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

-- Insert sample user notifications
INSERT INTO user_notifications 
(user_id, org_id, app_id, notification_type, message, read_status, importance, action_url, action_label, created_at, expires_at)
VALUES 
(1, 2, 3, 'system_update', 'System maintenance scheduled for tomorrow at 2 AM UTC', FALSE, 'high', '/maintenance/details', 'View Details', '2025-04-15 10:00:00', '2025-04-20 10:00:00'),
(2, 2, 3, 'account_security', 'Your password will expire in 3 days', FALSE, 'high', '/account/password', 'Change Password', '2025-04-17 09:15:00', '2025-04-21 00:00:00'),
(1, 2, 4, 'new_feature', 'Try our new dashboard analytics feature', TRUE, 'normal', '/dashboard/analytics', 'Try Now', '2025-04-12 14:30:00', '2025-05-12 14:30:00'),
(3, 5, 5, 'billing_update', 'Your invoice for April is now available', FALSE, 'normal', '/billing/invoices/april2025', 'View Invoice', '2025-04-18 00:01:00', '2025-05-18 00:01:00'),
(4, 5, 6, 'performance_alert', 'Your app is experiencing high CPU usage', FALSE, 'critical', '/monitoring/performance/app/6', 'View Metrics', '2025-04-19 03:22:15', NULL);

-- Insert sample role notifications
INSERT INTO role_notifications 
(role_id, org_id, app_id, notification_type, message, importance, action_url, action_label, created_at, expires_at)
VALUES 
(1, 2, NULL, 'policy_update', 'Security policy updated: Review required by all admins', 'high', '/policy/security/2025-04', 'Review Policy', '2025-04-10 11:00:00', '2025-04-30 11:00:00'),
(2, 5, NULL, 'permission_change', 'Developer permissions updated for cloud resources', 'normal', '/permissions/changelog', 'View Changes', '2025-04-15 16:45:00', '2025-05-15 16:45:00'),
(3, 2, 3, 'deployment_required', 'Critical update needs deployment approval', 'critical', '/deployments/pending/3542', 'Review & Approve', '2025-04-19 09:30:00', NULL),
(4, 5, 6, 'compliance_training', 'New compliance training available for all managers', 'normal', '/training/compliance/2025', 'Start Training', '2025-04-16 10:00:00', '2025-06-16 10:00:00'),
(1, 2, 4, 'quarterly_meeting', 'Admin quarterly meeting scheduled for next Friday', 'normal', '/calendar/meetings/admin-q2', 'Add to Calendar', '2025-04-18 13:20:00', '2025-04-26 15:00:00');

-- Insert notification acknowledgments
INSERT INTO notification_acknowledgments 
(user_id, notification_id, role_notification_id, acknowledged_at)
VALUES 
(1, 3, NULL, '2025-04-12 15:45:22'),
(2, NULL, 1, '2025-04-11 08:30:15'),
(3, 4, NULL, '2025-04-18 10:22:33'),
(4, NULL, 2, '2025-04-16 09:15:40'),
(1, NULL, 5, '2025-04-18 14:05:12');

INSERT INTO alerts 
(alert_type, severity, service, message, timestamp, status, resolved_at, resolved_by, metadata, org_id, app_id, instance_id, region_id, node_id)
VALUES 
('high_cpu', 'warning', 'compute', 'Instance CPU usage exceeding 90% for over 15 minutes', '2025-04-18 23:15:10', 'active', NULL, NULL, '{"cpu_usage": 92.5, "duration_minutes": 18}', 2, 3, 10, 1, 5),
('memory_leak', 'critical', 'app_service', 'Possible memory leak detected in production service', '2025-04-19 01:30:22', 'acknowledged', NULL, NULL, '{"memory_growth_rate": "2MB/min", "process_id": 4588}', 5, 6, 11, 2, 7),
('disk_space', 'warning', 'storage', 'Database storage approaching 85% capacity', '2025-04-17 10:45:33', 'resolved', '2025-04-17 14:20:15', 2, '{"disk_usage": 85.2, "growth_rate": "500MB/day"}', 2, 4, 12, 1, NULL),
('api_latency', 'info', 'api_gateway', 'API response time increased by 35%', '2025-04-18 16:22:45', 'auto_resolved', '2025-04-18 16:55:30', NULL, '{"avg_response_ms": 320, "baseline_ms": 230}', 5, 6, NULL, 2, 8),
('security_event', 'critical', 'auth_service', 'Multiple failed login attempts detected from unusual location', '2025-04-19 04:10:05', 'active', NULL, NULL, '{"attempts": 12, "ip": "203.0.113.42", "location": "Unknown"}', 2, NULL, NULL, NULL, NULL),
('network_outage', 'critical', 'network', 'Network connectivity lost in region us-east-1', '2025-04-20 02:15:00', 'active', NULL, NULL, '{"affected_nodes": 15, "region": "us-east-1"}', 3, NULL, NULL, 1, NULL),
('service_crash', 'critical', 'app_service', 'Critical service crashed unexpectedly', '2025-04-20 03:45:00', 'active', NULL, NULL, '{"service_name": "auth-service", "error_code": 500}', 4, 7, 13, 2, 9),
('high_memory', 'warning', 'compute', 'Memory usage exceeded 85% for over 10 minutes', '2025-04-20 04:30:00', 'active', NULL, NULL, '{"memory_usage": 87.3, "duration_minutes": 12}', 5, 8, 14, 3, 10),
('ssl_expiry', 'info', 'security', 'SSL certificate expiring in 15 days', '2025-04-20 05:00:00', 'active', NULL, NULL, '{"certificate_name": "api.example.com", "expiry_date": "2025-05-05"}', 6, NULL, NULL, NULL, NULL),
('database_error', 'critical', 'database', 'Frequent database connection errors detected', '2025-04-20 06:15:00', 'active', NULL, NULL, '{"error_rate": "15/min", "database": "analytics-db"}', 7, 9, 15, 4, 11);


-- Insert alert acknowledgments
INSERT INTO alert_acknowledgments 
(alert_id, user_id, acknowledged_at, notes)
VALUES 
(2, 3, '2025-04-19 01:45:18', 'Investigating the memory growth pattern'),
(1, 1, '2025-04-19 00:05:30', 'Monitoring - will add more capacity if trend continues'),
(5, 2, '2025-04-19 04:15:22', 'Security team notified, implementing IP block'),
(3, 2, '2025-04-17 11:00:12', 'Added additional storage to prevent capacity issues'),
(4, 4, '2025-04-18 16:35:10', NULL);

-- Insert alert escalations
INSERT INTO alert_escalations 
(alert_id, escalation_level, escalated_at, escalated_to, escalation_method, response_required_by)
VALUES 
(1, 1, '2025-04-18 23:30:15', '{"roles": [3], "users": [1,4]}', 'email', '2025-04-19 00:30:15'),
(2, 2, '2025-04-19 02:00:22', '{"roles": [2], "users": [3,5]}', 'sms', '2025-04-19 03:00:22'),
(5, 1, '2025-04-19 04:25:05', '{"roles": [1], "users": [2]}', 'email', '2025-04-19 05:25:05'),
(5, 2, '2025-04-19 05:30:05', '{"roles": [1], "users": [2,6]}', 'sms', '2025-04-19 06:00:05'),
(3, 1, '2025-04-17 11:15:33', '{"roles": [3], "users": [2]}', 'email', '2025-04-17 13:15:33');

-- Insert alert history
INSERT INTO alert_history 
(alert_id, action, performed_by, performed_at, previous_state, new_state, notes)
VALUES 
(1, 'created', NULL, '2025-04-18 23:15:10', NULL, '{"status": "active", "severity": "warning"}', 'Automatically created by monitoring system'),
(2, 'created', NULL, '2025-04-19 01:30:22', NULL, '{"status": "active", "severity": "critical"}', 'Automatically created by memory monitor'),
(2, 'acknowledged', 3, '2025-04-19 01:45:18', '{"status": "active"}', '{"status": "acknowledged"}', 'Engineering team is investigating'),
(3, 'created', NULL, '2025-04-17 10:45:33', NULL, '{"status": "active", "severity": "warning"}', 'Automatically created by storage monitor'),
(3, 'acknowledged', 2, '2025-04-17 11:00:12', '{"status": "active"}', '{"status": "acknowledged"}', 'Adding more storage'),
(3, 'resolved', 2, '2025-04-17 14:20:15', '{"status": "acknowledged"}', '{"status": "resolved"}', 'Added 500GB to database storage'),
(4, 'created', NULL, '2025-04-18 16:22:45', NULL, '{"status": "active", "severity": "info"}', 'Automatically created by API monitor'),
(4, 'auto_resolved', NULL, '2025-04-18 16:55:30', '{"status": "active"}', '{"status": "auto_resolved"}', 'API response times returned to normal'),
(5, 'created', NULL, '2025-04-19 04:10:05', NULL, '{"status": "active", "severity": "critical"}', 'Automatically created by security service'),
(5, 'acknowledged', 2, '2025-04-19 04:15:22', '{"status": "active"}', '{"status": "acknowledged"}', 'Security team investigating');

INSERT INTO nodes (worker_id, node_type, hostname, ip_address, cpu_cores, ram_gb, disk_gb, status) VALUES
-- Region 1 nodes
(1, 'master', 'master-node-001.region1', '10.0.1.10', 16, 64, 500, 'active'),
(1, 'worker', 'worker-node-001.region1', '10.0.1.20', 32, 128, 1000, 'active'),
(2, 'worker', 'worker-node-002.region1', '10.0.1.21', 32, 128, 1000, 'active'),
(3, 'storage', 'storage-node-001.region1', '10.0.1.30', 16, 64, 4000, 'active'),
(4, 'edge', 'edge-node-001.region1', '10.0.1.40', 8, 32, 250, 'active');

-- Populate resource_types table with common cloud resources
INSERT INTO resource_types (name, category, unit_of_measurement, description) VALUES
('cpu_usage', 'compute', 'vCPU-hour', 'Virtual CPU usage per hour'),
('memory_usage', 'compute', 'GB-hour', 'Memory usage per hour'),
('disk_usage', 'storage', 'GB-month', 'Persistent disk storage per month'),
('network_egress', 'network', 'GB', 'Outbound data transfer'),
('network_ingress', 'network', 'GB', 'Inbound data transfer'),
('database_storage', 'database', 'GB-month', 'Database storage per month'),
('database_iops', 'database', 'million-iops', 'Database I/O operations per second'),
('snapshot_storage', 'storage', 'GB-month', 'Snapshot storage per month'),
('load_balancer', 'network', 'hour', 'Load balancer operating hours'),
('api_requests', 'service', 'million-requests', 'API requests processed'),
('container_runtime', 'compute', 'container-hour', 'Container runtime per hour'),
('gpu_usage', 'compute', 'GPU-hour', 'GPU usage per hour'),
('cache_usage', 'service', 'GB-hour', 'In-memory cache usage per hour'),
('queue_messages', 'service', 'million-messages', 'Queue messages processed'),
('dns_queries', 'network', 'million-queries', 'DNS queries processed');

-- Sample resource pricing for different providers
INSERT INTO resource_pricing (resource_type_id, provider_id, region_id, tier_name, unit_price, currency, 
                            effective_from, effective_to, pricing_model, commitment_period, volume_discount_tiers) VALUES
-- Provider 1 pricing (AWS-like)
(1, 1, 1, 'standard', 0.0456, 'USD', '2025-01-01 00:00:00', NULL, 'on-demand', NULL, 
 '{"tiers": [{"min": 0, "max": 1000, "discount": 0}, {"min": 1001, "max": 10000, "discount": 10}, {"min": 10001, "max": null, "discount": 20}]}'),
(2, 1, 1, 'standard', 0.0125, 'USD', '2025-01-01 00:00:00', NULL, 'on-demand', NULL, 
 '{"tiers": [{"min": 0, "max": 1000, "discount": 0}, {"min": 1001, "max": 10000, "discount": 5}, {"min": 10001, "max": null, "discount": 15}]}'),
(3, 1, 1, 'standard', 0.10, 'USD', '2025-01-01 00:00:00', NULL, 'on-demand', NULL, 
 '{"tiers": [{"min": 0, "max": 1000, "discount": 0}, {"min": 1001, "max": null, "discount": 10}]}'),
(4, 1, 1, 'standard', 0.09, 'USD', '2025-01-01 00:00:00', NULL, 'on-demand', NULL, NULL),

-- Provider 2 pricing (GCP-like)
(1, 2, 2, 'standard', 0.0438, 'USD', '2025-01-01 00:00:00', NULL, 'on-demand', NULL, 
 '{"tiers": [{"min": 0, "max": 1000, "discount": 0}, {"min": 1001, "max": null, "discount": 15}]}'),
(2, 2, 2, 'standard', 0.0118, 'USD', '2025-01-01 00:00:00', NULL, 'on-demand', NULL, 
 '{"tiers": [{"min": 0, "max": 1000, "discount": 0}, {"min": 1001, "max": null, "discount": 10}]}'),
(3, 2, 2, 'standard', 0.08, 'USD', '2025-01-01 00:00:00', NULL, 'on-demand', NULL, NULL),
(4, 2, 2, 'standard', 0.085, 'USD', '2025-01-01 00:00:00', NULL, 'on-demand', NULL, NULL),

-- Provider 3 pricing (Azure-like)
(1, 3, 3, 'standard', 0.0460, 'USD', '2025-01-01 00:00:00', NULL, 'on-demand', NULL, NULL),
(2, 3, 3, 'standard', 0.0127, 'USD', '2025-01-01 00:00:00', NULL, 'on-demand', NULL, NULL),
(3, 3, 3, 'standard', 0.095, 'USD', '2025-01-01 00:00:00', NULL, 'on-demand', NULL, NULL),
(4, 3, 3, 'standard', 0.087, 'USD', '2025-01-01 00:00:00', NULL, 'on-demand', NULL, NULL),

-- Reserved instance pricing examples
(1, 1, 1, 'reserved', 0.0290, 'USD', '2025-01-01 00:00:00', NULL, 'reserved', '1-year', NULL),
(1, 1, 1, 'reserved', 0.0210, 'USD', '2025-01-01 00:00:00', NULL, 'reserved', '3-year', NULL),
(1, 2, 2, 'reserved', 0.0280, 'USD', '2025-01-01 00:00:00', NULL, 'reserved', '1-year', NULL),
(1, 3, 3, 'reserved', 0.0295, 'USD', '2025-01-01 00:00:00', NULL, 'reserved', '1-year', NULL),

-- Spot/preemptible pricing examples
(1, 1, 1, 'spot', 0.0137, 'USD', '2025-01-01 00:00:00', NULL, 'spot', NULL, NULL),
(1, 2, 2, 'preemptible', 0.0131, 'USD', '2025-01-01 00:00:00', NULL, 'spot', NULL, NULL),
(1, 3, 3, 'spot', 0.0138, 'USD', '2025-01-01 00:00:00', NULL, 'spot', NULL, NULL),

-- Historical price changes (showing how prices have decreased over time)
(1, 1, 1, 'standard', 0.0520, 'USD', '2024-01-01 00:00:00', '2024-12-31 23:59:59', 'on-demand', NULL, NULL),
(2, 1, 1, 'standard', 0.0145, 'USD', '2024-01-01 00:00:00', '2024-12-31 23:59:59', 'on-demand', NULL, NULL),
(1, 2, 2, 'standard', 0.0490, 'USD', '2024-01-01 00:00:00', '2024-12-31 23:59:59', 'on-demand', NULL, NULL),
(1, 3, 3, 'standard', 0.0510, 'USD', '2024-01-01 00:00:00', '2024-12-31 23:59:59', 'on-demand', NULL, NULL);

-- Sample cost metrics data (assumes you have apps, regions, providers, workers, and orgs already defined)
INSERT INTO cost_metrics (resource_type_id, provider_id, region_id, app_id, worker_id, org_id, 
                        start_time, end_time, usage_quantity, unit_cost, currency, total_cost, 
                        discount_percentage, discount_reason, billing_period) VALUES
-- App 1 metrics for different resources over time
(1, 1, 1, 1, 1, 1, '2025-05-01 00:00:00', '2025-05-01 01:00:00', 8.0, 0.0456, 'USD', 0.3648, 0, NULL, '2025-05'),
(2, 1, 1, 1, 1, 1, '2025-05-01 00:00:00', '2025-05-01 01:00:00', 16.0, 0.0125, 'USD', 0.2000, 0, NULL, '2025-05'),
(3, 1, 1, 1, 1, 1, '2025-05-01 00:00:00', '2025-05-01 01:00:00', 100.0, 0.10, 'USD', 10.0000, 0, NULL, '2025-05'),
(4, 1, 1, 1, 1, 1, '2025-05-01 00:00:00', '2025-05-01 01:00:00', 5.2, 0.09, 'USD', 0.4680, 0, NULL, '2025-05'),

-- App 1, next hour
(1, 1, 1, 1, 1, 1, '2025-05-01 01:00:00', '2025-05-01 02:00:00', 8.0, 0.0456, 'USD', 0.3648, 0, NULL, '2025-05'),
(2, 1, 1, 1, 1, 1, '2025-05-01 01:00:00', '2025-05-01 02:00:00', 16.5, 0.0125, 'USD', 0.2063, 0, NULL, '2025-05'),
(3, 1, 1, 1, 1, 1, '2025-05-01 01:00:00', '2025-05-01 02:00:00', 100.0, 0.10, 'USD', 10.0000, 0, NULL, '2025-05'),
(4, 1, 1, 1, 1, 1, '2025-05-01 01:00:00', '2025-05-01 02:00:00', 6.1, 0.09, 'USD', 0.5490, 0, NULL, '2025-05'),

-- App 2 metrics
(1, 1, 1, 2, 2, 1, '2025-05-01 00:00:00', '2025-05-01 01:00:00', 4.0, 0.0456, 'USD', 0.1824, 0, NULL, '2025-05'),
(2, 1, 1, 2, 2, 1, '2025-05-01 00:00:00', '2025-05-01 01:00:00', 8.0, 0.0125, 'USD', 0.1000, 0, NULL, '2025-05'),
(3, 1, 1, 2, 2, 1, '2025-05-01 00:00:00', '2025-05-01 01:00:00', 50.0, 0.10, 'USD', 5.0000, 0, NULL, '2025-05'),
(4, 1, 1, 2, 2, 1, '2025-05-01 00:00:00', '2025-05-01 01:00:00', 2.8, 0.09, 'USD', 0.2520, 0, NULL, '2025-05'),

-- App 3 metrics (different provider)
(1, 2, 2, 3, 3, 2, '2025-05-01 00:00:00', '2025-05-01 01:00:00', 16.0, 0.0438, 'USD', 0.7008, 0, NULL, '2025-05'),
(2, 2, 2, 3, 3, 2, '2025-05-01 00:00:00', '2025-05-01 01:00:00', 32.0, 0.0118, 'USD', 0.3776, 0, NULL, '2025-05'),
(3, 2, 2, 3, 3, 2, '2025-05-01 00:00:00', '2025-05-01 01:00:00', 200.0, 0.08, 'USD', 16.0000, 0, NULL, '2025-05'),
(4, 2, 2, 3, 3, 2, '2025-05-01 00:00:00', '2025-05-01 01:00:00', 15.6, 0.085, 'USD', 1.3260, 0, NULL, '2025-05'),

-- App 4 metrics (another provider)
(1, 3, 3, 4, 4, 2, '2025-05-01 00:00:00', '2025-05-01 01:00:00', 8.0, 0.0460, 'USD', 0.3680, 0, NULL, '2025-05'),
(2, 3, 3, 4, 4, 2, '2025-05-01 00:00:00', '2025-05-01 01:00:00', 16.0, 0.0127, 'USD', 0.2032, 0, NULL, '2025-05'),
(3, 3, 3, 4, 4, 2, '2025-05-01 00:00:00', '2025-05-01 01:00:00', 100.0, 0.095, 'USD', 9.5000, 0, NULL, '2025-05'),
(4, 3, 3, 4, 4, 2, '2025-05-01 00:00:00', '2025-05-01 01:00:00', 7.4, 0.087, 'USD', 0.6438, 0, NULL, '2025-05'),

-- App 1 with special discount
(1, 1, 1, 1, 1, 1, '2025-05-02 00:00:00', '2025-05-02 01:00:00', 8.0, 0.0456, 'USD', 0.3283, 10, 'Volume discount', '2025-05'),
(2, 1, 1, 1, 1, 1, '2025-05-02 00:00:00', '2025-05-02 01:00:00', 16.0, 0.0125, 'USD', 0.1800, 10, 'Volume discount', '2025-05'),

-- Reserved instance example
(1, 1, 1, 5, 5, 1, '2025-05-01 00:00:00', '2025-05-01 01:00:00', 8.0, 0.0290, 'USD', 0.2320, 0, 'Reserved instance', '2025-05'),
(2, 1, 1, 5, 5, 1, '2025-05-01 00:00:00', '2025-05-01 01:00:00', 16.0, 0.0125, 'USD', 0.2000, 0, NULL, '2025-05'),

-- Spot instance example
(1, 1, 1, 6, 6, 2, '2025-05-01 00:00:00', '2025-05-01 01:00:00', 8.0, 0.0137, 'USD', 0.1096, 0, 'Spot instance', '2025-05'),
(2, 1, 1, 6, 6, 2, '2025-05-01 00:00:00', '2025-05-01 01:00:00', 16.0, 0.0125, 'USD', 0.2000, 0, NULL, '2025-05');

-- Sample cost projections
INSERT INTO cost_projections (org_id, app_id, projection_period, start_date, end_date, projected_cost, currency, 
                           projection_model, confidence_level, metadata) VALUES
-- Organization 1 projections
(1, NULL, 'monthly', '2025-05-01', '2025-05-31', 15420.50, 'USD', 'linear', 0.95, 
 '{"basis": "last_30_days", "growth_factor": 1.05}'),
(1, 1, 'monthly', '2025-05-01', '2025-05-31', 5280.75, 'USD', 'linear', 0.95, 
 '{"basis": "last_30_days", "growth_factor": 1.02}'),
(1, 2, 'monthly', '2025-05-01', '2025-05-31', 2640.40, 'USD', 'linear', 0.95, 
 '{"basis": "last_30_days", "growth_factor": 1.03}'),

-- Organization 2 projections
(2, NULL, 'monthly', '2025-05-01', '2025-05-31', 28750.25, 'USD', 'linear', 0.95, 
 '{"basis": "last_30_days", "growth_factor": 1.08}'),
(2, 3, 'monthly', '2025-05-01', '2025-05-31', 18250.30, 'USD', 'linear', 0.95, 
 '{"basis": "last_30_days", "growth_factor": 1.07}'),
(2, 4, 'monthly', '2025-05-01', '2025-05-31', 10500.60, 'USD', 'linear', 0.95, 
 '{"basis": "last_30_days", "growth_factor": 1.04}'),

-- Quarterly projections
(1, NULL, 'quarterly', '2025-05-01', '2025-07-31', 48250.75, 'USD', 'linear', 0.90, 
 '{"basis": "last_90_days", "growth_factor": 1.10}'),
(2, NULL, 'quarterly', '2025-05-01', '2025-07-31', 89250.50, 'USD', 'linear', 0.90, 
 '{"basis": "last_90_days", "growth_factor": 1.15}');

-- Sample budget thresholds
INSERT INTO cost_budgets (org_id, app_id, budget_name, budget_amount, currency, budget_period, 
                        period_start, period_end, alert_threshold_percentage, alert_contacts, 
                        is_active, created_by) VALUES
-- Organization 1 budgets
(1, NULL, 'Org 1 Monthly Budget', 20000.00, 'USD', 'monthly', '2025-05-01', '2025-05-31', 80.00, 
 '["admin@org1.com", "finance@org1.com"]', TRUE, 1),
(1, 1, 'App 1 Monthly Budget', 6000.00, 'USD', 'monthly', '2025-05-01', '2025-05-31', 75.00, 
 '["app1-team@org1.com", "finance@org1.com"]', TRUE, 1),
(1, 2, 'App 2 Monthly Budget', 3000.00, 'USD', 'monthly', '2025-05-01', '2025-05-31', 75.00, 
 '["app2-team@org1.com", "finance@org1.com"]', TRUE, 1),

-- Organization 2 budgets
(2, NULL, 'Org 2 Monthly Budget', 30000.00, 'USD', 'monthly', '2025-05-01', '2025-05-31', 80.00, 
 '["admin@org2.com", "finance@org2.com"]', TRUE, 2),
(2, 3, 'App 3 Monthly Budget', 20000.00, 'USD', 'monthly', '2025-05-01', '2025-05-31', 75.00, 
 '["app3-team@org2.com", "finance@org2.com"]', TRUE, 2),
(2, 4, 'App 4 Monthly Budget', 12000.00, 'USD', 'monthly', '2025-05-01', '2025-05-31', 75.00, 
 '["app4-team@org2.com", "finance@org2.com"]', TRUE, 2),

-- Quarterly budgets
(1, NULL, 'Org 1 Q2 Budget', 55000.00, 'USD', 'quarterly', '2025-04-01', '2025-06-30', 80.00, 
 '["admin@org1.com", "finance@org1.com", "cfo@org1.com"]', TRUE, 1),
(2, NULL, 'Org 2 Q2 Budget', 95000.00, 'USD', 'quarterly', '2025-04-01', '2025-06-30', 80.00, 
 '["admin@org2.com", "finance@org2.com", "cfo@org2.com"]', TRUE, 2);

-- Sample cost allocation tags
INSERT INTO cost_allocation_tags (tag_key, tag_value, resource_id, resource_type) VALUES
-- Project tags
('project', 'e-commerce', 1, 'app'),
('project', 'e-commerce', 2, 'app'),
('project', 'analytics', 3, 'app'),
('project', 'crm', 4, 'app'),

-- Department tags
('department', 'engineering', 1, 'app'),
('department', 'engineering', 2, 'app'),
('department', 'data-science', 3, 'app'),
('department', 'sales', 4, 'app'),

-- Environment tags
('environment', 'production', 1, 'app'),
('environment', 'staging', 2, 'app'),
('environment', 'production', 3, 'app'),
('environment', 'production', 4, 'app'),

-- Team tags
('team', 'backend', 1, 'app'),
('team', 'frontend', 2, 'app'),
('team', 'data', 3, 'app'),
('team', 'integrations', 4, 'app'),

-- Cost center tags
('cost-center', 'cc-1001', 1, 'app'),
('cost-center', 'cc-1001', 2, 'app'),
('cost-center', 'cc-2001', 3, 'app'),
('cost-center', 'cc-3001', 4, 'app');

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