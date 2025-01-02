BEGIN TRANSACTION;

-- Input Parameters (example values provided)
-- Replace these placeholders with your actual data
SET @role_id = 1;  -- The ID of the role to be updated
SET @new_role_name = 'SuperAdmin';  -- The new name for the role
SET @new_role_description = 'Super administrator role with all permissions';  -- The new description for the role

-- Check if the role with the new name already exists
IF EXISTS (
    SELECT 1
    FROM roles
    WHERE name = @new_role_name
    AND id != @role_id  -- Ensure it is not the same role being updated
)
THEN
    ROLLBACK;
    SELECT 'Error: A role with the new name already exists.' AS error_message;
    LEAVE;
END IF;

-- Update the role's name and description
UPDATE roles
SET name = @new_role_name,
    description = @new_role_description,
    updated_at = CURRENT_TIMESTAMP
WHERE id = @role_id;

-- Commit the transaction
COMMIT;

-- Output a success message
SELECT 'Role updated successfully.' AS status_message;