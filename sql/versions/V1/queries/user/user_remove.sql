BEGIN TRANSACTION;

-- Input Parameters (example value provided)
-- Replace these placeholders with your actual data
SET @user_id = 1;  -- The ID of the user to be removed

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

-- Check if the user has any associated roles or data
-- Remove associated role_user entries
DELETE FROM role_user
WHERE user_id = @user_id;

-- Remove associated orgmember entries
DELETE FROM orgmember
WHERE user_id = @user_id;

-- Remove associated audit logs
DELETE FROM audit_logs
WHERE user_id = @user_id;

-- Remove the user
DELETE FROM users
WHERE id = @user_id;

-- Commit the transaction
COMMIT;

-- Output a success message
SELECT 'User removed successfully.' AS status_message;