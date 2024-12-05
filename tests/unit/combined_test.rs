use eyre::Result;
use pg_filters::{
    sorting::{SortOrder, SortedColumn},
    ColumnDef, FilteringOptions, PaginationOptions, PgFilters,
};

#[test]
fn test_filtering_with_sorting_with_pagination() -> Result<()> {
    let filters = PgFilters::new(
        Some(PaginationOptions {
            current_page: 1,
            per_page: 10,
            per_page_limit: 10,
            total_records: 1000,
        }),
        vec![SortedColumn {
            column: "name".to_string(),
            order: SortOrder::Asc,
        }],
        Some(FilteringOptions::new(vec![
            (ColumnDef::Text("name"), "=".to_string(), "John".to_string()),
            (ColumnDef::Integer("age"), ">".to_string(), "18".to_string()),
        ])),
    )?;

    let sql = filters.sql()?;
    assert_eq!(
        sql,
        " WHERE LOWER(name) = LOWER('John') AND age > 18 ORDER BY name ASC LIMIT 10 OFFSET 0"
    );
    Ok(())
}

#[test]
fn test_filtering_with_case_sensitive() -> Result<()> {
    let filters = PgFilters::new(
        Some(PaginationOptions {
            current_page: 1,
            per_page: 10,
            per_page_limit: 10,
            total_records: 1000,
        }),
        vec![SortedColumn {
            column: "name".to_string(),
            order: SortOrder::Asc,
        }],
        Some(FilteringOptions::case_sensitive(vec![
            (ColumnDef::Text("name"), "=".to_string(), "John".to_string()),
            (ColumnDef::Integer("age"), ">".to_string(), "18".to_string()),
        ])),
    )?;

    let sql = filters.sql()?;
    assert_eq!(
        sql,
        " WHERE name = 'John' AND age > 18 ORDER BY name ASC LIMIT 10 OFFSET 0"
    );
    Ok(())
}

#[test]
fn test_filtering_without_sorting_with_pagination() -> Result<()> {
    let filters = PgFilters::new(
        Some(PaginationOptions {
            current_page: 1,
            per_page: 10,
            per_page_limit: 10,
            total_records: 1000,
        }),
        vec![],
        Some(FilteringOptions::new(vec![
            (ColumnDef::Text("name"), "=".to_string(), "John".to_string()),
            (ColumnDef::Integer("age"), ">".to_string(), "18".to_string()),
        ])),
    )?;

    let sql = filters.sql()?;
    assert_eq!(
        sql,
        " WHERE LOWER(name) = LOWER('John') AND age > 18 LIMIT 10 OFFSET 0"
    );
    Ok(())
}

#[test]
fn test_filtering_with_sorting_without_pagination() -> Result<()> {
    let filters = PgFilters::new(
        None,
        vec![SortedColumn {
            column: "name".to_string(),
            order: SortOrder::Asc,
        }],
        Some(FilteringOptions::new(vec![
            (ColumnDef::Text("name"), "=".to_string(), "John".to_string()),
            (ColumnDef::Integer("age"), ">".to_string(), "18".to_string()),
        ])),
    )?;

    let sql = filters.sql()?;
    assert_eq!(
        sql,
        " WHERE LOWER(name) = LOWER('John') AND age > 18 ORDER BY name ASC"
    );
    Ok(())
}

#[test]
fn test_filtering_without_sorting_without_pagination() -> Result<()> {
    let filters = PgFilters::new(
        None,
        vec![],
        Some(FilteringOptions::new(vec![
            (ColumnDef::Text("name"), "=".to_string(), "John".to_string()),
            (ColumnDef::Integer("age"), ">".to_string(), "18".to_string()),
        ])),
    )?;

    let sql = filters.sql()?;
    assert_eq!(sql, " WHERE LOWER(name) = LOWER('John') AND age > 18");
    Ok(())
}

#[test]
fn test_filtering_with_sorting_with_pagination_with_empty_filters() -> Result<()> {
    let filters = PgFilters::new(
        Some(PaginationOptions {
            current_page: 1,
            per_page: 10,
            per_page_limit: 10,
            total_records: 1000,
        }),
        vec![SortedColumn {
            column: "name".to_string(),
            order: SortOrder::Asc,
        }],
        Some(FilteringOptions::new(vec![])),
    )?;

    let sql = filters.sql()?;
    assert_eq!(sql, " ORDER BY name ASC LIMIT 10 OFFSET 0");
    Ok(())
}

#[test]
fn test_filtering_without_sorting_with_pagination_with_empty_filters() -> Result<()> {
    let filters = PgFilters::new(
        Some(PaginationOptions {
            current_page: 1,
            per_page: 10,
            per_page_limit: 10,
            total_records: 1000,
        }),
        vec![],
        Some(FilteringOptions::new(vec![])),
    )?;

    let sql = filters.sql()?;
    assert_eq!(sql, " LIMIT 10 OFFSET 0");
    Ok(())
}

#[test]
fn test_filtering_with_sorting_without_pagination_with_empty_filters() -> Result<()> {
    let filters = PgFilters::new(
        None,
        vec![SortedColumn {
            column: "name".to_string(),
            order: SortOrder::Asc,
        }],
        Some(FilteringOptions::new(vec![])),
    )?;

    let sql = filters.sql()?;
    assert_eq!(sql, " ORDER BY name ASC");
    Ok(())
}

#[test]
fn test_filtering_without_sorting_without_pagination_with_empty_filters() -> Result<()> {
    let filters = PgFilters::new(None, vec![], Some(FilteringOptions::new(vec![])))?;

    let sql = filters.sql()?;
    assert_eq!(sql, "");
    Ok(())
}

#[test]
fn test_with_many_filters_and_many_sorting_and_pagination() -> Result<()> {
    let filters = PgFilters::new(
        Some(PaginationOptions {
            current_page: 1,
            per_page: 10,
            per_page_limit: 10,
            total_records: 1000,
        }),
        vec![
            SortedColumn {
                column: "name".to_string(),
                order: SortOrder::Asc,
            },
            SortedColumn {
                column: "age".to_string(),
                order: SortOrder::Desc,
            },
        ],
        Some(FilteringOptions::new(vec![
            (ColumnDef::Text("name"), "=".to_string(), "John".to_string()),
            (ColumnDef::Integer("age"), ">".to_string(), "18".to_string()),
            (
                ColumnDef::Text("email"),
                "LIKE".to_string(),
                "%gmail.com%".to_string(),
            ),
        ])),
    )?;

    let sql = filters.sql()?;
    assert_eq!(
        sql,
        " WHERE LOWER(name) = LOWER('John') AND age > 18 AND LOWER(email) LIKE LOWER('%gmail.com%') ORDER BY age DESC, name ASC LIMIT 10 OFFSET 0"
    );
    Ok(())
}
