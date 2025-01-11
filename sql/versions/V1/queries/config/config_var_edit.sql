UPDATE config_vars
SET value = :value, 
    is_secret = :is_secret, 
    updated_at = DATETIME('now')
WHERE id = :id;