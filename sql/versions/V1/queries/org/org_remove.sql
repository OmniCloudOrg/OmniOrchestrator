BEGIN TRANSACTION;

-- Input Parameters (example values provided)
-- Replace these placeholders with your actual data
SET @org_id = 1;  -- The ID of the organization to be removed

-- Check if the organization exists
IF NOT EXISTS (
    SELECT 1
    FROM orgs
    WHERE id = @org_id
)
THEN
    ROLLBACK;
    SELECT 'Error: Organization not found.' AS error_message;
    LEAVE;
END IF;

-- Check if there are any dependencies (orgmembers, apps, etc.)
IF EXISTS (
    SELECT 1
    FROM orgmember
    WHERE org_id = @org_id
)
THEN
    ROLLBACK;
    SELECT 'Error: Organization cannot be removed because it has associated members.' AS error_message;
    LEAVE;
END IF;

IF EXISTS (
    SELECT 1
    FROM apps
    WHERE org_id = @org_id
)
THEN
    ROLLBACK;
    SELECT 'Error: Organization cannot be removed because it has associated apps.' AS error_message;
    LEAVE;
END IF;

-- Optionally, you can add additional checks for other dependencies like deployments, etc.

-- Delete the organization
DELETE FROM orgs
WHERE id = @org_id;

-- Commit the transaction
COMMIT;

-- Output a success message
SELECT 'Organization removed successfully.' AS status_message;