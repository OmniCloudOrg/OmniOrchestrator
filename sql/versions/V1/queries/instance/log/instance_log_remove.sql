DELETE FROM instance_logs
WHERE id = :log_id
AND EXISTS (SELECT 1 FROM instance_logs WHERE id = :log_id);