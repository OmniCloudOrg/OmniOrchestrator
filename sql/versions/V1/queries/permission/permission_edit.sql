BEGIN TRANSACTION;

-- Input Parameters (example values provided)
-- Replace these placeholders with your actual data
SET @permission_id = 1;  -- The ID of the permission to be edited
SET @new_permission_name = 'edit_dashboard';  -- The new name for the permission
SET @new_permission_description = 'Permission to edit the dashboard';
SET @new_resource_type = 'dashboard';  -- The new resource type this permission applies to

-- Check if the permission ID exists
IF NOT EXISTS (
    SELECT 1
    FROM permissions
    WHERE id = @permission_id
)
THEN
    ROLLBACK;
    SELECT 'Error: Permission does not exist.' AS error_message;
    LEAVE;
END IF;

-- Check if the new permission name already exists (ensure no name duplication)
IF EXISTS (
    SELECT 1
    FROM permissions
    WHERE name = @new_permission_name AND id != @permission_id
)
THEN
    ROLLBACK;
    SELECT 'Error: Permission name already exists for another permission.' AS error_message;
    LEAVE;
END IF;

-- Update the permission details
UPDATE permissions
SET name = @new_permission_name,
    description = @new_permission_description,
    resource_type = @new_resource_type,
    updated_at = CURRENT_TIMESTAMP
WHERE id = @permission_id;

-- Output the ID of the updated permission
SELECT @permission_id AS updated_permission_id;

-- Commit the transaction
COMMIT;

-- Output a success message
SELECT 'Permission updated successfully.' AS status_message;