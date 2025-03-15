pub mod utils;
pub mod v1;

use sqlx::{Acquire, MySql};
use syntect::{
    easy::HighlightLines,
    highlighting::{Style, ThemeSet},
    parsing::SyntaxSet,
    util::{as_24_bit_terminal_escaped, LinesWithEndings},
};
use utils::split_sql_statements;
pub use v1::tables;

pub async fn init_schema(version: i64, pool: &sqlx::Pool<MySql>) -> Result<(), sqlx::Error> {
    println!("Initializing schema version {}", version);

    // Load base schema
    let mut statements = split_sql_statements(include_str!("../../sql/db_init.sql"));

    // Add all versions up to the requested schema version
    for v in 1..=version {
        let version_file = format!("./sql/versions/V{}/up.sql", v);
        if let Ok(sql) = std::fs::read_to_string(version_file.clone()) {
            println!("Stepping up to version {} using {}", v, version_file);
            statements.extend(split_sql_statements(&sql));
        }
    }

    // let ps = SyntaxSet::load_defaults_newlines();
    // let ts = ThemeSet::load_defaults();
    // let theme = &ts.themes["base16-ocean.dark"];
    // let new_theme = theme.clone();
    // new_theme.settings.background.unwrap().a = 1;
    // let syntax = ps.find_syntax_by_extension("sql").unwrap();
    // let mut h = HighlightLines::new(syntax, &new_theme);
    // let statements_str = statements.join("\n");
    // for line in LinesWithEndings::from(statements_str.as_str()) {
    //     let ranges: Vec<(Style, &str)> = h.highlight_line(line, &ps).unwrap();
    //     let escaped = as_24_bit_terminal_escaped(&ranges[..], true);
    //     print!("{}", escaped);
    // }

    // Execute each statement separately
    for statement in statements {
        if !statement.trim().is_empty() {
            println!("Executing statement: {}", statement);
            sqlx::query(&statement).execute(pool).await?;
        }
    }

    Ok(())
}

pub async fn sample_data(pool: &sqlx::Pool<MySql>) -> Result<(), sqlx::Error> {
    let mut conn = pool.acquire().await?;
    let trans = conn.begin().await?;
    let statements = split_sql_statements(include_str!("../../sql/sample_data.sql"));

    // Execute each statement separately
    for statement in statements {
        if !statement.trim().is_empty() {
            println!("Executing statement: {}", statement);
            sqlx::query(&statement).execute(pool).await?;
        }
    }

    Ok(())
}
