BEGIN TRANSACTION;

-- Input Parameter (replace with actual value)
SET @deployment_id = 1; -- The ID of the deployment to remove

-- Check if the deployment exists
IF NOT EXISTS (
    SELECT 1
    FROM deployments
    WHERE id = @deployment_id
)
THEN
    ROLLBACK;
    SELECT 'Error: Deployment not found.' AS error_message;
    LEAVE;
END IF;

-- Remove the deployment
DELETE FROM deployments
WHERE id = @deployment_id;

-- Optional: Remove related logs or references (if necessary)
DELETE FROM instance_logs
WHERE log_type = 'deployment'
  AND EXISTS (
      SELECT 1
      FROM instances
      WHERE id = instance_logs.instance_id
        AND app_id = (SELECT app_id FROM deployments WHERE id = @deployment_id)
  );

-- Commit the transaction
COMMIT;

SELECT 'Deployment removed successfully.' AS status_message;