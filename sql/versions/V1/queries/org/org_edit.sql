BEGIN TRANSACTION;

-- Input Parameters (example values provided)
-- Replace these placeholders with your actual data
SET @org_id = 1;  -- The ID of the organization to be edited
SET @new_org_name = 'UpdatedOrganizationName';  -- The new name of the organization

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

-- Check if the new organization name already exists for another organization
IF EXISTS (
    SELECT 1
    FROM orgs
    WHERE name = @new_org_name AND id != @org_id
)
THEN
    ROLLBACK;
    SELECT 'Error: Organization name already exists for another organization.' AS error_message;
    LEAVE;
END IF;

-- Update the organization with the new name
UPDATE orgs
SET name = @new_org_name, updated_at = CURRENT_TIMESTAMP
WHERE id = @org_id;

-- Commit the transaction
COMMIT;

-- Output a success message
SELECT 'Organization updated successfully.' AS status_message;