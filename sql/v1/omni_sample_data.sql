-- Sample Data Generation Script for OmniCloud
-- This script uses only direct inserts (no temp tables, no stored procedures)
-- Compatible with all MySQL environments

-- Disable foreign key checks temporarily for faster inserts
SET FOREIGN_KEY_CHECKS = 0;
SET UNIQUE_CHECKS = 0;
SET SQL_MODE = '';

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