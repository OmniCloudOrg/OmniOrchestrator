BEGIN TRANSACTION;

-- Input Parameters (example values provided)
-- Replace these placeholders with your actual data
SET @domain_id = 1;  -- The ID of the domain to remove

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

-- Check if the domain is associated with an app before removing
IF EXISTS (
    SELECT 1
    FROM instances
    WHERE app_id IN (SELECT app_id FROM domains WHERE id = @domain_id)
)
THEN
    ROLLBACK;
    SELECT 'Error: Domain is in use by an app and cannot be removed.' AS error_message;
    LEAVE;
END IF;

-- Delete the domain from the domains table
DELETE FROM domains
WHERE id = @domain_id;

-- Commit the transaction
COMMIT;

-- Output a success message
SELECT @domain_id AS removed_domain_id, 'Domain removed successfully.' AS status_message;