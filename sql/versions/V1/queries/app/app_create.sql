BEGIN TRANSACTION;

-- Input Parameters (example values provided)
-- Replace these placeholders with your actual data
SET @app_name = 'MyApp';
SET @org_id = 1;  -- The ID of the organization to which the app belongs
SET @region_id = 1; -- The ID of the region where the app will be deployed
SET @git_repo = 'https://github.com/example/repo.git';
SET @git_branch = 'main';
SET @buildpack_url = 'https://buildpack.example.com';

-- Insert the new app into the apps table
INSERT INTO apps (name, org_id, git_repo, git_branch, buildpack_url, region_id, maintenance_mode)
VALUES (@app_name, @org_id, @git_repo, @git_branch, @buildpack_url, @region_id, 0);

-- Capture the ID of the newly created app
SET @new_app_id = last_insert_rowid();

-- Optional: Log the creation action (for audit purposes)
INSERT INTO audit_logs (user_id, org_id, action, resource_type, resource_id, created_at)
VALUES (1, -- Replace with actual user_id
        @org_id,
        'create',
        'app',
        @new_app_id,
        CURRENT_TIMESTAMP);

-- Commit the transaction
COMMIT;

-- Output the ID of the newly created app
SELECT @new_app_id AS new_app_id;