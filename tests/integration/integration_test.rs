use crate::integration::{get_client, get_container, setup_data};
use eyre::Result;
use pg_filters::{
    sorting::{SortOrder, SortedColumn},
    ColumnDef, FilteringOptions, PaginationOptions,
    PgFilters,
};

#[tokio::test]
async fn test_string_int() -> Result<()> {
    let client = get_client().await;
    setup_data().await;

    let filters = PgFilters::new(
        Some(PaginationOptions {
            current_page: 1,
            per_page: 10,
            per_page_limit: 10,
            total_records: 1000,
        }),
        vec![
            SortedColumn {
                column: "age".to_string(),
                order: SortOrder::Desc,
            },
            SortedColumn {
                column: "name".to_string(),
                order: SortOrder::Asc,
            },
        ],
        Some(FilteringOptions::new(vec![
            (ColumnDef::Text("name"), "LIKE".to_string(), "%name1%".to_string()),
            (ColumnDef::Integer("age"), ">".to_string(), "10".to_string()),
        ])),
    )?;

    let sql = filters.sql()?;
    println!("Generated SQL: {}", sql);

    let query = format!("SELECT * FROM person {}", sql);
    let rows = client.query(query.as_str(), &[]).await?;

    let rows: Vec<(String, i32)> = rows
        .iter()
        .map(|row| {
            let name: String = row.get("name");
            let age: i32 = row.get("age");
            (name, age)
        })
        .collect();

    // Only expecting entries where name contains "name1" and age > 10
    let expected_rows = vec![
        ("name19".to_string(), 19),
        ("name18".to_string(), 18),
        ("name17".to_string(), 17),
        ("name16".to_string(), 16),
        ("name15".to_string(), 15),
        ("name14".to_string(), 14),
        ("name13".to_string(), 13),
        ("name12".to_string(), 12),
        ("name11".to_string(), 11),
    ];

    assert_eq!(rows, expected_rows);
    get_container().await.as_ref().unwrap().stop().await?;
    Ok(())
}

#[tokio::test]
async fn test_float_bool() -> Result<()> {
    let client = get_client().await;
    setup_data().await;

    let filters = PgFilters::new(
        Some(PaginationOptions {
            current_page: 1,
            per_page: 10,
            per_page_limit: 10,
            total_records: 1000,
        }),
        vec![
            SortedColumn {
                column: "capacity".to_string(),
                order: SortOrder::Desc,
            },
            SortedColumn {
                column: "name".to_string(),
                order: SortOrder::Asc,
            },
        ],
        Some(FilteringOptions::new(vec![
            (ColumnDef::Boolean("active"), "=".to_string(), "true".to_string()),
            (ColumnDef::DoublePrecision("capacity"), ">".to_string(), "2".to_string()),
            (ColumnDef::DoublePrecision("capacity"), "<".to_string(), "6".to_string()),
        ])),
    )?;

    let sql = filters.sql()?;
    println!("Generated SQL: {}", sql);

    let query = format!("SELECT * FROM person {}", sql);
    let rows = client.query(query.as_str(), &[]).await?;

    let rows: Vec<(String, f64, bool)> = rows
        .iter()
        .map(|row| {
            let name: String = row.get("name");
            let capacity: f64 = row.get("capacity");
            let active: bool = row.get("active");
            (name, capacity, active)
        })
        .collect();

    // Expecting entries with even index (active=true) and capacity between 2 and 6
    let expected_rows = vec![
        ("name4".to_string(), 4.0, true),
    ];

    assert_eq!(rows, expected_rows);
    get_container().await.as_ref().unwrap().stop().await?;
    Ok(())
}

