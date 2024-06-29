use pg_filters::{filtering::{ConditionalOperator, FilterOperator, FilterValue, FilteringRule}, sorting::{SortOrder, SortedColumn}, PaginationOptions, PgFilters};

#[test]
fn test_filtering_with_sorting_with_pagination() {
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
        ],
        vec![
            FilteringRule {
                column: "name".to_string(),
                filter_operator: FilterOperator::Equal,
                conditional_operator: ConditionalOperator::And,
                value: FilterValue::String("John".to_string()),
            },
            FilteringRule {
                column: "age".to_string(),
                filter_operator: FilterOperator::GreaterThan,
                conditional_operator: ConditionalOperator::Or,
                value: FilterValue::Int(18),
            },
        ],
    );

   let sql = filters.sql();
    assert_eq!(sql, " WHERE age > 18 AND name = 'John' ORDER BY name ASC LIMIT 10 OFFSET 0");
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
        vec![
            FilteringRule {
                column: "name".to_string(),
                filter_operator: FilterOperator::Equal,
                conditional_operator: ConditionalOperator::And,
                value: FilterValue::String("John".to_string()),
            },
            FilteringRule {
                column: "age".to_string(),
                filter_operator: FilterOperator::GreaterThan,
                conditional_operator: ConditionalOperator::Or,
                value: FilterValue::Int(18),
            },
        ],
    );

    let sql = filters.sql();
    assert_eq!(sql, " WHERE age > 18 AND name = 'John' LIMIT 10 OFFSET 0");
}

#[test]
fn test_filtering_with_sorting_without_pagination() {
    let filters = PgFilters::new(
        None,
        vec![
            SortedColumn {
                column: "name".to_string(),
                order: SortOrder::Asc,
            },
        ],
        vec![
            FilteringRule {
                column: "name".to_string(),
                filter_operator: FilterOperator::Equal,
                conditional_operator: ConditionalOperator::And,
                value: FilterValue::String("John".to_string()),
            },
            FilteringRule {
                column: "age".to_string(),
                filter_operator: FilterOperator::GreaterThan,
                conditional_operator: ConditionalOperator::Or,
                value: FilterValue::Int(18),
            },
        ],
    );

    let sql = filters.sql();
    assert_eq!(sql, " WHERE age > 18 AND name = 'John' ORDER BY name ASC");
}

#[test]
fn test_filtering_without_sorting_without_pagination() {
    let filters = PgFilters::new(
        None,
        vec![],
        vec![
            FilteringRule {
                column: "name".to_string(),
                filter_operator: FilterOperator::Equal,
                conditional_operator: ConditionalOperator::And,
                value: FilterValue::String("John".to_string()),
            },
            FilteringRule {
                column: "age".to_string(),
                filter_operator: FilterOperator::GreaterThan,
                conditional_operator: ConditionalOperator::Or,
                value: FilterValue::Int(18),
            },
        ],
    );

    let sql = filters.sql();
    assert_eq!(sql, " WHERE age > 18 AND name = 'John'");
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
        vec![
            SortedColumn {
                column: "name".to_string(),
                order: SortOrder::Asc,
            },
        ],
        vec![],
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
        vec![],
    );

    let sql = filters.sql();
    assert_eq!(sql, " LIMIT 10 OFFSET 0");
}

#[test]
fn test_filtering_with_sorting_without_pagination_with_empty_filters() {
    let filters = PgFilters::new(
        None,
        vec![
            SortedColumn {
                column: "name".to_string(),
                order: SortOrder::Asc,
            },
        ],
        vec![],
    );

    let sql = filters.sql();
    assert_eq!(sql, " ORDER BY name ASC");
}

#[test]
fn test_filtering_without_sorting_without_pagination_with_empty_filters() {
    let filters = PgFilters::new(None, vec![], vec![]);

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
        vec![
            FilteringRule {
                column: "name".to_string(),
                filter_operator: FilterOperator::Equal,
                conditional_operator: ConditionalOperator::And,
                value: FilterValue::String("John".to_string()),
            },
            FilteringRule {
                column: "age".to_string(),
                filter_operator: FilterOperator::GreaterThan,
                conditional_operator: ConditionalOperator::Or,
                value: FilterValue::Int(18),
            },
        ],
    );

    let sql = filters.sql();
    assert_eq!(sql, " WHERE age > 18 AND name = 'John' LIMIT 10 OFFSET 0");
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
        vec![
            FilteringRule {
                column: "name".to_string(),
                filter_operator: FilterOperator::Equal,
                conditional_operator: ConditionalOperator::And,
                value: FilterValue::String("John".to_string()),
            },
            FilteringRule {
                column: "age".to_string(),
                filter_operator: FilterOperator::GreaterThan,
                conditional_operator: ConditionalOperator::Or,
                value: FilterValue::Int(18),
            },
            FilteringRule {
                column: "email".to_string(),
                filter_operator: FilterOperator::Like,
                conditional_operator: ConditionalOperator::And,
                value: FilterValue::String("gmail.com".to_string()),
            },
        ],
    );

    let sql = filters.sql();
    assert_eq!(
        sql,
        " WHERE age > 18 AND email LIKE 'gmail.com' AND name = 'John' ORDER BY age DESC, name ASC LIMIT 10 OFFSET 0"
    );
}
