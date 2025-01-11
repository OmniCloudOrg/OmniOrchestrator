INSERT INTO cluster.apps (
    name,
    org_id,
    git_repo,
    git_branch,
    container_image_url,
    region_id,
    maintenance_mode
)
VALUES (
    :name,
    :org_id,
    :git_repo,
    :git_branch,
    :container_image_url,
    :region_id,
    0
)
RETURNING id;