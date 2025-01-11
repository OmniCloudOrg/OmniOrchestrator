UPDATE cluster.builds
SET source_version = :source_version,
    status = :status,
    started_at = :started_at,
    completed_at = :completed_at
WHERE id = :id;