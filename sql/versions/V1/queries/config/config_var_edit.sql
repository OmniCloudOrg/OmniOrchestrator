BEGIN TRANSACTION;

-- Input Parameters (example values provided)
-- Replace these placeholders with your actual data
SET @config_var_id = 1;   -- The ID of the configuration variable to be updated
SET @new_value = '67890';  -- The new value for the configuration variable
SET @new_is_secret = 0;    -- New secret flag (1 = secret, 0 = not secret)

-- Update the configuration variable with the new value and secret flag
UPDATE config_vars
SET value = @new_value,
    is_secret = @new_is_secret,
    updated_at = CURRENT_TIMESTAMP  -- Ensure updated_at is set to the current timestamp
WHERE id = @config_var_id;

-- Check if the update was successful
SELECT changes() AS rows_affected;

-- Commit the transaction
COMMIT;