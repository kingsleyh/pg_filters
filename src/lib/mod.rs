//! PgFilters - a simple library to handle pagination, sorting and filtering in Postgres
//!
//! # Examples
//!
//! ```rust
//! use pg_filters::{filtering::{FilteringRule, ColumnName}, sorting::{SortedColumn}, PaginationOptions, PgFilters};
//! use pg_filters::FilteringOptions;
//!
//!let filters = PgFilters::new(
//!    Some(PaginationOptions {
//!        current_page: 1,
//!        per_page: 10,
//!        per_page_limit: 10,
//!        total_records: 1000,
//!    }),
//!    vec![
//!        SortedColumn::new("age", "desc"),
//!        SortedColumn::new("name", "asc"),
//!    ],
//!    Some(FilteringOptions::new(vec![
//!        FilteringRule::new("and", ColumnName::String("name"), "=", "John"),
//!        FilteringRule::new("or", ColumnName::Int("age"), ">", "18"),
//!    ])),
//!);
//!
//!let sql = filters.sql();
//!assert_eq!(sql, " WHERE LOWER(name) = LOWER('John') OR age > 18 ORDER BY age DESC, name ASC LIMIT 10 OFFSET 0");
//! ```
//!
//! # Notes
//!
//! This library is designed to work with Postgres databases
//! The total_records field in PaginationOptions must be fetched from the database
//!
//! ### Valid Filter Values
//!
//! can be upper or lower case
//!
//! * "="
//! * "!="
//! * ">"
//! * ">="
//! * "<"
//! * "<="
//! * "LIKE"
//! * "NOT LIKE"
//! * "IN"
//! * "NOT IN"
//! * "IS NULL"
//! * "IS NOT NULL"
//! * "STARTS WITH"
//! * "ENDS WITH"
//!
//!
//! ### Valid Conditional Filter Values
//!
//! can be upper or lower case
//!
//! * "AND"
//! * "OR"
//!
//!
use filtering::{Filtering, FilteringRule};
use pagination::Paginate;
use sorting::{SortedColumn, Sorting};

pub mod filtering;
pub mod pagination;
pub mod sorting;

/// Struct to hold the pagination options, sorting columns and filtering rules
#[derive(Debug, Clone)]
pub struct PgFilters {
    /// Pagination options
    pub pagination: Option<Paginate>,
    /// Sorting columns
    pub sorting: Option<Sorting>,
    /// Filtering rules
    pub filters: Option<Filtering>,
}

/// Struct to hold the pagination options
/// total_records must be fetched from the database and passed to the constructor
#[derive(Debug, Clone)]
pub struct PaginationOptions {
    /// Current page - the page number to fetch, starts from 1 - usually passed from a query parameter in a web application
    pub current_page: i64,
    /// Number of records per page, usually passed from a query parameter in a web application
    pub per_page: i64,
    /// Maximum number of records per page
    pub per_page_limit: i64,
    /// Total number of records - must be fetched from the database and passed here
    pub total_records: i64,
}

/// New function for PaginationOptions
/// total_records must be fetched from the database and passed to the constructor
impl PaginationOptions {
    pub fn new(
        current_page: i64,
        per_page: i64,
        per_page_limit: i64,
        total_records: i64,
    ) -> PaginationOptions {
        PaginationOptions {
            current_page,
            per_page,
            per_page_limit,
            total_records,
        }
    }
}

/// Struct to hold the filtering options
/// filtering_rules is a Vec<FilteringRule>
/// case_insensitive is a boolean to determine if the filtering rules should be case-insensitive
/// By default, case_insensitive is true
/// If case_insensitive is true, the filtering rules will be case-insensitive
pub struct FilteringOptions {
    pub filtering_rules: Vec<eyre::Result<FilteringRule>>,
    pub case_insensitive: bool,
}

/// New function for FilteringOptions
/// filtering_rules is a Vec<eyre::Result<FilteringRule>>
/// case_insensitive is a boolean to determine if the filtering rules should be case-insensitive
/// By default, case_insensitive is true
impl FilteringOptions {
    pub fn new(filtering_rules: Vec<eyre::Result<FilteringRule>>) -> FilteringOptions {
        FilteringOptions {
            filtering_rules,
            case_insensitive: true,
        }
    }

    /// Function to return the filtering object to access the sql for filtering
    pub fn filtering(&self) -> Filtering {
        Filtering::new(&self.filtering_rules, self.case_insensitive)
    }

    /// Function to create case-sensitive filtering rules
    pub fn case_sensitive(filtering_rules: Vec<eyre::Result<FilteringRule>>) -> FilteringOptions {
        FilteringOptions {
            filtering_rules,
            case_insensitive: false,
        }
    }
}

/// New function for PgFilters
/// pagination, sorting_columns and filtering_rules are optional
/// pagination is an Option<PaginationOptions>
/// sorting_columns is a Vec<SortedColumn>
/// filtering_rules is an Option<FilteringOptions>
///
impl PgFilters {
    pub fn new(
        pagination: Option<PaginationOptions>,
        sorting_columns: Vec<SortedColumn>,
        filtering_options: Option<FilteringOptions>,
    ) -> PgFilters {
        let pagination = pagination.map(|pagination| {
            Paginate::new(
                pagination.current_page,
                pagination.per_page,
                pagination.per_page_limit,
                pagination.total_records,
            )
        });

        let sorting = Sorting::new(sorting_columns);

        let filters = filtering_options.map(|filtering_rules| filtering_rules.filtering());

        PgFilters {
            pagination,
            sorting: Some(sorting),
            filters,
        }
    }

    /// Function to generate the SQL query
    pub fn sql(&self) -> String {
        let mut sql = "".to_string();

        if let Some(filters) = &self.filters {
            sql.push_str(&filters.sql);
        }

        if let Some(sorting) = &self.sorting {
            sql.push_str(&sorting.sql);
        }

        if let Some(pagination) = &self.pagination {
            sql.push(' ');
            sql.push_str(&pagination.sql);
        }

        sql
    }
}
