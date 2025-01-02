BEGIN TRANSACTION;

-- Input Parameters (example values provided)
-- Replace these placeholders with your actual data
SET @build_id = 1;  -- The ID of the build to be updated
SET @status = 'building';  -- New status of the build
SET @completed_at = NULL;  -- Timestamp when the build completes (can be updated if needed)
SET @source_version = 'v1.0.1';  -- Updated version of the code to be built
SET @started_at = CURRENT_TIMESTAMP;  -- Timestamp when the build started (this could be updated as well)

-- Update the build record
UPDATE builds
SET source_version = @source_version,
    status = @status,
    started_at = @started_at,
    completed_at = @completed_at
WHERE id = @build_id;

-- Ensure that the build ID exists
SELECT CASE
           WHEN EXISTS (SELECT 1 FROM builds WHERE id = @build_id) THEN 'Build Updated'
           ELSE 'Build Not Found'
       END AS update_status;

-- Commit the transaction
COMMIT;