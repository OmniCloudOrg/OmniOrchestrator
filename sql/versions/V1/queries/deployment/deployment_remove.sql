WITH deleted_deployment AS (
    DELETE FROM deployments
    WHERE id = :deployment_id
    AND EXISTS (SELECT 1 FROM deployments WHERE id = :deployment_id)
    RETURNING app_id
)
DELETE FROM instance_logs
WHERE log_type = 'deployment'
AND EXISTS (
    SELECT 1
    FROM instances i, deleted_deployment d
    WHERE i.id = instance_logs.instance_id
    AND i.app_id = d.app_id
);