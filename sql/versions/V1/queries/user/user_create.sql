-- Check if the email already exists
SELECT CASE 
    WHEN EXISTS (
        SELECT 1
        FROM users
        WHERE email = ?
    )
    THEN RAISE(ROLLBACK, 'Error: Email already exists.')
END;

-- Insert the new user
INSERT INTO users (
    email,             -- Required, from parameter
    name,              -- Required, from parameter
    password,          -- Hash of the user's password
    active,            -- Has DEFAULT 0, but we'll set it from parameter
    last_login_at,     -- Nullable TIMESTAMP
    created_at,        -- DEFAULT CURRENT_TIMESTAMP
    updated_at         -- DEFAULT CURRENT_TIMESTAMP
) 
VALUES (
    ?,                 -- email
    ?,                 -- name
    ?,                 -- password
    ?,                 -- active
    NULL,              -- last_login_at starts NULL
    CURRENT_TIMESTAMP, -- created_at
    CURRENT_TIMESTAMP  -- updated_at
);