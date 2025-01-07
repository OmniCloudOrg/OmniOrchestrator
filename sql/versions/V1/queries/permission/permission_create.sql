SELECT 
    CASE 
        WHEN EXISTS (
            SELECT 1 
            FROM permissions 
            WHERE name = @permission_name
        ) THEN 'Error: Permission already exists.'
        ELSE (
            INSERT INTO permissions (name, description, resource_type) 
            VALUES (@permission_name, @permission_description, @resource_type)
        ) || (
            SELECT last_insert_rowid() AS new_permission_id
        ) || 'Permission created successfully.'
    END AS result;