BEGIN TRANSACTION;

-- Input Parameters (example values provided)
-- Replace these placeholders with your actual data
SET @role_id = 1;  -- The ID of the role to be removed

-- Check if the role exists
IF NOT EXISTS (
    SELECT 1
    FROM roles
    WHERE id = @role_id
)
THEN
    ROLLBACK;
    SELECT 'Error: Role does not exist.' AS error_message;
    LEAVE;
END IF;

-- Check if any users are assigned to this role
IF EXISTS (
    SELECT 1
    FROM role_user
    WHERE role_id = @role_id
)
THEN
    ROLLBACK;
    SELECT 'Error: Role cannot be removed because it is assigned to one or more users.' AS error_message;
    LEAVE;
END IF;

-- Remove any associated permissions with the role
DELETE FROM permissions_role
WHERE role_id = @role_id;

-- Remove the role
DELETE FROM roles
WHERE id = @role_id;

-- Commit the transaction
COMMIT;

-- Output a success message
SELECT 'Role removed successfully.' AS status_message;