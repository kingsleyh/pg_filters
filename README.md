# PG Filters

A simple rust helper to generate postgres sql for pagination, sorting and filtering

## Usage

```rust
 let filters = PgFilters::new(
        Some(PaginationOptions {
            current_page: 1,
            per_page: 10,
            per_page_limit: 10,
            total_records: 1000,
        }),
        vec![
            SortedColumn::new("age".into(), "desc".into()),
            SortedColumn {
                column: "name".into(),
                order: SortOrder::Asc,
            },
        ],
        vec![
            FilteringRule::new("name".into(), "=".into(), "and".into(), "John".into()),
            FilteringRule {
                column: "age".into(),
                filter_operator: FilterOperator::GreaterThan,
                conditional_operator: ConditionalOperator::Or,
                value: FilterValue::Int(18),
            },
        ],
    );

   let sql = filters.sql();
    assert_eq!(sql, " WHERE name = 'John' OR age > 18 ORDER BY age DESC, name ASC LIMIT 10 OFFSET 0");
```

### Note

* filter rules are applied in the order which they are supplied


Along with the sql it also returns objects containing the pagination, sorting and filtering that has been applied e.g :

```rust
  let pagination_sql = filters.pagination.sql
  let pagination = filters.pagination.pagination

  pub struct Paginate {
      pub pagination: Pagination,
      pub sql: String,
  }

  pub struct Pagination {
        current_page,
        previous_page,
        next_page,
        total_pages,
        per_page,
        total_records,
  }
```

see the tests for more examples