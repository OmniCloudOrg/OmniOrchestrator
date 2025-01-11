INSERT INTO domains (app_id, name, ssl_enabled)
SELECT :app_id, :domain_name, :ssl_enabled
WHERE EXISTS (SELECT 1 FROM apps WHERE id = :app_id)
AND NOT EXISTS (
    SELECT 1 
    FROM domains 
    WHERE app_id = :app_id 
    AND name = :domain_name
);