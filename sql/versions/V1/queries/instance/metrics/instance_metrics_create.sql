BEGIN TRANSACTION;

-- Input Parameters (example values provided)
-- Replace these placeholders with your actual data
SET @instance_id = 1;  -- The ID of the instance where the metric is being recorded
SET @metric_name = 'cpu_usage';  -- The name of the metric being recorded
SET @metric_value = 75.5;  -- The value of the metric
SET @timestamp = CURRENT_TIMESTAMP;  -- The timestamp when the metric was recorded

-- Insert the new metric
INSERT INTO metrics (instance_id, metric_name, metric_value, timestamp)
VALUES (@instance_id, @metric_name, @metric_value, @timestamp);

-- Output the ID of the newly created metric
SELECT last_insert_rowid() AS new_metric_id;

-- Commit the transaction
COMMIT;