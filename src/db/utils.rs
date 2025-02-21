/// Split SQL into individual statements while handling edge cases
pub fn split_sql_statements(sql: &str) -> Vec<String> {
    let mut statements = Vec::new();
    let mut current_statement = String::new();
    let mut in_string = false;
    let mut in_comment = false;
    let mut delimiter = ';';

    for line in sql.lines() {
        let trimmed = line.trim();
        
        // Skip empty lines
        if trimmed.is_empty() {
            continue;
        }

        // Handle DELIMITER changes (common in MySQL scripts)
        if trimmed.to_uppercase().starts_with("DELIMITER") {
            if let Some(new_delimiter) = trimmed.chars().nth(9) {
                delimiter = new_delimiter;
                continue;
            }
        }

        // Handle comments
        if trimmed.starts_with("--") || trimmed.starts_with("#") {
            continue;
        }

        if trimmed.starts_with("/*") {
            in_comment = true;
            continue;
        }

        if trimmed.ends_with("*/") {
            in_comment = false;
            continue;
        }

        if in_comment {
            continue;
        }

        // Add the line to current statement
        current_statement.push_str(line);
        current_statement.push('\n');

        // Check for statement termination
        let mut chars: Vec<char> = line.chars().collect();
        while let Some(c) = chars.pop() {
            if c == '"' || c == '\'' {
                in_string = !in_string;
            } else if c == delimiter && !in_string {
                // We found a statement terminator
                if !current_statement.trim().is_empty() {
                    statements.push(current_statement.trim().to_string());
                    current_statement.clear();
                }
                break;
            }
        }
    }

    // Add the last statement if it doesn't end with a delimiter
    if !current_statement.trim().is_empty() {
        statements.push(current_statement.trim().to_string());
    }

    statements
}