// build.rs - Complete SQL INSERT formatter that aligns columns in a grid layout
use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    // Create directory for the formatter source if it doesn't exist
    let project_root = env::current_dir().unwrap();
    let formatter_dir = project_root.join(".cargo/sql_formatter");
    fs::create_dir_all(&formatter_dir).unwrap();

    // Write the SQL formatter source code
    let formatter_src = formatter_dir.join("sql_formatter.rs");
    fs::write(&formatter_src, SQL_FORMATTER_SOURCE).unwrap();

    // Create the Cargo.toml for the formatter
    let formatter_toml = formatter_dir.join("Cargo.toml");
    fs::write(&formatter_toml, SQL_FORMATTER_TOML).unwrap();

    // Set up the post-fmt hook
    let hooks_dir = project_root.join(".cargo/hooks");
    fs::create_dir_all(&hooks_dir).unwrap();

    let hook_script = hooks_dir.join("post-fmt.sh");
    fs::write(&hook_script, POST_FMT_HOOK).unwrap();

    // Make the script executable on Unix-like systems
    #[cfg(not(windows))]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&hook_script).unwrap().permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&hook_script, perms).unwrap();
    }

    // Build the formatter
    println!("Building SQL formatter...");
    let status = Command::new("cargo")
        .current_dir(&formatter_dir)
        .args(&["build", "--release"])
        .status();

    match status {
        Ok(exit_status) if exit_status.success() => {
            println!("SQL formatter built successfully");

            // Copy the formatter to the project's .cargo/bin directory
            let bin_dir = project_root.join(".cargo/bin");
            fs::create_dir_all(&bin_dir).unwrap();

            let formatter_exe = if cfg!(windows) {
                formatter_dir.join("target/release/sql_formatter.exe")
            } else {
                formatter_dir.join("target/release/sql_formatter")
            };

            let dest_exe = if cfg!(windows) {
                bin_dir.join("sql_formatter.exe")
            } else {
                bin_dir.join("sql_formatter")
            };

            if formatter_exe.exists() {
                fs::copy(&formatter_exe, &dest_exe).unwrap();
                println!("SQL formatter installed to {}", dest_exe.display());

                // Create a simple README to document usage
                let readme = project_root.join(".cargo/sql_formatter/README.md");
                fs::write(&readme, README_CONTENT).unwrap();
            } else {
                eprintln!(
                    "Warning: SQL formatter executable not found at {}",
                    formatter_exe.display()
                );
            }
        }
        _ => {
            eprintln!("Warning: Failed to build SQL formatter");
        }
    }

    // Update .gitignore to ignore the formatter build artifacts
    let mut ignore_patterns = Vec::new();
    let gitignore_path = project_root.join(".gitignore");

    if gitignore_path.exists() {
        let gitignore_content = fs::read_to_string(&gitignore_path).unwrap_or_default();
        ignore_patterns = gitignore_content.lines().map(String::from).collect();
    }

    // Add our patterns if they don't exist
    let patterns_to_add = [
        ".cargo/sql_formatter/target/",
        ".cargo/sql_formatter/Cargo.lock",
    ];

    let mut updated = false;
    for pattern in &patterns_to_add {
        if !ignore_patterns.contains(&pattern.to_string()) {
            ignore_patterns.push(pattern.to_string());
            updated = true;
        }
    }

    if updated {
        let mut file = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&gitignore_path)
            .unwrap();

        for pattern in &ignore_patterns {
            writeln!(file, "{}", pattern).unwrap();
        }
    }
}

const SQL_FORMATTER_TOML: &str = r#"[package]
name = "sql_formatter"
version = "0.1.0"
edition = "2021"

[dependencies]
regex = "1"
"#;

const POST_FMT_HOOK: &str = r#"#!/bin/sh
# Auto-generated post-fmt hook to format SQL files with grid-aligned columns
# This runs after 'cargo fmt'

# Locate the formatter
FORMATTER="$(pwd)/.cargo/bin/sql_formatter"

if [ ! -f "$FORMATTER" ]; then
    echo "SQL formatter not found at $FORMATTER, skipping SQL formatting."
    exit 0
fi

# Find all SQL files in the project and format them
find . -name "*.sql" -type f -not -path "./target/*" -not -path "./.cargo/*" -print0 | xargs -0 -n 1 "$FORMATTER"

echo "SQL files formatted with grid alignment"
"#;

const README_CONTENT: &str = r#"# SQL INSERT Grid Formatter

This tool formats SQL INSERT statements in `.sql` files to create a nicely aligned grid layout.

## How it works

When you run `cargo fmt`, this tool will automatically:
1. Find all .sql files in your project
2. Format INSERT statements with perfect column alignment
3. Preserve your SQL's functionality while enhancing readability

## Manual usage

You can also run the formatter manually:

```bash
# Format a specific SQL file in-place
.cargo/bin/sql_formatter path/to/your/file.sql

# Format a file and save to a new location
.cargo/bin/sql_formatter path/to/your/file.sql output.sql

# Format from stdin to stdout
cat file.sql | .cargo/bin/sql_formatter - > formatted.sql
```

This formatter was automatically set up by your project's build script.
"#;

