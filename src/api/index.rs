use rocket::response::content;
use serde::Serialize;
use lazy_static::lazy_static;
use rocket::Build;
use rocket::Rocket;
use std::sync::Arc;
use std::sync::Mutex;

/// Route information structure for API documentation
#[derive(Serialize, Clone)]
pub struct RouteInfo {
    /// Path of the route
    path: String,
    /// HTTP methods supported by the route
    methods: Vec<String>,
}

/// Response structure for routes listing endpoint
#[derive(Serialize)]
pub struct RoutesResponse {
    /// List of all available routes and their methods
    routes: Vec<RouteInfo>,
}

/// Routes collection that will be populated during startup
#[derive(Clone)]
pub struct RoutesCollection {
    routes: Vec<RouteInfo>,
}

impl RoutesCollection {
    pub fn new() -> Self {
        Self { routes: Vec::new() }
    }

    pub fn add_route(&mut self, path: String, method: String) {
        // Check if the route already exists
        if let Some(route) = self.routes.iter_mut().find(|r| r.path == path) {
            // Add method if it doesn't exist
            if !route.methods.contains(&method) {
                route.methods.push(method);
            }
        } else {
            // Add new route info
            self.routes.push(RouteInfo {
                path,
                methods: vec![method],
            });
        }
    }

    pub fn get_routes(&self) -> Vec<RouteInfo> {
        self.routes.clone()
    }
}

/// Global singleton instance of the routes collection
/// Stores information about all registered API routes
lazy_static! {
    static ref ROUTES_COLLECTION: Arc<Mutex<RoutesCollection>> = Arc::new(Mutex::new(RoutesCollection::new()));
}

/// Decodes any encoded characters in route paths and preserves parameter notation
fn decode_route_path(path: &str) -> String {
    let mut result = path.to_string();
    
    // Handle common Unicode escape sequences
    let replacements = [
        ("\\u003C", "<"), ("\\u003E", ">"),
        ("\\u003c", "<"), ("\\u003e", ">"),
        ("\\u0026", "&"), ("\\u0027", "'"),
        ("\\u0022", "\""), ("\\u003D", "="),
        ("\\u003F", "?"), ("\\u002F", "/"),
    ];
    
    for (encoded, decoded) in replacements.iter() {
        result = result.replace(encoded, decoded);
    }
    
    result
}

fn escape_html_in_path(path: &str) -> String {
    path.replace("<", "&lt;").replace(">", "&gt;")
}

// Keep your existing collect_routes function
pub fn collect_routes(rocket: &Rocket<Build>) {
    let mut routes_collection = ROUTES_COLLECTION.lock().unwrap();
    
    for route in rocket.routes() {
        // Get the path and decode any escaped characters
        let path = decode_route_path(&route.uri.to_string());
        
        routes_collection.add_route(
            path,
            route.method.to_string(),
        );
    }
}

