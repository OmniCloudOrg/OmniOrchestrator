BEGIN TRANSACTION;

-- Input Parameters (example values provided)
-- Replace these placeholders with your actual data
SET @instance_id = 1;  -- The ID of the instance to be removed

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

-- Check if there are associated logs for the instance
IF EXISTS (
    SELECT 1
    FROM instance_logs
    WHERE instance_id = @instance_id
)
THEN
    -- Optionally remove the associated logs first (if desired)
    DELETE FROM instance_logs
    WHERE instance_id = @instance_id;
END IF;

-- Delete the instance
DELETE FROM instances
WHERE id = @instance_id;

-- Commit the transaction
COMMIT;

-- Output a success message
SELECT 'Instance removed successfully.' AS status_message;