const SQL_FORMATTER_SOURCE: &str = r#"use regex::Regex;
use std::env;
use std::fs;
use std::io::{self, Read, Write};
use std::path::Path;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: {} <input_file> [output_file]", args[0]);
        return Ok(());
    }
    
    let input_path = &args[1];
    let output_path = args.get(2);
    
    let content = if input_path == "-" {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)?;
        buffer
    } else {
        fs::read_to_string(input_path)?
    };
    
    let formatted = format_sql_inserts(&content);
    
    match output_path {
        Some(path) if path != "-" => fs::write(path, formatted)?,
        _ if input_path != "-" => fs::write(input_path, formatted)?,
        _ => io::stdout().write_all(formatted.as_bytes())?,
    }
    
    Ok(())
}

fn format_sql_inserts(sql: &str) -> String {
    // Find all INSERT statements
    let insert_regex = Regex::new(r"(?is)(INSERT\s+INTO\s+\w+\s*\([^)]+\))\s*VALUES\s*\n?\s*\(([^;]+)(?:;|$)").unwrap();
    
    let mut result = String::from(sql);
    let mut offset = 0;
    
    for captures in insert_regex.captures_iter(sql) {
        let full_match = captures.get(0).unwrap();
        let start_pos = full_match.start();
        let end_pos = full_match.end();
        let match_len = end_pos - start_pos;
        
        let header = captures.get(1).unwrap().as_str();
        let values_section = captures.get(2).unwrap().as_str();
        
        // Format the INSERT statement
        let formatted_insert = format_insert_statement(header, values_section);
        
        // Replace the original with the formatted version
        result.replace_range(
            (start_pos + offset)..(end_pos + offset),
            &formatted_insert
        );
        
        // Adjust offset for future replacements
        offset += formatted_insert.len() - match_len;
    }
    
    result
}

fn format_insert_statement(header: &str, values_section: &str) -> String {
    // Split values into rows by finding closing and opening parentheses patterns
    let row_regex = Regex::new(r"\)\s*,\s*\(").unwrap();
    let rows_text = if values_section.contains("),") {
        format!("({})", values_section)
    } else {
        format!("({}", values_section)
    };
    
    let mut rows: Vec<&str> = row_regex.split(&rows_text).collect();
    
    // Clean up rows (remove trailing/leading parentheses)
    for i in 0..rows.len() {
        rows[i] = rows[i].trim();
        if rows[i].ends_with(')') {
            rows[i] = &rows[i][..rows[i].len() - 1];
        }
        if rows[i].starts_with('(') {
            rows[i] = &rows[i][1..];
        }
    }
    
    // Parse each row into values, properly handling quoted strings and functions
    let mut values_per_row: Vec<Vec<String>> = Vec::new();
    for row in rows {
        let mut values = Vec::new();
        let mut current_value = String::new();
        let mut in_quote = false;
        let mut in_function = 0; // Nested function depth counter
        let mut escaped = false;
        
        for c in row.chars() {
            match c {
                '\\' => {
                    current_value.push(c);
                    escaped = true;
                },
                '\'' => {
                    current_value.push(c);
                    if !escaped {
                        in_quote = !in_quote;
                    }
                    escaped = false;
                },
                '(' => {
                    current_value.push(c);
                    if !in_quote {
                        in_function += 1;
                    }
                    escaped = false;
                },
                ')' => {
                    current_value.push(c);
                    if !in_quote && in_function > 0 {
                        in_function -= 1;
                    }
                    escaped = false;
                },
                ',' => {
                    if in_quote || in_function > 0 {
                        current_value.push(c);
                    } else {
                        values.push(current_value.trim().to_string());
                        current_value = String::new();
                    }
                    escaped = false;
                },
                _ => {
                    current_value.push(c);
                    escaped = false;
                }
            }
        }
        
        if !current_value.is_empty() {
            values.push(current_value.trim().to_string());
        }
        
        values_per_row.push(values);
    }
    
    // Find the maximum width for each column
    let column_count = values_per_row.iter().map(|row| row.len()).max().unwrap_or(0);
    let mut column_widths = vec![0; column_count];
    
    for row in &values_per_row {
        for (i, value) in row.iter().enumerate() {
            if i < column_widths.len() {
                column_widths[i] = column_widths[i].max(value.len());
            }
        }
    }
    
    // Format the rows with proper alignment
    let mut formatted_rows = Vec::new();
    
    for row in values_per_row {
        let mut formatted_row = String::from("(");
        
        for (i, value) in row.iter().enumerate() {
            if i > 0 {
                formatted_row.push_str(", ");
            }
            
            // Right-align numbers, POINTs, and numeric functions; left-align everything else
            if value.starts_with("POINT(") || 
               (value.parse::<f64>().is_ok() && !value.starts_with('\'')) || 
               value.parse::<i64>().is_ok() || 
               value == "0" || value == "1" {
                formatted_row.push_str(&format!("{:>width$}", value, width=column_widths[i]));
            } else {
                formatted_row.push_str(&format!("{:<width$}", value, width=column_widths[i]));
            }
        }
        
        formatted_row.push_str("),");
        formatted_rows.push(formatted_row);
    }
    
    // Combine everything with proper layout
    let mut result = String::new();
    result.push_str(header);
    result.push_str("\nVALUES\n");
    result.push_str(&formatted_rows.join("\n"));
    
    // Fix the last row (remove trailing comma, add semicolon)
    if result.ends_with(",") {
        result.pop();
        result.push(';');
    }
    
    result
}
"#;
