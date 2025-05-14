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
DROP TABLE IF EXISTS users, user_meta, user_pii, user_sessions;

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

-- User metadata - preferences and non-sensitive settings
CREATE TABLE user_meta (
    id BIGINT NOT NULL AUTO_INCREMENT,
    user_id BIGINT NOT NULL,
    timezone VARCHAR(50) DEFAULT 'UTC',
    language VARCHAR(10) DEFAULT 'en',
    theme VARCHAR(50) DEFAULT 'light',
    notification_preferences JSON,
    profile_image VARCHAR(255),
    dashboard_layout JSON,
    onboarding_completed TINYINT(1) DEFAULT 0,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    PRIMARY KEY (id),
    UNIQUE KEY unique_user_id (user_id),
    INDEX idx_user_meta_user_id (user_id),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- Personally Identifiable Information (PII) - sensitive data
CREATE TABLE user_pii (
    id BIGINT NOT NULL AUTO_INCREMENT,
    user_id BIGINT NOT NULL,
    first_name VARCHAR(100),
    last_name VARCHAR(100),
    full_name VARCHAR(255),
    identity_verified TINYINT(1) DEFAULT 0,
    identity_verification_date DATETIME,
    identity_verification_method VARCHAR(100),
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    PRIMARY KEY (id),
    UNIQUE KEY unique_user_id (user_id),
    INDEX idx_user_pii_user_id (user_id),
    INDEX idx_user_pii_identity_verified (identity_verified),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- User sessions table for better session management
CREATE TABLE user_sessions (
    id BIGINT NOT NULL AUTO_INCREMENT,
    user_id BIGINT NOT NULL,
    session_token VARCHAR(255) NOT NULL,
    refresh_token VARCHAR(255),
    ip_address VARCHAR(45),
    user_agent TEXT,
    device_info JSON,
    location_info JSON,
    is_active TINYINT(1) DEFAULT 1,
    last_activity DATETIME,
    expires_at DATETIME NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (id),
    UNIQUE KEY unique_session_token (session_token),
    INDEX idx_user_sessions_user_id (user_id),
    INDEX idx_user_sessions_is_active (is_active),
    INDEX idx_user_sessions_expires_at (expires_at),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
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