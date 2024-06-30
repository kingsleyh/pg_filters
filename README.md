# PG Filters

[![License](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/kingsleyh/pg_filters#license)
[![Docs](https://docs.rs/pg_filters/badge.svg)](https://docs.rs/pg_filters/latest/pg_filters/)
[![Test](https://github.com/kingsleyh/pg_filters/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/kingsleyh/pg_filters/actions/workflows/ci.yml)

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
            SortedColumn::new("name".into(), "asc".into()),
        ],
        vec![
            FilteringRule::new("name".into(), "=".into(), "and".into(), "John".into()),
            FilteringRule::new("age".into(), ">".into(), "or".into(), "18".into()),
        ],
    );

    let sql = filters.sql();
    assert_eq!(sql, " WHERE name = 'John' OR age > 18 ORDER BY age DESC, name ASC LIMIT 10 OFFSET 0");
```

## Note

* filter rules are applied in the order which they are supplied
* sorting is applied after sorting on column name alphabetically (duplicates are removed)

## Valid Filtering Options

The filtering accepts a filter operator and a conditional operator the valid options are below:

### Filtering Operator

can be upper or lower case

* "="
* "!="
* ">"
* ">="
* "<"
* "<="
* "LIKE"
* "NOT LIKE"
* "IN"
* "NOT IN"
* "IS NULL"
* "IS NOT NULL"


### Valid Conditional Filter Values

can be upper or lower case

* "AND"
* "OR"

## Returned Objects

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

# License

Licensed under either of these:
- MIT([https://opensource.org/licenses/MIT](https://opensource.org/licenses/MIT)) 
- Apache-2.0([https://www.apache.org/licenses/LICENSE-2.0](https://www.apache.org/licenses/LICENSE-2.0)) 