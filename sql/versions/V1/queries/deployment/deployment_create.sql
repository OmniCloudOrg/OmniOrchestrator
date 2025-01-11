INSERT INTO deployments (app_id, build_id, status, started_at, created_at)
SELECT :app_id, :build_id, :status, :started_at, CURRENT_TIMESTAMP
WHERE EXISTS (
    SELECT 1 
    FROM builds 
    WHERE id = :build_id 
    AND app_id = :app_id
);