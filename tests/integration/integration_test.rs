use crate::integration::run_with_container;
use chrono::NaiveDateTime;
use pg_filters::{
    filtering::{FilterCondition, FilterExpression, FilterOperator},
    sorting::{SortOrder, SortedColumn},
    FilteringOptions, PaginationOptions, PgFilters, ColumnDef,
};
use uuid::Uuid;
use std::collections::HashMap;

fn setup_test_columns() -> HashMap<&'static str, ColumnDef> {
    let mut columns = HashMap::new();
    columns.insert("name", ColumnDef::Text("name"));
    columns.insert("nickname", ColumnDef::Text("nickname"));
    columns.insert("age", ColumnDef::Integer("age"));
    columns.insert("capacity", ColumnDef::DoublePrecision("capacity"));
    columns.insert("active", ColumnDef::Boolean("active"));
    columns.insert("registration", ColumnDef::Timestamp("registration"));
    columns.insert("uuid", ColumnDef::Uuid("uuid"));
    columns
}

#[tokio::test]
async fn test_logical_filters() {
    run_with_container(|pool| async move {
        let columns = setup_test_columns();
        let filters = PgFilters::new(
            Some(PaginationOptions {
                current_page: 1,
                per_page: 10,
                per_page_limit: 10,
                total_records: 20,
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
            Some(FilteringOptions::new(
                vec![
                    FilterExpression::Condition(FilterCondition::TextValue {
                        column: "name".to_string(),
                        operator: FilterOperator::Like,
                        value: Some("%name1%".to_string()),
                    }),
                    FilterExpression::Condition(FilterCondition::IntegerValue {
                        column: "age".to_string(),
                        operator: FilterOperator::GreaterThan,
                        value: Some(10),
                    }),
                    FilterExpression::Condition(FilterCondition::DoublePrecisionValue {
                        column: "capacity".to_string(),
                        operator: FilterOperator::LessThanOrEqual,
                        value: Some(15.0),
                    }),
                    FilterExpression::Condition(FilterCondition::BooleanValue {
                        column: "active".to_string(),
                        operator: FilterOperator::Equal,
                        value: Some(true),
                    }),
                ],
                columns.clone(),
            )),
            columns
        )
            .unwrap();

        let sql = filters.sql().unwrap();
        println!("Generated SQL: {}", sql);

        let query = format!("SELECT * FROM person {}", sql);
        let client = pool.get().await.unwrap();
        let rows = client.query(query.as_str(), &[]).await.unwrap();

        let rows: Vec<(String, i32)> = rows
            .iter()
            .map(|row| {
                let name: String = row.get("name");
                let age: i32 = row.get("age");
                (name, age)
            })
            .collect();

        let expected_rows = vec![("name14".to_string(), 14), ("name12".to_string(), 12)];

        assert_eq!(rows, expected_rows);
    })
        .await;
}

#[tokio::test]
async fn test_date_and_uuid() {
    run_with_container(|pool| async move {
        let columns = setup_test_columns();
        let filters = PgFilters::new(
            Some(PaginationOptions {
                current_page: 1,
                per_page: 10,
                per_page_limit: 10,
                total_records: 20,
            }),
            vec![SortedColumn {
                column: "registration".to_string(),
                order: SortOrder::Asc,
            }],
            Some(FilteringOptions::new(
                vec![
                    FilterExpression::Condition(FilterCondition::TimestampValue {
                        column: "registration".to_string(),
                        operator: FilterOperator::GreaterThanOrEqual,
                        value: Some("2023-10-10 12:00:00".to_string()),
                    }),
                    FilterExpression::Condition(FilterCondition::InValues {
                        column: "uuid".to_string(),
                        operator: FilterOperator::In,
                        values: vec![
                            "550e8400-e29b-41d4-a716-446655440001".to_string(),
                            "550e8400-e29b-41d4-a716-446655440003".to_string(),
                        ],
                    }),
                ],
                columns.clone(),
            )),
            columns
        )
            .unwrap();

        let sql = filters.sql().unwrap();
        println!("Generated SQL: {}", sql);

        let query = format!("SELECT * FROM person {}", sql);
        let client = pool.get().await.unwrap();
        let rows = client.query(query.as_str(), &[]).await.unwrap();

        let rows: Vec<(String, NaiveDateTime, Uuid)> = rows
            .iter()
            .map(|row| {
                let name: String = row.get("name");
                let registration: NaiveDateTime = row.get("registration");
                let uuid: Uuid = row.get("uuid");
                (name, registration, uuid)
            })
            .collect();

        let expected_rows = vec![
            (
                "name11".to_string(),
                NaiveDateTime::parse_from_str("2023-10-12 12:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
                Uuid::parse_str("550e8400-e29b-41d4-a716-446655440001").unwrap(),
            ),
            (
                "name13".to_string(),
                NaiveDateTime::parse_from_str("2023-10-14 12:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
                Uuid::parse_str("550e8400-e29b-41d4-a716-446655440003").unwrap(),
            ),
        ];

        assert_eq!(rows, expected_rows);
    })
        .await;
}

#[tokio::test]
async fn test_boolean_and_capacity() {
    run_with_container(|pool| async move {
        let columns = setup_test_columns();
        let filters = PgFilters::new(
            Some(PaginationOptions {
                current_page: 1,
                per_page: 10,
                per_page_limit: 10,
                total_records: 20,
            }),
            vec![SortedColumn {
                column: "capacity".to_string(),
                order: SortOrder::Desc,
            }],
            Some(FilteringOptions::new(
                vec![
                    FilterExpression::Condition(FilterCondition::BooleanValue {
                        column: "active".to_string(),
                        operator: FilterOperator::Equal,
                        value: Some(true),
                    }),
                    FilterExpression::Condition(FilterCondition::DoublePrecisionValue {
                        column: "capacity".to_string(),
                        operator: FilterOperator::LessThanOrEqual,
                        value: Some(10.0),
                    }),
                ],
                columns.clone(),
            )),
            columns
        )
            .unwrap();

        let sql = filters.sql().unwrap();
        println!("Generated SQL: {}", sql);

        let query = format!("SELECT * FROM person {}", sql);
        let client = pool.get().await.unwrap();
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

        let expected_rows = vec![
            ("name10".to_string(), 10.0, true),
            ("name8".to_string(), 8.0, true),
            ("name6".to_string(), 6.0, true),
            ("name4".to_string(), 4.0, true),
            ("name2".to_string(), 2.0, true),
            ("name0".to_string(), 0.0, true),
        ];

        assert_eq!(rows, expected_rows);
    })
        .await;
}

#[tokio::test]
async fn test_name_and_age() {
    run_with_container(|pool| async move {
        let columns = setup_test_columns();
        let filters = PgFilters::new(
            Some(PaginationOptions {
                current_page: 1,
                per_page: 10,
                per_page_limit: 10,
                total_records: 20,
            }),
            vec![
                SortedColumn {
                    column: "age".to_string(),
                    order: SortOrder::Asc,
                },
                SortedColumn {
                    column: "name".to_string(),
                    order: SortOrder::Desc,
                },
            ],
            Some(FilteringOptions::new(
                vec![
                    FilterExpression::Condition(FilterCondition::TextValue {
                        column: "name".to_string(),
                        operator: FilterOperator::Like,
                        value: Some("%name%".to_string()),
                    }),
                    FilterExpression::Condition(FilterCondition::IntegerValue {
                        column: "age".to_string(),
                        operator: FilterOperator::GreaterThan,
                        value: Some(5),
                    }),
                ],
                columns.clone(),
            )),
            columns
        )
            .unwrap();

        let sql = filters.sql().unwrap();
        println!("Generated SQL: {}", sql);

        let query = format!("SELECT * FROM person {}", sql);
        let client = pool.get().await.unwrap();
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
            ("name6".to_string(), 6),
            ("name7".to_string(), 7),
            ("name8".to_string(), 8),
            ("name9".to_string(), 9),
            ("name10".to_string(), 10),
            ("name11".to_string(), 11),
            ("name12".to_string(), 12),
            ("name13".to_string(), 13),
            ("name14".to_string(), 14),
            ("name15".to_string(), 15),
        ];

        assert_eq!(rows, expected_rows);
    })
        .await;
}

#[tokio::test]
async fn test_string_int() {
    run_with_container(|pool| async move {
        let columns = setup_test_columns();
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
            Some(FilteringOptions::new(
                vec![
                    FilterExpression::Condition(FilterCondition::TextValue {
                        column: "name".to_string(),
                        operator: FilterOperator::Like,
                        value: Some("%name1%".to_string()),
                    }),
                    FilterExpression::Condition(FilterCondition::IntegerValue {
                        column: "age".to_string(),
                        operator: FilterOperator::GreaterThan,
                        value: Some(10),
                    }),
                ],
                columns.clone(),
            )),
            columns
        )
            .unwrap();

        let sql = filters.sql().unwrap();
        println!("Generated SQL: {}", sql);

        let query = format!("SELECT * FROM person {}", sql);
        let client = pool.get().await.unwrap();
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
    })
        .await;
}

#[tokio::test]
async fn test_float_bool() {
    run_with_container(|pool| async move {
        let columns = setup_test_columns();
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
            Some(FilteringOptions::new(
                vec![
                    FilterExpression::Condition(FilterCondition::BooleanValue {
                        column: "active".to_string(),
                        operator: FilterOperator::Equal,
                        value: Some(true),
                    }),
                    FilterExpression::Condition(FilterCondition::DoublePrecisionValue {
                        column: "capacity".to_string(),
                        operator: FilterOperator::GreaterThan,
                        value: Some(2.0),
                    }),
                    FilterExpression::Condition(FilterCondition::DoublePrecisionValue {
                        column: "capacity".to_string(),
                        operator: FilterOperator::LessThan,
                        value: Some(6.0),
                    }),
                ],
                columns.clone(),
            )),
            columns
        )
            .unwrap();

        let sql = filters.sql().unwrap();
        println!("Generated SQL: {}", sql);

        let query = format!("SELECT * FROM person {}", sql);
        let client = pool.get().await.unwrap();
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
    })
        .await;
}

#[tokio::test]
async fn test_in() {
    run_with_container(|pool| async move {
        let columns = setup_test_columns();
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
            Some(FilteringOptions::new(
                vec![FilterExpression::Condition(FilterCondition::InValues {
                    column: "age".to_string(),
                    operator: FilterOperator::In,
                    values: vec!["11".to_string(), "12".to_string(), "13".to_string()],
                })],
                columns.clone(),
            )),
            columns
        )
            .unwrap();

        let sql = filters.sql().unwrap();
        println!("Generated SQL: {}", sql);

        let query = format!("SELECT * FROM person {}", sql);
        let client = pool.get().await.unwrap();
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
    })
        .await;
}

