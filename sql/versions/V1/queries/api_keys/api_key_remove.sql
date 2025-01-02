BEGIN TRANSACTION;

-- Input Parameters (example values provided)
-- Replace these placeholders with your actual data
SET @api_key_id = 1;  -- The ID of the API key to remove

-- Delete the API key
DELETE FROM api_keys
WHERE id = @api_key_id;

-- Commit the transaction
COMMIT;