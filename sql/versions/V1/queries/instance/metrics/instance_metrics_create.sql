INSERT INTO metrics (instance_id, metric_name, metric_value, timestamp)
SELECT :instance_id, :metric_name, :metric_value, :timestamp
WHERE EXISTS (SELECT 1 FROM instances WHERE id = :instance_id);