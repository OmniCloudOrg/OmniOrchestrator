BEGIN TRANSACTION;

-- Example Input Parameters
-- Replace the placeholders with actual data
SET @app_id = 1;               -- The ID of the app to edit
SET @new_app_name = 'NewAppName'; -- The updated app name
SET @new_git_repo = 'https://github.com/example/new-repo.git'; -- Updated git repository URL
SET @new_git_branch = 'develop';  -- Updated git branch
SET @new_buildpack_url = 'https://buildpack.example.com'; -- Updated buildpack URL
SET @new_region_id = 2;        -- Updated region ID
SET @new_maintenance_mode = 1; -- Updated maintenance mode (0 for off, 1 for on)

-- Update the app
UPDATE apps
SET 
    name = @new_app_name,
    git_repo = @new_git_repo,
    git_branch = @new_git_branch,
    buildpack_url = @new_buildpack_url,
    region_id = @new_region_id,
    maintenance_mode = @new_maintenance_mode,
    updated_at = CURRENT_TIMESTAMP
WHERE id = @app_id;

-- Example: Update associated instances (optional)
-- Ensures consistency across related data
UPDATE instances
SET 
    status = 'stopping',
    updated_at = CURRENT_TIMESTAMP
WHERE app_id = @app_id;

-- Example: Log the update for audit purposes
INSERT INTO audit_logs (user_id, org_id, action, resource_type, resource_id, created_at)
VALUES 
    (1, -- Replace with actual user_id
     (SELECT org_id FROM apps WHERE id = @app_id), -- Fetch the associated org_id
     'update',
     'app',
     @app_id,
     CURRENT_TIMESTAMP);

-- Commit the transaction
COMMIT;

-- Output the updated app details
SELECT * FROM apps WHERE id = @app_id;