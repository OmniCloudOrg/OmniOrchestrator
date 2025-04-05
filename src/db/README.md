# OmniOrchestrator Database Developer Guide

## Architecture Overview

The OmniOrchestrator database layer is designed with a focus on versioning, maintainability, and performance. This guide outlines the architectural patterns, implementation standards, and best practices for database development.

## Directory Structure

```
./
├── db/
│   ├── mod.rs           # Root database module, schema initialization
│   ├── utils.rs         # SQL utilities and helpers
│   ├── v1/              # Version 1 database components
│   │   ├── mod.rs       # V1 module registration
│   │   ├── queries/     # Database query implementations
│   │   │   ├── mod.rs   # Query module registration
│   │   │   ├── app.rs   # App-related queries
│   │   │   └── ...      # Other resource queries
│   │   └── tables.rs    # Table struct definitions
│   └── v2/              # Version 2 database components
│       └── ...          # Same structure as v1
├── sql/
│   ├── db_init.sql      # Base schema initialization
│   ├── sample_data.sql  # Development test data
│   └── versions/        # Version-specific migrations
│       ├── V1/
│       │   ├── up.sql   # Version 1 migration script
│       │   └── down.sql # Version 1 rollback script
│       └── V2/
│           ├── up.sql   # Version 2 migration script
│           └── down.sql # Version 2 rollback script
```

## Version Management

### Database Versioning Principles

1. **API Alignment**: Each database schema version corresponds directly to an API version
2. **Migration Scripts**: All schema changes are managed through version-specific migration scripts
3. **Isolation**: Each version's code is isolated in its own module
4. **Backward Compatibility**: Schema changes must consider existing data and operations

### Version Transition Process

When creating a new database version:

1. Create a new directory under `db/` (e.g., `db/v2/`)
2. Create migration scripts in `sql/versions/V2/`
3. Copy and modify the previous version's table definitions in a new `tables.rs` file
4. Create new query implementations in the `queries/` directory
5. Register the new module in `db/mod.rs`

## Schema Migration System

### Migration Scripts

Migration scripts follow a consistent pattern:

```sql
-- sql/versions/V2/up.sql
-- Migration: V2 up

-- Add new columns to existing tables
ALTER TABLE apps ADD COLUMN app_type VARCHAR(50) NOT NULL DEFAULT 'standard';

-- Create new tables
CREATE TABLE features (
    id BIGINT NOT NULL AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    -- Additional columns...
);

-- Add foreign keys
ALTER TABLE apps 
    ADD CONSTRAINT fk_apps_feature
    FOREIGN KEY (feature_id) REFERENCES features(id);
```

### Migration Execution

Migrations are executed through the `init_schema` function in `db/mod.rs`:

```rust
pub async fn init_schema(version: i64, pool: &sqlx::Pool<MySql>) -> Result<(), sqlx::Error> {
    // Load base schema
    let mut statements = split_sql_statements(include_str!("../../sql/db_init.sql"));

    // Add all versions up to the requested schema version
    for v in 1..=version {
        let version_file = format!("./sql/versions/V{}/up.sql", v);
        if let Ok(sql) = std::fs::read_to_string(version_file.clone()) {
            statements.extend(split_sql_statements(&sql));
        }
    }

    // Execute each statement
    for statement in statements {
        if !statement.trim().is_empty() {
            sqlx::query(&statement).execute(pool).await?;
        }
    }

    Ok(())
}
```

## Data Model Framework

### Table Definitions

Table structs follow a consistent pattern:

```rust
#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct Resource {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    // Additional fields...
}
```

### Best Practices for Table Definitions

1. **Use appropriate types**: Match Rust types to database column types
2. **Make nullable fields optional**: Use `Option<T>` for nullable columns
3. **Implement necessary traits**: Include `Debug`, `FromRow`, and `Serialize`
4. **Document complex fields**: Add comments for fields with special semantics
5. **Use consistent naming**: Follow database naming conventions (snake_case)

## Query Implementation

