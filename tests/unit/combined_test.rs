use eyre::Result;
use pg_filters::{
    filtering::{FilterCondition, FilterExpression, FilterOperator, LogicalOperator},
    sorting::{SortOrder, SortedColumn}
    , FilteringOptions, PaginationOptions, PgFilters,
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
            FilterExpression::Condition(FilterCondition::TextValue {
                column: "name".to_string(),
                operator: FilterOperator::Equal,
                value: Some("John".to_string()),
            }),
            FilterExpression::Condition(FilterCondition::IntegerValue {
                column: "age".to_string(),
                operator: FilterOperator::GreaterThan,
                value: Some(18),
            }),
        ])),
    )?;

    let sql = filters.sql()?;
    assert_eq!(
        sql,
        " WHERE (LOWER(name) = LOWER('John') AND age > 18) ORDER BY name ASC LIMIT 10 OFFSET 0"
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
            FilterExpression::Condition(FilterCondition::TextValue {
                column: "name".to_string(),
                operator: FilterOperator::Equal,
                value: Some("John".to_string()),
            }),
            FilterExpression::Condition(FilterCondition::IntegerValue {
                column: "age".to_string(),
                operator: FilterOperator::GreaterThan,
                value: Some(18),
            }),
        ])),
    )?;

    let sql = filters.sql()?;
    assert_eq!(
        sql,
        " WHERE (name = 'John' AND age > 18) ORDER BY name ASC LIMIT 10 OFFSET 0"
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
            FilterExpression::Condition(FilterCondition::TextValue {
                column: "name".to_string(),
                operator: FilterOperator::Equal,
                value: Some("John".to_string()),
            }),
            FilterExpression::Condition(FilterCondition::IntegerValue {
                column: "age".to_string(),
                operator: FilterOperator::GreaterThan,
                value: Some(18),
            }),
        ])),
    )?;

    let sql = filters.sql()?;
    assert_eq!(
        sql,
        " WHERE (LOWER(name) = LOWER('John') AND age > 18) LIMIT 10 OFFSET 0"
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
            FilterExpression::Condition(FilterCondition::TextValue {
                column: "name".to_string(),
                operator: FilterOperator::Equal,
                value: Some("John".to_string()),
            }),
            FilterExpression::Condition(FilterCondition::IntegerValue {
                column: "age".to_string(),
                operator: FilterOperator::GreaterThan,
                value: Some(18),
            }),
        ])),
    )?;

    let sql = filters.sql()?;
    assert_eq!(
        sql,
        " WHERE (LOWER(name) = LOWER('John') AND age > 18) ORDER BY name ASC"
    );
    Ok(())
}

#[test]
fn test_filtering_without_sorting_without_pagination() -> Result<()> {
    let filters = PgFilters::new(
        None,
        vec![],
        Some(FilteringOptions::new(vec![
            FilterExpression::Condition(FilterCondition::TextValue {
                column: "name".to_string(),
                operator: FilterOperator::Equal,
                value: Some("John".to_string()),
            }),
            FilterExpression::Condition(FilterCondition::IntegerValue {
                column: "age".to_string(),
                operator: FilterOperator::GreaterThan,
                value: Some(18),
            }),
        ])),
    )?;

    let sql = filters.sql()?;
    assert_eq!(sql, " WHERE (LOWER(name) = LOWER('John') AND age > 18)");
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
            FilterExpression::Condition(FilterCondition::TextValue {
                column: "name".to_string(),
                operator: FilterOperator::Equal,
                value: Some("John".to_string()),
            }),
            FilterExpression::Condition(FilterCondition::IntegerValue {
                column: "age".to_string(),
                operator: FilterOperator::GreaterThan,
                value: Some(18),
            }),
            FilterExpression::Condition(FilterCondition::TextValue {
                column: "email".to_string(),
                operator: FilterOperator::Like,
                value: Some("%gmail.com%".to_string()),
            }),
        ])),
    )?;

    let sql = filters.sql()?;
    assert_eq!(
        sql,
        " WHERE (LOWER(name) = LOWER('John') AND age > 18 AND LOWER(email) LIKE LOWER('%gmail.com%')) ORDER BY age DESC, name ASC LIMIT 10 OFFSET 0"
    );
    Ok(())
}

#[test]
fn test_complex_filtering_with_and_or_conditions() -> Result<()> {
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
            FilterExpression::Group {
                operator: LogicalOperator::Or,
                expressions: vec![
                    FilterExpression::Group {
                        operator: LogicalOperator::And,
                        expressions: vec![
                            FilterExpression::Condition(FilterCondition::TextValue {
                                column: "name".to_string(),
                                operator: FilterOperator::Equal,
                                value: Some("John".to_string()),
                            }),
                            FilterExpression::Condition(FilterCondition::IntegerValue {
                                column: "age".to_string(),
                                operator: FilterOperator::GreaterThan,
                                value: Some(18),
                            }),
                        ],
                    },
                    FilterExpression::Condition(FilterCondition::TextValue {
                        column: "city".to_string(),
                        operator: FilterOperator::In,
                        value: Some("New York,London".to_string()),
                    }),
                ],
            }
        ])),
    )?;

    let sql = filters.sql()?;
    assert_eq!(
        sql,
        " WHERE ((LOWER(name) = LOWER('John') AND age > 18) OR LOWER(city) IN LOWER('New York,London')) ORDER BY name ASC LIMIT 10 OFFSET 0"
    );
    Ok(())
}