UPDATE config_vars
SET value = ?, is_secret = ?, updated_at = DATETIME('now')
WHERE id = ?;