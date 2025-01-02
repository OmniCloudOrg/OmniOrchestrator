BEGIN TRANSACTION;

-- Input Parameters (example values provided)
-- Replace these placeholders with your actual data
SET @user_id = 1;  -- The ID of the user to be updated
SET @new_email = 'new_email@example.com';  -- The new email to be updated
SET @new_name = 'New Name';  -- The new name to be updated
SET @new_password = 'new_secure_password';  -- The new password to be updated
SET @new_active = 1;  -- Set to 1 if the user should be active, 0 for inactive

-- Check if the user exists
IF NOT EXISTS (
    SELECT 1
    FROM users
    WHERE id = @user_id
)
THEN
    ROLLBACK;
    SELECT 'Error: User not found.' AS error_message;
    LEAVE;
END IF;

-- Update user information
UPDATE users
SET
    email = @new_email,
    name = @new_name,
    password = @new_password,
    active = @new_active,
    updated_at = CURRENT_TIMESTAMP
WHERE id = @user_id;

-- Commit the transaction
COMMIT;

-- Output a success message
SELECT 'User updated successfully.' AS status_message;