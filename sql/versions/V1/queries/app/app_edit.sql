UPDATE cluster.apps
SET 
    name = :name,
    git_repo = :git_repo,
    git_branch = :git_branch,
    container_image_url = :container_image_url,
    region_id = :region_id,
    maintenance_mode = :maintenance_mode,
    updated_at = CURRENT_TIMESTAMP
WHERE id = :id;