# PG Filters

[![License](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/kingsleyh/pg_filters#license)
[![Docs](https://docs.rs/pg_filters/badge.svg)](https://docs.rs/pg_filters/latest/pg_filters/)
[![Test](https://github.com/kingsleyh/pg_filters/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/kingsleyh/pg_filters/actions/workflows/ci.yml)
[![Coverage Status](https://coveralls.io/repos/github/kingsleyh/pg_filters/badge.svg?branch=main)](https://coveralls.io/github/kingsleyh/pg_filters?branch=main)
[![Crates](https://img.shields.io/crates/v/pg_filters.svg)](https://crates.io/crates/pg_filters)

A powerful Rust helper to generate PostgreSQL SQL for pagination, sorting, and advanced filtering with support for complex AND/OR conditions.

## Usage

### Simple Filtering

Basic filtering with multiple AND conditions:

```rust
use pg_filters::{PgFilters, PaginationOptions, FilteringOptions, ColumnDef};
use pg_filters::filtering::{FilterCondition, FilterExpression, FilterOperator};
use pg_filters::sorting::SortedColumn;

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
    Some(FilteringOptions::new(vec![name_condition, age_condition])),
)?;

let sql = filters.sql()?;
// Results in: WHERE (LOWER(name) = LOWER('John') AND age > 18) ORDER BY age DESC, name ASC LIMIT 10 OFFSET 0
```

### Complex Filtering

Example with complex AND/OR conditions:

```rust
use pg_filters::filtering::{FilterExpression, LogicalOperator};

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

let city_condition = FilterExpression::Condition(FilterCondition::InValues {
    column: "city".to_string(),
    operator: FilterOperator::In,
    values: vec!["New York".to_string(), "London".to_string()],
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
    Some(FilteringOptions::new(vec![filter_group])),
)?;

let sql = filters.sql()?;
// Results in: WHERE ((LOWER(name) = LOWER('John') AND age > 18) OR city IN ('New York', 'London')) ORDER BY name ASC LIMIT 10 OFFSET 0
```

### Nested Complex Conditions

Example with multiple levels of nesting:

```rust
// Create a filter: (name = 'John' AND age > 18) OR (name = 'Jane' AND age < 25)
let filters = FilteringOptions::new(vec![
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
            FilterExpression::Group {
                operator: LogicalOperator::And,
                expressions: vec![
                    FilterExpression::Condition(FilterCondition::TextValue {
                        column: "name".to_string(),
                        operator: FilterOperator::Equal,
                        value: Some("Jane".to_string()),
                    }),
                    FilterExpression::Condition(FilterCondition::IntegerValue {
                        column: "age".to_string(),
                        operator: FilterOperator::LessThan,
                        value: Some(25),
                    }),
                ],
            },
        ],
    }
]);
```

### Pagination with Filtered Count

When you need to apply filtering rules for pagination:

```rust
let filtering_options = FilteringOptions::new(vec![filter_expression]);

let pagination_options = if filtering_options.expressions.is_empty() {
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

### Case Sensitivity

By default, text searches are case-insensitive. You can make them case-sensitive using:

```rust
FilteringOptions::case_sensitive(vec![filter_expression]);
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