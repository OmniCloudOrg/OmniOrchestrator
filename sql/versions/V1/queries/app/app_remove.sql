BEGIN TRANSACTION;

-- Input Parameter (example value provided)
-- Replace this placeholder with your actual data
SET @app_id = 1;  -- The ID of the app to be removed

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

-- Delete associated data from dependent tables
-- Delete config vars associated with the app
DELETE FROM config_vars
WHERE app_id = @app_id;

-- Delete domains associated with the app
DELETE FROM domains
WHERE app_id = @app_id;

-- Delete instances associated with the app
DELETE FROM instances
WHERE app_id = @app_id;

-- Delete deployments associated with the app
DELETE FROM deployments
WHERE app_id = @app_id;

-- Delete builds associated with the app
DELETE FROM builds
WHERE app_id = @app_id;

-- Finally, delete the app itself
DELETE FROM apps
WHERE id = @app_id;

-- Commit the transaction
COMMIT;

-- Output a success message
SELECT 'App removed successfully.' AS status_message;