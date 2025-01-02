BEGIN TRANSACTION;

-- Input Parameters (example values provided)
-- Replace these placeholders with your actual data
SET @log_id = 1;  -- The ID of the log to be removed

-- Check if the log entry exists
IF NOT EXISTS (
    SELECT 1
    FROM deployment_logs
    WHERE id = @log_id
)
THEN
    ROLLBACK;
    SELECT 'Error: Log entry not found.' AS error_message;
    LEAVE;
END IF;

-- Delete the log entry
DELETE FROM deployment_logs
WHERE id = @log_id;

-- Commit the transaction
COMMIT;

-- Output a success message
SELECT 'Log entry removed successfully.' AS status_message;