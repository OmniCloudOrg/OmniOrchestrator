BEGIN TRANSACTION;

-- Input Parameters (example values provided)
-- Replace these placeholders with your actual data
SET @config_var_id = 1;  -- The ID of the configuration variable to be removed

-- Remove the configuration variable
DELETE FROM config_vars
WHERE id = @config_var_id;

-- Check if the deletion was successful
SELECT changes() AS rows_affected;

-- Commit the transaction
COMMIT;