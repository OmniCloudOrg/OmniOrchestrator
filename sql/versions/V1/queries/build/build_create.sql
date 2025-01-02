BEGIN TRANSACTION;

-- Input Parameters (example values provided)
-- Replace these placeholders with your actual data
SET @app_id = 1;  -- The ID of the app for which the build is being created
SET @source_version = 'v1.0.0';  -- The version of the code to be built
SET @status = 'pending';  -- Initial status of the build
SET @started_at = CURRENT_TIMESTAMP;  -- Timestamp when the build started (can be set to current timestamp)
SET @completed_at = NULL;  -- Timestamp when the build completes (will be NULL initially)

-- Insert a new build into the builds table
INSERT INTO builds (app_id, source_version, status, started_at, completed_at)
VALUES (@app_id, @source_version, @status, @started_at, @completed_at);

-- Output the ID of the newly created build
SELECT last_insert_rowid() AS new_build_id;

-- Commit the transaction
COMMIT;