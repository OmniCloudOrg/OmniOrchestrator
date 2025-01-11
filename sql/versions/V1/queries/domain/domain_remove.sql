DELETE FROM domains
WHERE id = :domain_id
AND EXISTS (SELECT 1 FROM domains WHERE id = :domain_id)
AND NOT EXISTS (
    SELECT 1 
    FROM instances i
    JOIN domains d ON d.id = :domain_id
    WHERE i.app_id = d.app_id
);