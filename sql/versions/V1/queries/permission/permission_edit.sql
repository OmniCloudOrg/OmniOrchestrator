SELECT CASE WHEN NOT EXISTS (
    SELECT 1 FROM permissions
    WHERE id = @permission_id
)
THEN 
    'Error: Permission does not exist.'
WHEN EXISTS (
    SELECT 1 FROM permissions
    WHERE name = @new_permission_name
    AND id != @permission_id
)
THEN 'Error: Permission name already exists for another permission.'
ELSE (
    UPDATE permissions 
    SET name = @new_permission_name, description = @new_permission_description, resource_type = @new_resource_type, updated_at = CURRENT_TIMESTAMP
    WHERE id = @permission_id
) || (
    SELECT @permission_id
    AS updated_permission_id
) || 'Permission updated successfully.' END AS result;