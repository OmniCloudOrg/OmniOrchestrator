BEGIN TRANSACTION;

-- Input Parameters (example values provided)
-- Replace these placeholders with your actual data
SET @app_id = 1;                 -- The ID of the app for which the instance is being created
SET @instance_type = 't2.micro';  -- The type of the instance (e.g., t2.micro)
SET @container_id = 'abc123';     -- The container ID associated with the instance
SET @pod_name = 'pod-xyz';        -- The pod name associated with the instance
SET @node_name = 'node-01';       -- The node name where the instance is running

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

-- Insert the new instance into the instances table
INSERT INTO instances (app_id, instance_type, status, container_id, pod_name, node_name, created_at, updated_at)
VALUES (@app_id, @instance_type, 'provisioning', @container_id, @pod_name, @node_name, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);

-- Output the ID of the newly created instance
SELECT last_insert_rowid() AS new_instance_id;

-- Commit the transaction
COMMIT;

-- Output a success message
SELECT 'Instance created successfully.' AS status_message;