UPDATE domains
SET name = :new_domain_name,
    ssl_enabled = :new_ssl_enabled,
    updated_at = CURRENT_TIMESTAMP
WHERE id = :domain_id
AND EXISTS (SELECT 1 FROM domains WHERE id = :domain_id)
AND NOT EXISTS (
    SELECT 1 
    FROM domains 
    WHERE name = :new_domain_name 
    AND id != :domain_id
);