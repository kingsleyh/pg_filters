# PG Filters

[![License](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/kingsleyh/pg_filters#license)
[![Docs](https://docs.rs/pg_filters/badge.svg)](https://docs.rs/pg_filters/latest/pg_filters/)
[![Test](https://github.com/kingsleyh/pg_filters/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/kingsleyh/pg_filters/actions/workflows/ci.yml)
[![Coverage Status](https://coveralls.io/repos/github/kingsleyh/pg_filters/badge.svg?branch=main)](https://coveralls.io/github/kingsleyh/pg_filters?branch=main)
[![Crates](https://img.shields.io/crates/v/pg_filters.svg)](https://crates.io/crates/pg_filters)

A simple rust helper to generate postgres sql for pagination, sorting and filtering

## Usage

```rust
use pg_filters::{PgFilters, PaginationOptions, FilteringOptions, ColumnDef};
use pg_filters::sorting::{SortedColumn, SortOrder};

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
        (ColumnDef::Text("name"), "=".to_string(), "John".to_string()),
        (ColumnDef::Integer("age"), ">".to_string(), "18".to_string()),
    ])),
)?;

let sql = filters.sql()?;
assert_eq!(sql, " WHERE LOWER(name) = LOWER('John') AND age > 18 ORDER BY age DESC, name ASC LIMIT 10 OFFSET 0");
```

If you need to apply the filtering rules for pagination you can get the sql for that from the filter options:

```rust
let filtering_options = FilteringOptions::new(vec![
    (ColumnDef::Text("name"), "=".to_string(), "John".to_string()),
]);

let pagination_options = if filtering_options.columns.is_empty() {
    let total_rows = db.query_one(total_rows_select_statement.as_str(), &[])
        .await
        .map_err(|e| eyre::eyre!("Error getting total rows: {}", e))?;
    let total_records = total_rows.get::<usize, i64>(0);

    PaginationOptions::new(
        current_page as i64,
        per_page as i64,
        50,
        total_records as i64,
    )
} else {
    let builder = filtering_options.to_filter_builder()?;
    let filtering_sql = builder.build()?;
    let count_sql = format!("select count(*) from {}", filtering_sql);

    let total_rows = db.query_one(count_sql.as_str(), &[])
        .await
        .map_err(|e| eyre::eyre!("Error getting total rows: {}", e))?;
    let total_records = total_rows.get::<usize, i64>(0);
    
    PaginationOptions::new(
        current_page as i64,
        per_page as i64,
        50,
        total_records as i64,
    )
};
```

## Supported Column Types

* Text - Text/VARCHAR/CHAR columns
* Integer - INTEGER columns
* BigInt - BIGINT columns
* SmallInt - SMALLINT columns
* Boolean - BOOLEAN columns
* DoublePrecision - DOUBLE PRECISION columns
* Real - REAL columns
* Date - DATE columns
* Timestamp - TIMESTAMP columns
* TimestampTz - TIMESTAMP WITH TIME ZONE columns
* Uuid - UUID columns
* Json/Jsonb - JSON and JSONB columns

## Valid Filtering Options

The filtering supports various operators for different column types:

### Filtering Operators

Can be upper or lower case:

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
* "STARTS WITH"
* "ENDS WITH"

### Case Sensitivity

By default, text searches are case-insensitive. You can make them case-sensitive using:

```rust
FilteringOptions::case_sensitive(vec![
    (ColumnDef::Text("name"), "=".to_string(), "John".to_string()),
]);
```

## Pagination Details

The pagination information is returned in the following structure:

```rust
pub struct Paginate {
    pub pagination: Pagination,
    pub sql: String,
}

pub struct Pagination {
    current_page: i64,
    previous_page: i64,
    next_page: i64,
    total_pages: i64,
    per_page: i64,
    total_records: i64,
}
```

See the tests for more examples.

# License

Licensed under either of these:
- MIT ([https://opensource.org/licenses/MIT](https://opensource.org/licenses/MIT))
- Apache-2.0 ([https://www.apache.org/licenses/LICENSE-2.0](https://www.apache.org/licenses/LICENSE-2.0))