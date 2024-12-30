# PG Filters

[![License](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/kingsleyh/pg_filters#license)
[![Docs](https://docs.rs/pg_filters/badge.svg)](https://docs.rs/pg_filters/latest/pg_filters/)
[![Test](https://github.com/kingsleyh/pg_filters/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/kingsleyh/pg_filters/actions/workflows/ci.yml)
[![Coverage Status](https://coveralls.io/repos/github/kingsleyh/pg_filters/badge.svg?branch=main)](https://coveralls.io/github/kingsleyh/pg_filters?branch=main)
[![Crates](https://img.shields.io/crates/v/pg_filters.svg)](https://crates.io/crates/pg_filters)

A powerful Rust helper to generate PostgreSQL SQL for pagination, sorting, and advanced filtering with support for complex AND/OR conditions.

## Usage

### Column Definitions

First, define your column types:

```rust
use std::collections::HashMap;
use pg_filters::ColumnDef;

fn setup_columns() -> HashMap<&'static str, ColumnDef> {
    let mut columns = HashMap::new();
    columns.insert("name", ColumnDef::Text("name"));
    columns.insert("age", ColumnDef::Integer("age"));
    columns.insert("email", ColumnDef::Text("email"));
    columns.insert("active", ColumnDef::Boolean("active"));
    columns.insert("created_at", ColumnDef::Timestamp("created_at"));
    columns.insert("id", ColumnDef::Uuid("id"));
    columns
}
```

### Simple Filtering

Basic filtering with multiple AND conditions:

```rust
use pg_filters::{PgFilters, PaginationOptions, FilteringOptions, ColumnDef};
use pg_filters::filtering::{FilterCondition, FilterExpression, FilterOperator};
use pg_filters::sorting::SortedColumn;

let columns = setup_columns();

// Create simple conditions
let name_condition = FilterExpression::Condition(FilterCondition::TextValue {
    column: "name".to_string(),
    operator: FilterOperator::Equal,
    value: Some("John".to_string()),
});

let age_condition = FilterExpression::Condition(FilterCondition::IntegerValue {
    column: "age".to_string(),
    operator: FilterOperator::GreaterThan,
    value: Some(18),
});

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
    Some(FilteringOptions::new(
        vec![name_condition, age_condition],
        columns.clone(),
    )),
    columns,
)?;

let sql = filters.sql()?;
// Results in: WHERE (LOWER(name) = LOWER('John') AND age > 18) ORDER BY age DESC, name ASC LIMIT 10 OFFSET 0
```

### Complex Filtering

Example with complex AND/OR conditions:

```rust
use pg_filters::filtering::{FilterExpression, LogicalOperator};

let columns = setup_columns();

// Create individual conditions
let name_condition = FilterExpression::Condition(FilterCondition::TextValue {
    column: "name".to_string(),
    operator: FilterOperator::Equal,
    value: Some("John".to_string()),
});

let age_condition = FilterExpression::Condition(FilterCondition::IntegerValue {
    column: "age".to_string(),
    operator: FilterOperator::GreaterThan,
    value: Some(18),
});

let city_condition = FilterExpression::Condition(FilterCondition::TextValue {
    column: "city".to_string(),
    operator: FilterOperator::In,
    value: Some("New York,London".to_string()),
});

// Create a complex filter: (name = 'John' AND age > 18) OR city IN ('New York', 'London')
let filter_group = FilterExpression::Group {
    operator: LogicalOperator::Or,
    expressions: vec![
        FilterExpression::Group {
            operator: LogicalOperator::And,
            expressions: vec![name_condition, age_condition],
        },
        city_condition,
    ],
};

let filters = PgFilters::new(
    Some(PaginationOptions {
        current_page: 1,
        per_page: 10,
        per_page_limit: 10,
        total_records: 1000,
    }),
    vec![SortedColumn::new("name", "asc")],
    Some(FilteringOptions::new(vec![filter_group], columns.clone())),
    columns,
)?;

let sql = filters.sql()?;
// Results in: WHERE ((LOWER(name) = LOWER('John') AND age > 18) OR city IN ('New York', 'London')) ORDER BY name ASC LIMIT 10 OFFSET 0
```

### JSON Filter Support

PG Filters supports creating filters from JSON input:

```rust
use pg_filters::{JsonFilter, FilterBuilder};

let columns = setup_columns();

let json_filters = vec![
    JsonFilter {
        n: "name".to_string(),      // column name
        f: "LIKE".to_string(),      // filter operator
        v: "%John%".to_string(),    // value
        c: None,                    // connector (AND/OR)
    },
    JsonFilter {
        n: "age".to_string(),
        f: ">".to_string(),
        v: "18".to_string(),
        c: Some("AND".to_string()),
    },
];

let filter_builder = FilterBuilder::from_json_filters(&json_filters, true, &columns)?;
```

### Pagination with Filtered Count

When you need to apply filtering rules for pagination:

```rust
let columns = setup_columns();
let filtering_options = FilteringOptions::new(vec![filter_expression], columns.clone());

// Create count filters
let count_filters = PgFilters::new(
    None,
    vec![],
    Some(filtering_options.clone()),
    columns.clone(),
)?;

// Get total count with filters applied
let count_sql = count_filters.count_sql(schema, table)?;
let total_rows = db.query_one(&count_sql, &[]).await?;
let total_records = total_rows.get::<usize, i64>(0);

// Create pagination options
let pagination = PaginationOptions::new(
    current_page as i64,
    per_page as i64,
    50,
    total_records,
);

// Create final filters with pagination
let filters = PgFilters::new(
    Some(pagination),
    sorting_columns,
    Some(filtering_options),
    columns,
)?;
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
* Uuid - UUID columns (case-sensitive comparison)
* Json/Jsonb - JSON and JSONB columns
* TextArray - TEXT[] array columns
* And many more (see documentation for full list)

## Valid Filtering Operators

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
* "CONTAINS" (for array types)
* "OVERLAPS" (for array types)

### Array Filtering

PG Filters supports filtering on PostgreSQL array columns. Here's how to use array filtering:

```rust
let columns = setup_columns();
columns.insert("services", ColumnDef::TextArray("services"));

// Using JSON filters:

// Find records where services array contains ALL specified values
let contains_filter = JsonFilter {
    n: "services".to_string(),
    f: "CONTAINS".to_string(),
    v: "EPC,Search".to_string(),
    c: None,
};

// Find records where services array contains ANY of the specified values
let overlaps_filter = JsonFilter {
    n: "services".to_string(),
    f: "OVERLAPS".to_string(),
    v: "EPC,Search".to_string(),
    c: None,
};

// Using direct conditions:
let contains_condition = FilterExpression::Condition(FilterCondition::ArrayContains {
    column: "services".to_string(),
    operator: FilterOperator::Contains,
    value: "EPC,Search".to_string(),
});

let overlaps_condition = FilterExpression::Condition(FilterCondition::ArrayOverlap {
    column: "services".to_string(),
    operator: FilterOperator::Overlaps,
    values: vec!["EPC".to_string(), "Search".to_string()],
});
```

Array filtering supports two operations:
* CONTAINS (@>) - Finds records where the array column contains ALL specified values
* OVERLAPS (&&) - Finds records where the array column contains ANY of the specified values

Note: Array operations are case-sensitive and perform exact matching.

### Array Filtering SQL Examples

```sql
-- CONTAINS: Find all records where services include both 'EPC' and 'Search'
services @> ARRAY['EPC','Search']::text[]

-- OVERLAPS: Find all records where services include either 'EPC' or 'Search'
services && ARRAY['EPC','Search']::text[]
```

### Case Sensitivity

By default, text searches are case-insensitive. You can make them case-sensitive using:

```rust
let columns = setup_columns();
FilteringOptions::case_sensitive(vec![filter_expression], columns);
```

## Type-Aware Filtering

PG Filters now handles different column types appropriately:

* Text columns use case-insensitive comparison by default (can be made case-sensitive)
* UUID columns always use case-sensitive comparison
* Numeric columns use direct comparison
* Date/Time types use appropriate format
* Boolean values are handled correctly
* JSON fields use appropriate operators

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