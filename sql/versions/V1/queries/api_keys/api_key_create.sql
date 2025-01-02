BEGIN TRANSACTION;

-- Input Parameters (example values provided)
-- Replace these placeholders with your actual data
SET @org_id = 1;  -- The ID of the organization to which the API key will be associated
SET @name = 'MyAPIKey';  -- The name of the API key
SET @key_hash = 'hashed-api-key-value';  -- The hashed value of the API key

-- Insert the new API key
INSERT INTO api_keys (org_id, name, key_hash, created_at)
VALUES (@org_id, @name, @key_hash, CURRENT_TIMESTAMP);

-- Output the ID of the newly created API key
SELECT last_insert_rowid() AS new_api_key_id;

-- Commit the transaction
COMMIT;