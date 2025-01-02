BEGIN TRANSACTION;

-- Input Parameters (example values provided)
-- Replace these placeholders with your actual data
SET @api_key_id = 1;  -- The ID of the API key to edit
SET @new_name = 'UpdatedAPIKey';  -- The new name for the API key
SET @new_key_hash = 'new-hashed-api-key-value';  -- The new hashed value for the API key

-- Update the API key details
UPDATE api_keys
SET name = @new_name,
    key_hash = @new_key_hash,
    created_at = CURRENT_TIMESTAMP  -- Optionally update the timestamp if needed
WHERE id = @api_key_id;

-- Commit the transaction
COMMIT;