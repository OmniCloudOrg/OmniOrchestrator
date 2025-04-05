# OmniOrchestrator API Developer Guide

## Architecture Overview

The OmniOrchestrator API follows a layered architecture designed for maintainability, scalability, and extensibility while remaining relativly backwards compatible. This guide focuses on the architectural patterns, implementation guidelines, and best practices for developing new API components.

## Directory Structure

```
./
├── api/
│   ├── mod.rs           # API module registration
│   ├── index.rs         # Landing page and API documentation
│   ├── v1/              # Version 1 API implementation
│   │   ├── mod.rs       # V1 route registration
│   │   ├── apps.rs      # Applications endpoints
│   │   ├── instances.rs # Instances endpoints
│   │   └── ...
│   └── v2/              # Version 2 API implementation
├── db/
│   ├── mod.rs           # Database module configuration
│   ├── utils.rs         # Database utilities
│   └── v1/              # Version 1 database schema
│       ├── mod.rs       # V1 database module registration
│       ├── queries/     # Database query implementations
│       │   ├── mod.rs   # Queries module registration
│       │   ├── app.rs   # Application-related queries
│       │   └── ...
│       └── tables.rs    # Database table definitions
```

## Version Management

### API Versioning Principles

1. **Explicit Versioning**: All routes are explicitly versioned with a prefix (`/api/vX/...`)
2. **Independent Modules**: Each version is contained in its own module
3. **Database Alignment**: API versions align with database schema versions
4. **Backward Compatibility**: New versions should maintain backward compatibility when possible

### Adding a New API Version

When creating a new API version:

1. Create a new directory under `api/` (e.g., `api/v3/`)
2. Add corresponding database schema in `db/v3/`
3. Register the new module in `api/mod.rs`
4. Create migration scripts in `sql/versions/V3/`

## Implementation Guidelines

### Route Handlers

All API endpoint handlers should follow this pattern:

```rust
// Example structure for an endpoint handler
#[get("/resource?<page>&<per_page>")]
pub async fn list_resources(
    pool: &State<sqlx::Pool<MySql>>,
    page: Option<u32>,
    per_page: Option<u32>
) -> Json<Vec<Resource>> {
    // 1. Parameter validation
    let page = page.unwrap_or(1);
    let per_page = per_page.unwrap_or(10);
    
    // 2. Database query execution
    let resources = db::resource::list_resources(pool, page, per_page)
        .await
        .unwrap();
    
    // 3. Response formatting
    Json(resources)
}
```

### Error Handling

For consistent error handling, use the following pattern:

```rust
// Example error handling pattern
pub async fn get_resource(
    pool: &State<sqlx::Pool<MySql>>,
    resource_id: i64
) -> Result<Json<Resource>, Status> {
    // Attempt to fetch resource
    let result = db::resource::get_resource_by_id(pool, resource_id).await;
    
    // Handle result
    match result {
        Ok(resource) => Ok(Json(resource)),
        Err(e) => {
            log::error!("Failed to fetch resource {}: {}", resource_id, e);
            Err(Status::NotFound)
        }
    }
}
```

### Database Queries

Database query functions should:

1. **Be focused**: Perform a single, specific operation
2. **Use transactions**: For operations that modify data
3. **Return meaningful errors**: With context for debugging
4. **Follow consistent naming**: Use predictable function names

Example query implementation:

```rust
pub async fn update_resource(
    pool: &Pool<MySql>,
    id: i64,
    name: Option<&str>
) -> anyhow::Result<Resource> {
    // Create transaction
    let mut tx = pool.begin().await?;

    // Build query dynamically
    let mut query = String::from("UPDATE resources SET updated_at = CURRENT_TIMESTAMP");
    
    if let Some(_) = name {
        query.push_str(", name = ?");
    }
    
    query.push_str(" WHERE id = ?");
    
    // Build and execute query
    let mut db_query = sqlx::query_as::<_, Resource>(&query);
    
    if let Some(name) = name {
        db_query = db_query.bind(name);
    }
    
    db_query = db_query.bind(id);
    
    let resource = db_query
        .fetch_one(&mut *tx)
        .await
        .context("Failed to update resource")?;
    
    // Commit transaction
    tx.commit().await?;
    
    Ok(resource)
}
```

