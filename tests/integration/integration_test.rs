use crate::integration::{get_client, setup_data};
use pg_filters::FilteringOptions;
use pg_filters::{
    filtering::{ColumnName, FilteringRule},
    sorting::SortedColumn,
    PaginationOptions, PgFilters,
};

#[tokio::test]
async fn test_string_int() {
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
            SortedColumn::new("age", "desc"),
            SortedColumn::new("name", "asc"),
        ],
        Some(FilteringOptions::new(vec![
            FilteringRule::new("where", ColumnName::String("name"), "=", "John"),
            FilteringRule::new("or", ColumnName::Int("age"), ">", "10"),
        ])),
    );

    let sql = filters.sql();
    let query = format!("SELECT * FROM person {}", sql);
    let rows = client.query(query.as_str(), &[]).await.unwrap();

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
        ("name16".to_string(), 16),
        ("name15".to_string(), 15),
        ("name14".to_string(), 14),
        ("name13".to_string(), 13),
        ("name12".to_string(), 12),
        ("name11".to_string(), 11),
    ];

    assert_eq!(rows, expected_rows);
}

#[tokio::test]
async fn test_float_bool() {
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
            SortedColumn::new("capacity", "desc"),
            SortedColumn::new("name", "asc"),
        ],
        Some(FilteringOptions::new(vec![
            FilteringRule::new("where", ColumnName::Bool("active"), "=", "true"),
            FilteringRule::new("and", ColumnName::Int("capacity"), ">", "2"),
            FilteringRule::new("and", ColumnName::Int("capacity"), "<", "6"),
        ])),
    );

    let sql = filters.sql();
    let query = format!("SELECT * FROM person {}", sql);
    let rows = client.query(query.as_str(), &[]).await.unwrap();

    let rows: Vec<(String, f64, bool)> = rows
        .iter()
        .map(|row| {
            let name: String = row.get("name");
            let capacity: f64 = row.get("capacity");
            let active: bool = row.get("active");
            (name, capacity, active)
        })
        .collect();

    let expected_rows = vec![("name4".to_string(), 4.0, true)];

    assert_eq!(rows, expected_rows);
}

#[tokio::test]
async fn test_in() {
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
            SortedColumn::new("age", "desc"),
            SortedColumn::new("name", "asc"),
        ],
        Some(FilteringOptions::new(vec![FilteringRule::new(
            "where",
            ColumnName::Int("age"),
            "in",
            "11,12,13",
        )])),
    );

    let sql = filters.sql();
    let query = format!("SELECT * FROM person {}", sql);
    let rows = client.query(query.as_str(), &[]).await.unwrap();

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
}

#[tokio::test]
async fn test_starts_with() {
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
            SortedColumn::new("age", "desc"),
            SortedColumn::new("name", "asc"),
        ],
        Some(FilteringOptions::new(vec![
            FilteringRule::new("where", ColumnName::String("name"), "starts with", "NAme"),
            FilteringRule::new("and", ColumnName::Int("age"), ">=", "17"),
        ])),
    );

    let sql = filters.sql();
    println!("SQL: {}", sql);
    let query = format!("SELECT * FROM person {}", sql);
    let rows = client.query(query.as_str(), &[]).await.unwrap();

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
}
