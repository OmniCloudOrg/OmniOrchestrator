DELETE FROM deployment_logs
WHERE id = :log_id
AND EXISTS (SELECT 1 FROM deployment_logs WHERE id = :log_id);