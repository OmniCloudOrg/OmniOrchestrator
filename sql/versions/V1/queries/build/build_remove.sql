BEGIN TRANSACTION;

-- Input Parameters (example values provided)
-- Replace these placeholders with your actual data
SET @build_id = 1;  -- The ID of the build to be removed

-- Check if the build exists before deleting
SELECT CASE
           WHEN EXISTS (SELECT 1 FROM builds WHERE id = @build_id) THEN 'Build Exists'
           ELSE 'Build Not Found'
       END AS build_status;

-- Delete the build record
DELETE FROM builds
WHERE id = @build_id;

-- Ensure that the build was deleted
SELECT CASE
           WHEN NOT EXISTS (SELECT 1 FROM builds WHERE id = @build_id) THEN 'Build Deleted'
           ELSE 'Build Not Deleted'
       END AS delete_status;

-- Commit the transaction
COMMIT;