#[tokio::test]
async fn test_in() -> Result<()> {
    let client = get_client().await;
    setup_data().await;

    // Use `FilteringOptions` with a comma-separated string for `IN` values
    let filters = PgFilters::new(
        Some(PaginationOptions {
            current_page: 1,
            per_page: 10,
            per_page_limit: 10,
            total_records: 1000,
        }),
        vec![
            SortedColumn {
                column: "age".to_string(),
                order: SortOrder::Desc,
            },
            SortedColumn {
                column: "name".to_string(),
                order: SortOrder::Asc,
            },
        ],
        Some(FilteringOptions::new(vec![
            (ColumnDef::Integer("age"), "IN".to_string(), "11,12,13".to_string()),
        ])),
    )?;

    let sql = filters.sql()?;
    println!("Generated SQL: {}", sql);

    let query = format!("SELECT * FROM person {}", sql);
    let rows = client.query(query.as_str(), &[]).await?;

    let rows: Vec<(String, i32)> = rows
        .iter()
        .map(|row| {
            let name: String = row.get("name");
            let age: i32 = row.get("age");
            (name, age)
        })
        .collect();

    let expected_rows = vec![
        ("name13".to_string(), 13),
        ("name12".to_string(), 12),
        ("name11".to_string(), 11),
    ];

    assert_eq!(rows, expected_rows);
    get_container().await.as_ref().unwrap().stop().await?;
    Ok(())
}

#[tokio::test]
async fn test_starts_with() -> Result<()> {
    let client = get_client().await;
    setup_data().await;

    let filters = PgFilters::new(
        Some(PaginationOptions {
            current_page: 1,
            per_page: 10,
            per_page_limit: 10,
            total_records: 1000,
        }),
        vec![
            SortedColumn {
                column: "age".to_string(),
                order: SortOrder::Desc,
            },
            SortedColumn {
                column: "name".to_string(),
                order: SortOrder::Asc,
            },
        ],
        Some(FilteringOptions::new(vec![
            (ColumnDef::Text("name"), "STARTS WITH".to_string(), "name1".to_string()),
            (ColumnDef::Integer("age"), ">=".to_string(), "17".to_string()),
        ])),
    )?;

    let sql = filters.sql()?;
    println!("Generated SQL: {}", sql);

    let query = format!("SELECT * FROM person {}", sql);
    let rows = client.query(query.as_str(), &[]).await?;

    let rows: Vec<(String, i32)> = rows
        .iter()
        .map(|row| {
            let name: String = row.get("name");
            let age: i32 = row.get("age");
            (name, age)
        })
        .collect();

    let expected_rows = vec![
        ("name19".to_string(), 19),
        ("name18".to_string(), 18),
        ("name17".to_string(), 17),
    ];

    assert_eq!(rows, expected_rows);
    get_container().await.as_ref().unwrap().stop().await?;
    Ok(())
}

#[tokio::test]
async fn test_text_search() -> Result<()> {
    let client = get_client().await;
    setup_data().await;

    let filters = PgFilters::new(
        Some(PaginationOptions {
            current_page: 1,
            per_page: 5,
            per_page_limit: 10,
            total_records: 1000,
        }),
        vec![
            SortedColumn {
                column: "name".to_string(),
                order: SortOrder::Asc,
            },
        ],
        Some(FilteringOptions::new(vec![
            (ColumnDef::Text("name"), "LIKE".to_string(), "%name%".to_string()),
            (ColumnDef::Text("nickname"), "LIKE".to_string(), "%nickname1%".to_string()),
        ])),
    )?;

    let sql = filters.sql()?;
    println!("Generated SQL: {}", sql);

    let query = format!("SELECT * FROM person {}", sql);
    let rows = client.query(query.as_str(), &[]).await?;

    let rows: Vec<(String, String)> = rows
        .iter()
        .map(|row| {
            let name: String = row.get("name");
            let nickname: String = row.get("nickname");
            (name, nickname)
        })
        .collect();

    // Expecting entries where both name contains "name" and nickname contains "nickname1"
    let expected_rows = vec![
        ("name1".to_string(), "nickname1".to_string()),
        ("name10".to_string(), "nickname10".to_string()),
        ("name11".to_string(), "nickname11".to_string()),
        ("name12".to_string(), "nickname12".to_string()),
        ("name13".to_string(), "nickname13".to_string()),
    ];

    assert_eq!(rows, expected_rows);
    get_container().await.as_ref().unwrap().stop().await?;
    Ok(())
}