BEGIN TRANSACTION;

-- Input Parameters (example values provided)
-- Replace these placeholders with your actual data
SET @org_name = 'MyOrganization';  -- The name of the new organization

-- Check if the organization name already exists
IF EXISTS (
    SELECT 1
    FROM orgs
    WHERE name = @org_name
)
THEN
    ROLLBACK;
    SELECT 'Error: Organization name already exists.' AS error_message;
    LEAVE;
END IF;

-- Insert the new organization
INSERT INTO orgs (name)
VALUES (@org_name);

-- Output the ID of the newly created organization
SELECT last_insert_rowid() AS new_org_id;

-- Commit the transaction
COMMIT;

-- Output a success message
SELECT 'Organization created successfully.' AS status_message;