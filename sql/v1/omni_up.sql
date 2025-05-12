-------------------------------------------------------------------------------
-- OmniCloud deployment database schema
-------------------------------------------------------------------------------
-- This script creates the database schema for the V1 omni deployment database.
-- It references all the virtual platforms that exist in the deployment via the
-- platforms table. The platforms table is used to store the metadata for each
-- platform.
-------------------------------------------------------------------------------

DROP TABLE IF EXISTS omni_users;
DROP TABLE IF EXISTS platforms;
DROP TABLE IF EXISTS users;

CREATE TABLE users (
    id BIGINT NOT NULL AUTO_INCREMENT,
    email VARCHAR(255) NOT NULL,
    email_verified TINYINT(1) DEFAULT 0,
    password VARCHAR(255) NOT NULL,
    salt VARCHAR(255) NOT NULL,
    password_changed_at DATETIME,
    login_attempts BIGINT DEFAULT 0,
    locked_until DATETIME,
    two_factor_enabled TINYINT(1) DEFAULT 0,
    two_factor_verified TINYINT(1) DEFAULT 0,
    active TINYINT(1) DEFAULT 1,
    status ENUM('active', 'deactivated', 'suspended', 'pending') DEFAULT 'pending',
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    deleted_at DATETIME,
    last_login_at DATETIME,
    PRIMARY KEY (id),
    UNIQUE KEY unique_email (email),
    INDEX idx_users_email_verified (email_verified),
    INDEX idx_users_active (active),
    INDEX idx_users_deleted_at (deleted_at),
    INDEX idx_users_status (status)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE platforms (
    id BIGINT NOT NULL AUTO_INCREMENT,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    table_name VARCHAR(255),
    subdomain VARCHAR(255),
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    deleted_at DATETIME,
    PRIMARY KEY (id),
    UNIQUE KEY unique_platform_name (name),
    INDEX idx_platforms_deleted_at (deleted_at)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;