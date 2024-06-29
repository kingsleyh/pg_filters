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
```

### Note

* filter rules are sorted by column so in the above case age goes first and then name (with the AND conditional operator) - where you might have expected `name = 'John' OR age > 18` because of the sorting it put age first so `age > 18 AND name = 'John'`


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