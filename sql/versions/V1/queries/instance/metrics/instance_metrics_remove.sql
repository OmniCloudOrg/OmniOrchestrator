DELETE FROM metrics
WHERE id = :metric_id
AND EXISTS (SELECT 1 FROM metrics WHERE id = :metric_id);