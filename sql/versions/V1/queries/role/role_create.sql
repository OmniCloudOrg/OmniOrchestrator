BEGIN TRANSACTION;

-- Input Parameters (example values provided)
-- Replace these placeholders with your actual data
SET @role_name = 'Admin';  -- The name of the role to be created
SET @role_description = 'Administrator role with full access';  -- The description of the role

-- Check if the role already exists
IF EXISTS (
    SELECT 1
    FROM roles
    WHERE name = @role_name
)
THEN
    ROLLBACK;
    SELECT 'Error: Role already exists.' AS error_message;
    LEAVE;
END IF;

-- Insert the new role into the roles table
INSERT INTO roles (name, description)
VALUES (@role_name, @role_description);

-- Commit the transaction
COMMIT;

-- Output a success message
SELECT 'Role created successfully.' AS status_message;