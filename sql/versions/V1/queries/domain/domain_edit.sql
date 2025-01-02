BEGIN TRANSACTION;

-- Input Parameters (example values provided)
-- Replace these placeholders with your actual data
SET @domain_id = 1;           -- The ID of the domain to edit
SET @new_domain_name = 'new-example.com'; -- The new domain name
SET @new_ssl_enabled = 0;     -- New SSL enabled status (1 for true, 0 for false)

-- Check if the domain exists
IF NOT EXISTS (
    SELECT 1
    FROM domains
    WHERE id = @domain_id
)
THEN
    ROLLBACK;
    SELECT 'Error: Domain not found.' AS error_message;
    LEAVE;
END IF;

-- Check if the new domain name is already used by another domain for the same app
IF EXISTS (
    SELECT 1
    FROM domains
    WHERE name = @new_domain_name AND id != @domain_id
)
THEN
    ROLLBACK;
    SELECT 'Error: Domain name already in use.' AS error_message;
    LEAVE;
END IF;

-- Update the domain
UPDATE domains
SET name = @new_domain_name,
    ssl_enabled = @new_ssl_enabled,
    updated_at = CURRENT_TIMESTAMP
WHERE id = @domain_id;

-- Commit the transaction
COMMIT;

-- Output a success message
SELECT @domain_id AS updated_domain_id, 'Domain updated successfully.' AS status_message;