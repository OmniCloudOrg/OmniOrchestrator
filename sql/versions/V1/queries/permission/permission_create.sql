BEGIN TRANSACTION;

-- Input Parameters (example values provided)
-- Replace these placeholders with your actual data
SET @permission_name = 'access_dashboard';  -- The name of the permission
SET @permission_description = 'Permission to access the dashboard';
SET @resource_type = 'dashboard';  -- The resource type this permission applies to

-- Check if the permission already exists
IF EXISTS (
    SELECT 1
    FROM permissions
    WHERE name = @permission_name
)
THEN
    ROLLBACK;
    SELECT 'Error: Permission already exists.' AS error_message;
    LEAVE;
END IF;

-- Insert the new permission
INSERT INTO permissions (name, description, resource_type)
VALUES (@permission_name, @permission_description, @resource_type);

-- Output the ID of the newly created permission
SELECT last_insert_rowid() AS new_permission_id;

-- Commit the transaction
COMMIT;

-- Output a success message
SELECT 'Permission created successfully.' AS status_message;