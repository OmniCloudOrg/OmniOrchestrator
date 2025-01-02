BEGIN TRANSACTION;

-- Input Parameters (example values provided)
-- Replace these placeholders with your actual data
SET @permission_id = 1;  -- The ID of the permission to be removed

-- Check if the permission exists
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

-- Check if the permission is associated with any roles or users via permissions_role table
IF EXISTS (
    SELECT 1
    FROM permissions_role
    WHERE permissions_id = @permission_id
)
THEN
    ROLLBACK;
    SELECT 'Error: Cannot delete permission as it is associated with roles.' AS error_message;
    LEAVE;
END IF;

-- Remove the permission from the permissions table
DELETE FROM permissions
WHERE id = @permission_id;

-- Commit the transaction
COMMIT;

-- Output a success message
SELECT 'Permission removed successfully.' AS status_message;