#[tokio::test]
async fn test_starts_with() {
    run_with_container(|pool| async move {
        let columns = setup_test_columns();
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
            Some(FilteringOptions::new(
                vec![
                    FilterExpression::Condition(FilterCondition::TextValue {
                        column: "name".to_string(),
                        operator: FilterOperator::StartsWith,
                        value: Some("name1".to_string()),
                    }),
                    FilterExpression::Condition(FilterCondition::IntegerValue {
                        column: "age".to_string(),
                        operator: FilterOperator::GreaterThanOrEqual,
                        value: Some(17),
                    }),
                ],
                columns.clone(),
            )),
            columns
        )
            .unwrap();

        let sql = filters.sql().unwrap();
        println!("Generated SQL: {}", sql);

        let query = format!("SELECT * FROM person {}", sql);
        let client = pool.get().await.unwrap();
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
    })
        .await;
}

#[tokio::test]
async fn test_text_search() {
    run_with_container(|pool| async move {
        let columns = setup_test_columns();
        let filters = PgFilters::new(
            Some(PaginationOptions {
                current_page: 1,
                per_page: 5,
                per_page_limit: 10,
                total_records: 1000,
            }),
            vec![SortedColumn {
                column: "name".to_string(),
                order: SortOrder::Asc,
            }],
            Some(FilteringOptions::new(
                vec![
                    FilterExpression::Condition(FilterCondition::TextValue {
                        column: "name".to_string(),
                        operator: FilterOperator::Like,
                        value: Some("%name%".to_string()),
                    }),
                    FilterExpression::Condition(FilterCondition::TextValue {
                        column: "nickname".to_string(),
                        operator: FilterOperator::Like,
                        value: Some("%nickname1%".to_string()),
                    }),
                ],
                columns.clone(),
            )),
            columns
        )
            .unwrap();

        let sql = filters.sql().unwrap();
        println!("Generated SQL: {}", sql);

        let query = format!("SELECT * FROM person {}", sql);
        let client = pool.get().await.unwrap();
        let rows = client.query(query.as_str(), &[]).await.unwrap();

        let rows: Vec<(String, String)> = rows
            .iter()
            .map(|row| {
                let name: String = row.get("name");
                let nickname: String = row.get("nickname");
                (name, nickname)
            })
            .collect();

        let expected_rows = vec![
            ("name1".to_string(), "nickname1".to_string()),
            ("name10".to_string(), "nickname10".to_string()),
            ("name11".to_string(), "nickname11".to_string()),
            ("name12".to_string(), "nickname12".to_string()),
            ("name13".to_string(), "nickname13".to_string()),
        ];

        assert_eq!(rows, expected_rows);
    })
        .await;
}