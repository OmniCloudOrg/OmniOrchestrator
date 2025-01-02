BEGIN TRANSACTION;

-- Input Parameters (example values provided)
-- Replace these placeholders with your actual data
SET @instance_id = 1;          -- The ID of the instance for which the log is being created
SET @log_type = 'app';         -- Type of log: 'app', 'system', or 'deployment'
SET @message = 'Instance started successfully'; -- The log message

-- Check if the instance exists
IF NOT EXISTS (
    SELECT 1
    FROM instances
    WHERE id = @instance_id
)
THEN
    ROLLBACK;
    SELECT 'Error: Instance not found.' AS error_message;
    LEAVE;
END IF;

-- Insert the new log into the instance_logs table
INSERT INTO deployment_logs (instance_id, log_type, message, timestamp)
VALUES (@instance_id, @log_type, @message, CURRENT_TIMESTAMP);

-- Output the ID of the newly created log entry
SELECT last_insert_rowid() AS new_log_id;

-- Commit the transaction
COMMIT;

-- Output a success message
SELECT 'Log entry created successfully.' AS status_message;