## Testing Strategy

### Unit Tests

Focus on testing individual components in isolation:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_resource_validation() {
        // Test validation logic
    }
}
```

### Integration Tests

Test the full request/response cycle:

```rust
#[async_test]
async fn test_create_resource() {
    // Setup test client
    let client = rocket::local::asynchronous::Client::tracked(
        rocket::build()
            .mount("/api/v1", routes![create_resource])
    ).await.unwrap();
    
    // Perform test request
    let response = client.post("/api/v1/resources")
        .json(&json!({ "name": "Test Resource" }))
        .dispatch()
        .await;
    
    // Assert response
    assert_eq!(response.status(), Status::Created);
    
    // Verify database state
    // ...
}
```

## Authentication & Authorization

Authentication is handled via middleware that:

1. Validates the provided authentication token
2. Extracts user identity and permissions
3. Populates request context with user information

Example authentication middleware:

```rust
#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = AuthError;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        // Extract token from Authorization header
        let token = match request.headers().get_one("Authorization") {
            Some(token) if token.starts_with("Bearer ") => token[7..].to_string(),
            _ => return Outcome::Failure((Status::Unauthorized, AuthError::InvalidToken)),
        };
        
        // Validate token and get user identity
        // ...
        
        // Return authenticated user
        Outcome::Success(user)
    }
}
```

## Database Schema Versioning

Each API version corresponds to a specific database schema version. When making schema changes:

1. Create migration scripts in `sql/versions/VX/up.sql` and `sql/versions/VX/down.sql`
2. Update the corresponding `tables.rs` file to reflect the schema changes
3. Implement query functions in the appropriate modules

## Data Models

Data models are defined in `db/vX/tables.rs` and should:

1. Implement `sqlx::FromRow` for database mapping
2. Implement `Serialize` for JSON responses
3. Use appropriate data types and validation
4. Include documentation comments

Example model definition:

```rust
#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct Resource {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
```

## Audit Logging

All operations that modify data should generate audit logs:

```rust
// Example audit log integration
pub async fn update_resource(
    pool: &State<sqlx::Pool<MySql>>,
    user: User,
    resource_id: i64,
    data: Json<UpdateRequest>
) -> Result<Json<Resource>, Status> {
    // Update resource
    let resource = db::resource::update_resource(pool, resource_id, data.name.as_deref())
        .await
        .map_err(|_| Status::InternalServerError)?;
    
    // Create audit log
    let _ = db::audit_log::create_audit_log(
        pool,
        Some(user.id),
        Some(user.org_id),
        "update",
        "resource",
        Some(resource_id.to_string())
    ).await;
    
    Ok(Json(resource))
}
```

## API Response Format

All API responses should follow a consistent format:

- Success responses: Return the model directly as JSON
- Error responses: Return appropriate HTTP status code with error details

## Performance Considerations

1. **Pagination**: All list endpoints should support pagination
2. **Eager Loading**: Load related data efficiently
3. **Query Optimization**: Use database indexes effectively
4. **Caching**: Implement caching for frequently accessed data

## Documentation

All API endpoints should include:

1. Route documentation explaining the purpose and functionality
2. Parameter documentation describing inputs and constraints
3. Response documentation outlining expected output format

```rust
/// Retrieves a paginated list of resources.
///
/// This endpoint fetches resources with pagination support.
/// Results are ordered by creation time with the most recent first.
///
/// # Query Parameters
///
/// * `page` - Optional page number (1-based, defaults to 1)
/// * `per_page` - Optional items per page (defaults to 10, max 100)
///
/// # Returns
///
/// A JSON array of resource objects
#[get("/resources?<page>&<per_page>")]
pub async fn list_resources(
    // ...
) -> Json<Vec<Resource>> {
    // ...
}
```

## Extensions and Customization

When extending the API:

1. Follow existing patterns for consistency
2. Add new endpoints to the appropriate version module
3. Update route registration in the corresponding `mod.rs` file
4. Add database schema changes when necessary
5. Include comprehensive tests and documentation