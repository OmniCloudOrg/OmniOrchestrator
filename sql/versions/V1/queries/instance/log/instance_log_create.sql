INSERT INTO instance_logs (instance_id, log_type, message, timestamp)
SELECT :instance_id, :log_type, :message, CURRENT_TIMESTAMP
WHERE EXISTS (
    SELECT 1 
    FROM instances 
    WHERE id = :instance_id
);