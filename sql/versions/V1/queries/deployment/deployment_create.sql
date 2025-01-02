BEGIN TRANSACTION;

-- Input Parameters (replace with actual values)
SET @app_id = 1; -- The ID of the app for which the deployment is created
SET @build_id = 10; -- The ID of the associated build
SET @status = 'pending'; -- Initial deployment status (e.g., 'pending')
SET @started_at = CURRENT_TIMESTAMP; -- Deployment start timestamp (if already started)

-- Ensure the build belongs to the app
IF NOT EXISTS (
    SELECT 1
    FROM builds
    WHERE id = @build_id AND app_id = @app_id
)
THEN
    ROLLBACK;
    SELECT 'Error: Build does not belong to the specified app.' AS error_message;
    LEAVE;
END IF;

-- Insert the new deployment
INSERT INTO deployments (app_id, build_id, status, started_at, created_at)
VALUES (@app_id, @build_id, @status, @started_at, CURRENT_TIMESTAMP);

-- Output the ID of the newly created deployment
SELECT last_insert_rowid() AS new_deployment_id;

COMMIT;