/// Routes listing endpoint providing HTML representation of routes
#[get("/")]
pub fn routes_ui() -> content::RawHtml<String> {
    let routes_collection = ROUTES_COLLECTION.lock().unwrap();
    let routes = routes_collection.get_routes();

    // Collect unique versions dynamically
    let mut versions: Vec<String> = routes
        .iter()
        .filter_map(|route| {
            let path = &route.path;
            if let Some(start) = path.find("/api/v") {
                let rest = &path[start + 6..];
                let end = rest.find('/').unwrap_or(rest.len());
                let version = &rest[..end];
                if version.chars().all(|c| c.is_numeric()) {
                    Some(format!("v{}", version))
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect();

    versions.sort();
    versions.dedup();

    // Add "All Versions" and "Unversioned" options
    let mut version_options = String::from(
        r#"
        <option value="all">All Versions</option>
        <option value="unversioned">Unversioned</option>
    "#,
    );

    // Add detected versions dynamically
    version_options.push_str(
        &versions
            .iter()
            .map(|v| format!(r#"<option value="{}">{}</option>"#, v.to_lowercase(), v))
            .collect::<String>(),
    );

    // Start building the HTML for the table
    let mut route_rows = String::new();

    // Sort routes for better presentation
    let mut sorted_routes = routes.clone();
    sorted_routes.sort_by(|a, b| a.path.cmp(&b.path));

    // Create table rows
    for route in sorted_routes {
        // Sort methods for consistency
        let mut methods = route.methods.clone();
        methods.sort();

        for method in methods {
            let method_class = method.to_lowercase();
            let escaped_path = escape_html_in_path(&route.path);
            let version = if let Some(start) = route.path.find("/api/v") {
                let rest = &route.path[start + 6..];
                let end = rest.find('/').unwrap_or(rest.len());
                format!("v{}", &rest[..end])
            } else {
                "unversioned".to_string()
            };

            route_rows.push_str(&format!(
                r#"<tr class="route-row border-b border-gray-800" data-method="{}" data-path="{}" data-version="{}">
                    <td class="py-3 px-4">
                      <span class="method {} text-sm font-medium px-3 py-1 rounded">{}</span>
                    </td>
                    <td class="py-3 px-4">
                      <a href="{}" class="text-gray-300 hover:text-white hover:underline transition duration-150">{}</a>
                    </td>
                </tr>"#,
                method.to_lowercase(),
                escaped_path.to_lowercase(),
                version.to_lowercase(),
                method_class,
                method,
                escaped_path,
                escaped_path
            ));
        }
    }

    // Complete HTML with search, method, and version filters
    let html = format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>OmniOrchestrator API</title>
    <style>
        :root {{
            --color-gray-900: #111827;
            --color-gray-800: #1F2937;
            --color-gray-700: #374151;
            --color-gray-600: #4B5563;
            --color-gray-500: #6B7280;
            --color-gray-400: #9CA3AF;
            --color-gray-300: #D1D5DB;
            --color-gray-200: #E5E7EB;
            --color-gray-100: #F3F4F6;
            --color-gray-50: #F9FAFB;
            --color-blue-500: #3B82F6;
            --color-blue-600: #2563EB;
            --color-green-500: #10B981;
            --color-yellow-500: #F59E0B;
            --color-red-500: #EF4444;
            --color-purple-500: #8B5CF6;
        }}
        
        * {{
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }}
        
        body {{
            font-family: ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, sans-serif;
            background-color: var(--color-gray-900);
            color: var(--color-gray-300);
            line-height: 1.5;
        }}

        a {{
            color: var(--color-blue-200);
            text-decoration: none;
        }}
        
        .container {{
            max-width: 1200px;
            margin: 0 auto;
            padding: 2rem 1rem;
        }}
        
        .header {{
            margin-bottom: 2rem;
            border-bottom: 1px solid var(--color-gray-800);
            padding-bottom: 1.5rem;
        }}
        
        h1 {{
            font-size: 2rem;
            font-weight: 700;
            margin-bottom: 0.5rem;
            color: white;
        }}
        
        h2 {{
            font-size: 1.5rem;
            font-weight: 600;
            margin-bottom: 1.5rem;
            color: white;
        }}
        
        p {{
            margin-bottom: 1rem;
            color: var(--color-gray-400);
        }}
        
        .filters {{
            display: flex;
            gap: 1rem;
            margin-bottom: 1.5rem;
            flex-wrap: wrap;
        }}
        
        .filter-input {{
            flex: 1;
            min-width: 200px;
            padding: 0.75rem 1rem;
            background-color: var(--color-gray-800);
            border: 1px solid var(--color-gray-700);
            border-radius: 0.5rem;
            color: var(--color-gray-300);
            font-size: 0.875rem;
            transition: all 0.2s;
        }}
        
        .filter-input:focus {{
            outline: none;
            border-color: var(--color-blue-500);
            box-shadow: 0 0 0 2px rgba(59, 130, 246, 0.3);
        }}
        
        .table-container {{
            background-color: var(--color-gray-800);
            border-radius: 0.75rem;
            overflow: hidden;
            box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06);
        }}
        
        table {{
            width: 100%;
            border-collapse: collapse;
        }}
        
        thead {{
            background-color: var(--color-gray-800);
            border-bottom: 2px solid var(--color-gray-700);
        }}
        
        th {{
            text-align: left;
            padding: 1rem;
            font-weight: 600;
            color: var(--color-gray-200);
            text-transform: uppercase;
            font-size: 0.75rem;
            letter-spacing: 0.05em;
        }}
        
        td {{
            padding: 0.75rem 1rem;
        }}
        
        .method {{
            display: inline-block;
            min-width: 60px;
            text-align: center;
            font-weight: 500;
        }}
        
        .get {{
            background-color: var(--color-blue-500);
            color: white;
        }}
        
        .post {{
            background-color: var(--color-green-500);
            color: white;
        }}
        
        .put {{
            background-color: var(--color-yellow-500);
            color: white;
        }}
        
        .delete {{
            background-color: var(--color-red-500);
            color: white;
        }}
        
        .patch {{
            background-color: var(--color-purple-500);
            color: white;
        }}
        
        .empty-message {{
            padding: 2rem;
            text-align: center;
            color: var(--color-gray-500);
        }}

        .border-b {{
            border-bottom-width: 1px;
        }}

        .border-gray-800 {{
            border-color: var(--color-gray-700);
        }}

        /* Responsive adjustments */
        @media (max-width: 640px) {{
            .filters {{
                flex-direction: column;
            }}
            
            .filter-input {{
                width: 100%;
            }}
        }}
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>OmniOrchestrator API</h1>
            <p>Browse the available API endpoints and explore the platform capabilities.</p>
        </div>

        <h2>API Routes</h2>
        
        <div class="filters">
            <input type="text" id="searchInput" class="filter-input" placeholder="Search routes..." onkeyup="filterRoutes()">
            
            <select id="methodFilter" class="filter-input" onchange="filterRoutes()">
                <option value="all">All Methods</option>
                <option value="get">GET</option>
                <option value="post">POST</option>
                <option value="put">PUT</option>
                <option value="delete">DELETE</option>
                <option value="patch">PATCH</option>
            </select>
            
            <select id="versionFilter" class="filter-input" onchange="filterRoutes()">
                {version_options}
            </select>
        </div>
        
        <div class="table-container">
            <table>
                <thead>
                    <tr>
                        <th width="120">Method</th>
                        <th>Endpoint</th>
                    </tr>
                </thead>
                <tbody id="routesTable">
                    {route_rows}
                </tbody>
            </table>
            <div id="emptyMessage" class="empty-message" style="display: none;">
                No routes match your filter criteria
            </div>
        </div>
    </div>

    <script>
        function filterRoutes() {{
            const input = document.getElementById('searchInput').value.toLowerCase();
            const methodFilter = document.getElementById('methodFilter').value.toLowerCase();
            const versionFilter = document.getElementById('versionFilter').value.toLowerCase();
            const rows = document.querySelectorAll('.route-row');
            const emptyMessage = document.getElementById('emptyMessage');
            
            let visibleCount = 0;
            
            rows.forEach(row => {{
                const method = row.getAttribute('data-method').toLowerCase();
                const path = row.getAttribute('data-path').toLowerCase();
                const version = row.getAttribute('data-version').toLowerCase();
                
                // Match criteria
                const methodMatch = methodFilter === 'all' || method === methodFilter;
                const versionMatch = 
                    versionFilter === 'all' || 
                    (versionFilter === 'unversioned' && version === 'unversioned') || 
                    version === versionFilter;
                const pathMatch = path.includes(input);
                
                // Show/hide row based on matches
                if (methodMatch && versionMatch && pathMatch) {{
                    row.style.display = '';
                    visibleCount++;
                }} else {{
                    row.style.display = 'none';
                }}
            }});
            
            // Show empty message if no results
            emptyMessage.style.display = visibleCount > 0 ? 'none' : 'block';
        }}
    </script>
</body>
</html>"#
    );

    content::RawHtml(html)
}