### Design Patterns

All database access is implemented through query modules that follow these patterns:

1. **Function per operation**: Each CRUD operation has a dedicated function
2. **Transaction management**: Operations use transactions for data integrity
3. **Error handling**: Functions return `anyhow::Result<T>` for comprehensive error context
4. **Documentation**: Each function has thorough documentation

### Query Function Templates

#### Retrieve Single Entity

```rust
pub async fn get_resource_by_id(pool: &Pool<MySql>, id: i64) -> anyhow::Result<Resource> {
    let resource = sqlx::query_as::<_, Resource>("SELECT * FROM resources WHERE id = ?")
        .bind(id)
        .fetch_one(pool)
        .await
        .context("Failed to fetch resource")?;

    Ok(resource)
}
```

#### List Entities with Pagination

```rust
pub async fn list_resources(
    pool: &Pool<MySql>, 
    page: i64, 
    per_page: i64
) -> anyhow::Result<Vec<Resource>> {
    let resources = sqlx::query_as::<_, Resource>(
        "SELECT * FROM resources ORDER BY created_at DESC LIMIT ? OFFSET ?"
    )
    .bind(per_page)
    .bind(page * per_page)
    .fetch_all(pool)
    .await
    .context("Failed to fetch resources")?;

    Ok(resources)
}
```

#### Create Entity

```rust
pub async fn create_resource(
    pool: &Pool<MySql>,
    name: &str,
    description: Option<&str>
) -> anyhow::Result<Resource> {
    let mut tx = pool.begin().await?;

    let resource = sqlx::query_as::<_, Resource>(
        "INSERT INTO resources (name, description) VALUES (?, ?)"
    )
    .bind(name)
    .bind(description)
    .fetch_one(&mut *tx)
    .await
    .context("Failed to create resource")?;

    tx.commit().await?;
    Ok(resource)
}
```

#### Update Entity

```rust
pub async fn update_resource(
    pool: &Pool<MySql>,
    id: i64,
    name: Option<&str>,
    description: Option<&str>
) -> anyhow::Result<Resource> {
    // Define which fields are being updated
    let update_fields = [
        (name.is_some(), "name = ?"),
        (description.is_some(), "description = ?"),
    ];

    // Build update query with only the fields that have values
    let field_clauses = update_fields
        .iter()
        .filter(|(has_value, _)| *has_value)
        .map(|(_, field)| format!(", {}", field))
        .collect::<String>();

    let query = format!(
        "UPDATE resources SET updated_at = CURRENT_TIMESTAMP{} WHERE id = ?",
        field_clauses
    );

    // Start binding parameters
    let mut db_query = sqlx::query_as::<_, Resource>(&query);

    if let Some(name) = name {
        db_query = db_query.bind(name);
    }
    if let Some(description) = description {
        db_query = db_query.bind(description);
    }

    db_query = db_query.bind(id);

    // Execute the query in a transaction
    let mut tx = pool.begin().await?;
    let resource = db_query
        .fetch_one(&mut *tx)
        .await
        .context("Failed to update resource")?;

    tx.commit().await?;
    Ok(resource)
}
```

#### Delete Entity

```rust
pub async fn delete_resource(pool: &Pool<MySql>, id: i64) -> anyhow::Result<()> {
    let mut tx = pool.begin().await?;

    sqlx::query("DELETE FROM resources WHERE id = ?")
        .bind(id)
        .execute(&mut *tx)
        .await
        .context("Failed to delete resource")?;

    tx.commit().await?;
    Ok(())
}
```

## Testing Strategy

