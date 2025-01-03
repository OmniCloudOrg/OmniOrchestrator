UPDATE builds
SET source_version = ?,
    status = ?,
    started_at = ?,
    completed_at = ?
WHERE id = ?;