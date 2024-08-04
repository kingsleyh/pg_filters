use pg_filters::filtering::FilterColumn;
use pg_filters::{
    filtering::{ConditionalOperator, FilterOperator, FilteringRule},
    sorting::{SortOrder, SortedColumn},
    FilteringOptions, PaginationOptions, PgFilters,
};

#[test]
fn test_filtering_with_sorting_with_pagination() {
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
            Ok(FilteringRule {
                filter_column: FilterColumn::String("name", "'John'".to_string()),
                filter_operator: FilterOperator::Equal,
                conditional_operator: ConditionalOperator::And,
            }),
            Ok(FilteringRule {
                filter_column: FilterColumn::Int("age", 18),
                filter_operator: FilterOperator::GreaterThan,
                conditional_operator: ConditionalOperator::Or,
            }),
        ])),
    );

    let sql = filters.sql();
    assert_eq!(
        sql,
        " WHERE LOWER(name) = LOWER('John') OR age > 18 ORDER BY name ASC LIMIT 10 OFFSET 0"
    );
}

#[test]
fn test_filtering_with_case_sensitive() {
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
            Ok(FilteringRule {
                filter_column: FilterColumn::String("name", "'John'".to_string()),
                filter_operator: FilterOperator::Equal,
                conditional_operator: ConditionalOperator::And,
            }),
            Ok(FilteringRule {
                filter_column: FilterColumn::Int("age", 18),
                filter_operator: FilterOperator::GreaterThan,
                conditional_operator: ConditionalOperator::Or,
            }),
        ])),
    );

    let sql = filters.sql();
    assert_eq!(
        sql,
        " WHERE name = 'John' OR age > 18 ORDER BY name ASC LIMIT 10 OFFSET 0"
    );
}

#[test]
fn test_filtering_without_sorting_with_pagination() {
    let filters = PgFilters::new(
        Some(PaginationOptions {
            current_page: 1,
            per_page: 10,
            per_page_limit: 10,
            total_records: 1000,
        }),
        vec![],
        Some(FilteringOptions::new(vec![
            Ok(FilteringRule {
                filter_column: FilterColumn::String("name", "'John'".to_string()),
                filter_operator: FilterOperator::Equal,
                conditional_operator: ConditionalOperator::And,
            }),
            Ok(FilteringRule {
                filter_column: FilterColumn::Int("age", 18),
                filter_operator: FilterOperator::GreaterThan,
                conditional_operator: ConditionalOperator::Or,
            }),
        ])),
    );

    let sql = filters.sql();
    assert_eq!(
        sql,
        " WHERE LOWER(name) = LOWER('John') OR age > 18 LIMIT 10 OFFSET 0"
    );
}

#[test]
fn test_filtering_with_sorting_without_pagination() {
    let filters = PgFilters::new(
        None,
        vec![SortedColumn {
            column: "name".to_string(),
            order: SortOrder::Asc,
        }],
        Some(FilteringOptions::new(vec![
            Ok(FilteringRule {
                filter_column: FilterColumn::String("name", "'John'".to_string()),
                filter_operator: FilterOperator::Equal,
                conditional_operator: ConditionalOperator::And,
            }),
            Ok(FilteringRule {
                filter_column: FilterColumn::Int("age", 18),
                filter_operator: FilterOperator::GreaterThan,
                conditional_operator: ConditionalOperator::Or,
            }),
        ])),
    );

    let sql = filters.sql();
    assert_eq!(
        sql,
        " WHERE LOWER(name) = LOWER('John') OR age > 18 ORDER BY name ASC"
    );
}

#[test]
fn test_filtering_without_sorting_without_pagination() {
    let filters = PgFilters::new(
        None,
        vec![],
        Some(FilteringOptions::new(vec![
            Ok(FilteringRule {
                filter_column: FilterColumn::String("name", "'John'".to_string()),
                filter_operator: FilterOperator::Equal,
                conditional_operator: ConditionalOperator::And,
            }),
            Ok(FilteringRule {
                filter_column: FilterColumn::Int("age", 18),
                filter_operator: FilterOperator::GreaterThan,
                conditional_operator: ConditionalOperator::Or,
            }),
        ])),
    );

    let sql = filters.sql();
    assert_eq!(sql, " WHERE LOWER(name) = LOWER('John') OR age > 18");
}

#[test]
fn test_filtering_with_sorting_with_pagination_with_empty_filters() {
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
    );

    let sql = filters.sql();
    assert_eq!(sql, " ORDER BY name ASC LIMIT 10 OFFSET 0");
}

#[test]
fn test_filtering_without_sorting_with_pagination_with_empty_filters() {
    let filters = PgFilters::new(
        Some(PaginationOptions {
            current_page: 1,
            per_page: 10,
            per_page_limit: 10,
            total_records: 1000,
        }),
        vec![],
        Some(FilteringOptions::new(vec![])),
    );

    let sql = filters.sql();
    assert_eq!(sql, " LIMIT 10 OFFSET 0");
}

#[test]
fn test_filtering_with_sorting_without_pagination_with_empty_filters() {
    let filters = PgFilters::new(
        None,
        vec![SortedColumn {
            column: "name".to_string(),
            order: SortOrder::Asc,
        }],
        Some(FilteringOptions::new(vec![])),
    );

    let sql = filters.sql();
    assert_eq!(sql, " ORDER BY name ASC");
}

#[test]
fn test_filtering_without_sorting_without_pagination_with_empty_filters() {
    let filters = PgFilters::new(None, vec![], Some(FilteringOptions::new(vec![])));

    let sql = filters.sql();
    assert_eq!(sql, "");
}

#[test]
fn test_filtering_with_sorting_with_pagination_with_empty_sorting() {
    let filters = PgFilters::new(
        Some(PaginationOptions {
            current_page: 1,
            per_page: 10,
            per_page_limit: 10,
            total_records: 1000,
        }),
        vec![],
        Some(FilteringOptions::new(vec![
            Ok(FilteringRule {
                filter_column: FilterColumn::String("name", "'John'".to_string()),
                filter_operator: FilterOperator::Equal,
                conditional_operator: ConditionalOperator::And,
            }),
            Ok(FilteringRule {
                filter_column: FilterColumn::Int("age", 18),
                filter_operator: FilterOperator::GreaterThan,
                conditional_operator: ConditionalOperator::Or,
            }),
        ])),
    );

    let sql = filters.sql();
    assert_eq!(
        sql,
        " WHERE LOWER(name) = LOWER('John') OR age > 18 LIMIT 10 OFFSET 0"
    );
}

#[test]
fn test_with_many_filters_and_many_sorting_and_pagination() {
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
            Ok(FilteringRule {
                filter_column: FilterColumn::String("name", "'John'".to_string()),
                filter_operator: FilterOperator::Equal,
                conditional_operator: ConditionalOperator::And,
            }),
            Ok(FilteringRule {
                filter_column: FilterColumn::Int("age", 18),
                filter_operator: FilterOperator::GreaterThan,
                conditional_operator: ConditionalOperator::Or,
            }),
            Ok(FilteringRule {
                filter_column: FilterColumn::String("email", "'%gmail.com%'".to_string()),
                filter_operator: FilterOperator::Like,
                conditional_operator: ConditionalOperator::And,
            }),
        ])),
    );

    let sql = filters.sql();
    assert_eq!(
        sql,
        " WHERE LOWER(name) = LOWER('John') OR age > 18 AND LOWER(email) LIKE LOWER('%gmail.com%') ORDER BY age DESC, name ASC LIMIT 10 OFFSET 0"
    );
}
