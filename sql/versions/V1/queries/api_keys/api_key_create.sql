INSERT INTO cluster.api_keys (org_id, name, key_hash, created_at)
VALUES (:org_id, :name, :key_hash, CURRENT_TIMESTAMP);