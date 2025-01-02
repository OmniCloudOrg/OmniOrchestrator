BEGIN TRANSACTION;

-- Input Parameters (example values provided)
-- Replace these placeholders with your actual data
SET @user_email = 'user@example.com';  -- The email of the new user
SET @user_name = 'John Doe';  -- The name of the new user
SET @user_password = 'securepassword';  -- The password of the new user
SET @user_active = 1;  -- The status of the user (1 = active, 0 = inactive)

-- Check if the email already exists
IF EXISTS (
    SELECT 1
    FROM users
    WHERE email = @user_email
)
THEN
    ROLLBACK;
    SELECT 'Error: Email already exists.' AS error_message;
    LEAVE;
END IF;

-- Insert the new user
INSERT INTO users (email, name, password, active)
VALUES (@user_email, @user_name, @user_password, @user_active);

-- Output the ID of the newly created user
SELECT last_insert_rowid() AS new_user_id;

-- Commit the transaction
COMMIT;

-- Output a success message
SELECT 'User created successfully.' AS status_message;