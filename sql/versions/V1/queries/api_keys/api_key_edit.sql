UPDATE cluster.api_keys 
SET name = :name,
    key_hash = :key_hash,
    created_at = CURRENT_TIMESTAMP
WHERE id = :id;