### Unit Tests for Queries

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    // Create test database connection
    async fn setup_test_db() -> Pool<MySql> {
        // Initialize test database
        let pool = sqlx::MySqlPool::connect("mysql://test:test@localhost/test_db")
            .await
            .unwrap();
            
        // Run migrations for specific version
        crate::db::init_schema(1, &pool).await.unwrap();
        
        pool
    }
    
    #[tokio::test]
    async fn test_create_resource() {
        let pool = setup_test_db().await;
        
        // Test create operation
        let resource = create_resource(&pool, "Test Resource", Some("Description"))
            .await
            .unwrap();
            
        // Verify resource was created with expected values
        assert_eq!(resource.name, "Test Resource");
        assert_eq!(resource.description, Some("Description".to_string()));
    }
}
```

## Schema Design Principles

### Consistency Guidelines

1. **Primary Keys**: Always use `id BIGINT NOT NULL AUTO_INCREMENT PRIMARY KEY`
2. **Timestamps**: Include `created_at` and `updated_at` on all tables
3. **Foreign Keys**: Use consistent naming (`table_name_id`)
4. **Indexing**: Index foreign keys and frequently queried columns
5. **Soft Deletes**: Use `deleted_at` timestamp rather than removing records
6. **Status Fields**: Use enums for status fields with clear naming
7. **JSON Fields**: Use for flexible schema needs, but prefer structured columns

### Version-Specific Tables vs. Global Tables

- **Version-Specific Tables**: Tables that change schema between versions
- **Global Tables**: Core tables that remain consistent across versions

## Performance Considerations

### Indexing Strategy

1. **Primary Key Indexes**: Automatically created on primary key columns
2. **Foreign Key Indexes**: Always index foreign key columns
3. **Composite Indexes**: Use for frequent query patterns with multiple conditions
4. **Text Indexes**: Use sparingly and consider partial indexes

Example index creation:

```sql
-- Basic index
CREATE INDEX idx_resources_name ON resources(name);

-- Composite index for common query pattern
CREATE INDEX idx_resources_type_status ON resources(resource_type, status);

-- Partial index for specific values
CREATE INDEX idx_resources_active ON resources(status) WHERE status = 'active';
```

### Query Optimization

1. **Use Explicit Joins**: Prefer explicit JOIN syntax over implicit joins
2. **Limit Result Sets**: Always use pagination for large result sets
3. **Optimize WHERE Clauses**: Put most selective conditions first
4. **Use Prepared Statements**: Leverage SQL parameter binding
5. **Consider Query Plans**: Review execution plans for complex queries

## Cross-Version Compatibility

### Supporting Multiple API Versions

When a new API version is introduced, but both must be supported:

1. **Schema Compatibility**: New schema must be backward compatible or use views
2. **Query Module Versioning**: Maintain separate query modules for each API version
3. **Table Structures**: Use struct field matching to adapt between versions

### Migration Safety

1. **Data Preservation**: Never delete data during migrations
2. **Default Values**: Provide sensible defaults for new columns
3. **Backward Compatibility**: Ensure rollback scripts work correctly
4. **Test Migrations**: Always test migrations with production-like data volumes

## Metadata Storage

### Key-Value Store Pattern

For flexible system configuration, use the metadata pattern:

```rust
pub async fn get_meta_value(pool: &Pool<MySql>, key: &str) -> Result<String> {
    // Implementation...
}

pub async fn set_meta_value(pool: &Pool<MySql>, key: &str, value: &str) -> Result<()> {
    // Implementation...
}
```

## Troubleshooting

### Common Database Issues

1. **Connection Pooling**: Monitor for pool exhaustion
2. **Query Performance**: Use logging for slow queries
3. **Schema Drift**: Regularly verify schema matches expected version
4. **Transaction Handling**: Check for uncommitted transactions

### Diagnostic Queries

```sql
-- Check active connections
SHOW PROCESSLIST;

-- Examine query performance
SHOW PROFILE FOR QUERY <query_id>;

-- View table structure
DESCRIBE <table_name>;

-- Check indexes
SHOW INDEX FROM <table_name>;
```

## Development Workflow

1. **Local Development**: Use Docker for local database instances
2. **Schema Updates**: Always update tables.rs after schema changes
3. **Migration Testing**: Test both up and down migrations
4. **Version Coordination**: Coordinate database and API version releases
5. **Documentation**: Document schema changes in migration files