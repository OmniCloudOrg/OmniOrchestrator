INSERT INTO apps (
    name,
    org_id,
    git_repo,
    git_branch,
    container_image_url,
    region_id,
    maintenance_mode
)
VALUES (
    ?1,
    ?2,
    ?3,
    ?4,
    ?5,
    ?6,
    0
)
RETURNING last_insert_rowid() as new_app_id;