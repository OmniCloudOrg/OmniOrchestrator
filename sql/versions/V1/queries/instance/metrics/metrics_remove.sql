BEGIN TRANSACTION;

-- Input Parameters (example values provided)
-- Replace this placeholder with your actual data
SET @metric_id = 1;  -- The ID of the metric to be removed

-- Delete the metric from the 'metrics' table
DELETE FROM metrics
WHERE id = @metric_id;

-- Check if the metric was removed successfully
SELECT ROW_COUNT() AS rows_affected;

-- Commit the transaction
COMMIT;