BEGIN TRANSACTION;

-- Input Parameters (example values provided)
-- Replace these placeholders with your actual data
SET @app_id = 1;            -- ID of the app to associate the domain with
SET @domain_name = 'example.com'; -- The domain name to add
SET @ssl_enabled = 1;       -- Whether SSL is enabled for the domain (1 for true, 0 for false)

-- Check if the app exists
IF NOT EXISTS (
    SELECT 1
    FROM apps
    WHERE id = @app_id
)
THEN
    ROLLBACK;
    SELECT 'Error: App not found.' AS error_message;
    LEAVE;
END IF;

-- Check if the domain already exists for the app
IF EXISTS (
    SELECT 1
    FROM domains
    WHERE app_id = @app_id AND name = @domain_name
)
THEN
    ROLLBACK;
    SELECT 'Error: Domain already exists for the app.' AS error_message;
    LEAVE;
END IF;

-- Add the domain
INSERT INTO domains (app_id, name, ssl_enabled)
VALUES (@app_id, @domain_name, @ssl_enabled);

-- Commit the transaction
COMMIT;

-- Output the ID of the newly added domain
SELECT last_insert_rowid() AS new_domain_id, 'Domain added successfully.' AS status_message;