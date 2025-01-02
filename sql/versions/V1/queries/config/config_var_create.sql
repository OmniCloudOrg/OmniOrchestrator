BEGIN TRANSACTION;

-- Input Parameters (example values provided)
-- Replace these placeholders with your actual data
SET @app_id = 1;      -- The ID of the app to which the config variable belongs
SET @key = 'API_KEY';  -- The key of the configuration variable
SET @value = '12345';  -- The value of the configuration variable
SET @is_secret = 1;    -- Flag indicating whether the variable is secret (1 = secret, 0 = not secret)

-- Insert the new configuration variable into the config_vars table
INSERT INTO config_vars (app_id, key, value, is_secret)
VALUES (@app_id, @key, @value, @is_secret);

-- Output the ID of the newly created configuration variable
SELECT last_insert_rowid() AS new_config_var_id;

-- Commit the transaction
COMMIT;