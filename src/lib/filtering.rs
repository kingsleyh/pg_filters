//! This module contains the structures and functions to handle filtering rules
//!
//! The FilteringRule structure is used to define a rule for filtering data
//! The Filtering structure is used to define a set of filtering rules
//!
//! # Examples
//!
//! ```rust
//! use pg_filters::filtering::{FilteringRule, FilterOperator, FilterValue, Filtering, ConditionalOperator};
//!     
//! let filtering_rule = FilteringRule::new("and".into(), "name".into(), "=".into(), "John".into());
//!
//! assert_eq!(filtering_rule.column, "name");
//! assert_eq!(filtering_rule.filter_operator, FilterOperator::Equal);
//! assert_eq!(filtering_rule.conditional_operator, ConditionalOperator::And);
//! assert_eq!(filtering_rule.value, FilterValue::String("John".into()));
//! ```
//!

/// The FilterValue enum is used to define the value of a filtering rule
///
/// The FilterValue enum can be one of the following:
///
/// * String
/// * Int
/// * Float
/// * Bool
///
/// # Examples
///
/// ```rust
/// use pg_filters::filtering::FilterValue;
///
/// let filter_value = FilterValue::String("John".into());
///
/// assert_eq!(filter_value, FilterValue::String("John".into()));
/// ```
///
#[derive(Debug, Clone, PartialEq)]
pub enum FilterValue {
    String(String),
    Int(i64),
    Float(f64),
    Bool(bool),
}

/// The FilteringRule structure is used to define a rule for filtering data
///
/// The FilteringRule structure contains the following fields:
///
/// * column: The name of the column to filter
/// * filter_operator: The operator to use for filtering
/// * conditional_operator: The operator to use to combine this rule with the next rule
/// * value: The value to use for filtering
///
/// # Examples
///
/// ```rust
/// use pg_filters::filtering::{FilteringRule, FilterOperator, FilterValue, ConditionalOperator};
///
/// let filtering_rule = FilteringRule::new("and".into(), "name".into(), "=".into(), "John".into());
///
/// assert_eq!(filtering_rule.column, "name");
/// assert_eq!(filtering_rule.filter_operator, FilterOperator::Equal);
/// assert_eq!(filtering_rule.conditional_operator, ConditionalOperator::And);
/// assert_eq!(filtering_rule.value, FilterValue::String("John".into()));
/// ```
///
#[derive(Debug, Clone)]
pub struct FilteringRule {
    /// The name of the column to filter
    pub column: String,
    /// The operator to use for filtering
    pub filter_operator: FilterOperator,
    /// The operator to use to combine this rule with the next rule
    pub conditional_operator: ConditionalOperator,
    /// The value to use for filtering
    pub value: FilterValue,
}

/// The FilteringRule structure is used to define a rule for filtering data
///
/// The FilteringRule structure contains the following fields:
///
/// * column: The name of the column to filter
/// * filter_operator: The operator to use for filtering
/// * conditional_operator: The operator to use to combine this rule with the next rule
/// * value: The value to use for filtering
///
/// # Examples
///
/// ```rust
/// use pg_filters::filtering::{FilteringRule, FilterOperator, FilterValue, ConditionalOperator};
///
/// let filtering_rule = FilteringRule::new("and".into(), "name".into(), "=".into(), "John".into());
///
/// assert_eq!(filtering_rule.column, "name");
/// assert_eq!(filtering_rule.filter_operator, FilterOperator::Equal);
/// assert_eq!(filtering_rule.conditional_operator, ConditionalOperator::And);
/// assert_eq!(filtering_rule.value, FilterValue::String("John".into()));
/// ```
///
impl FilteringRule {
    pub fn new(
        conditional_operator: String,
        column: String,
        filter_operator: String,
        value: String,
    ) -> FilteringRule {
        let filter_operator = match filter_operator.to_uppercase().as_str() {
            "=" => FilterOperator::Equal,
            "!=" => FilterOperator::NotEqual,
            ">" => FilterOperator::GreaterThan,
            ">=" => FilterOperator::GreaterThanOrEqual,
            "<" => FilterOperator::LessThan,
            "<=" => FilterOperator::LessThanOrEqual,
            "LIKE" => FilterOperator::Like,
            "NOT LIKE" => FilterOperator::NotLike,
            "IN" => FilterOperator::In,
            "NOT IN" => FilterOperator::NotIn,
            "IS NULL" => FilterOperator::IsNull,
            "IS NOT NULL" => FilterOperator::IsNotNull,
            _ => FilterOperator::Equal,
        };

        let conditional_operator = match conditional_operator.to_uppercase().as_str() {
            "AND" => ConditionalOperator::And,
            "OR" => ConditionalOperator::Or,
            _ => ConditionalOperator::And,
        };

        let value = match filter_operator {
            FilterOperator::In | FilterOperator::NotIn => FilterValue::String(value),
            FilterOperator::IsNull | FilterOperator::IsNotNull => {
                FilterValue::String("".to_string())
            }
            _ => {
                if value.parse::<i64>().is_ok() {
                    FilterValue::Int(value.parse::<i64>().unwrap())
                } else if value.parse::<f64>().is_ok() {
                    FilterValue::Float(value.parse::<f64>().unwrap())
                } else if value.to_lowercase() == "true" || value.to_lowercase() == "false" {
                    FilterValue::Bool(value.parse::<bool>().unwrap())
                } else {
                    FilterValue::String(value)
                }
            }
        };

        FilteringRule {
            column,
            filter_operator,
            conditional_operator,
            value,
        }
    }
}

/// The ConditionalOperator enum is used to define the operator to use to combine filtering rules
///
/// The ConditionalOperator enum can be one of the following:
///
/// * And
/// * Or
///
/// # Examples
///
/// ```rust
/// use pg_filters::filtering::ConditionalOperator;
///
/// let conditional_operator = ConditionalOperator::And;
///
/// assert_eq!(conditional_operator, ConditionalOperator::And);
/// ```
///
#[derive(Debug, Clone, PartialEq)]
pub enum ConditionalOperator {
    And,
    Or,
}

/// The FilterOperator enum is used to define the operator to use for filtering
///
/// The FilterOperator enum can be one of the following:
///
/// * Equal
/// * NotEqual
/// * GreaterThan
/// * GreaterThanOrEqual
/// * LessThan
/// * LessThanOrEqual
/// * Like
/// * NotLike
/// * In
/// * NotIn
/// * IsNull
/// * IsNotNull
///
/// # Examples
///
/// ```rust
/// use pg_filters::filtering::FilterOperator;
///
/// let filter_operator = FilterOperator::Equal;
///
/// assert_eq!(filter_operator, FilterOperator::Equal);
/// ```
///
#[derive(Debug, Clone, PartialEq)]
pub enum FilterOperator {
    Equal,
    NotEqual,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
    Like,
    NotLike,
    In,
    NotIn,
    IsNull,
    IsNotNull,
}

/// The Filtering structure is used to define a set of filtering rules
///
/// The Filtering structure contains the following fields:
///
/// * filters: A vector of FilteringRule structures
/// * sql: The SQL string generated from the filtering rules
///
/// # Examples
///
/// ```rust
/// use pg_filters::filtering::{Filtering, FilteringRule};
///
/// let filters = Filtering::new(vec![
///    FilteringRule::new("and".into(), "name".into(), "=".into(), "John".into()),
///   FilteringRule::new("or".into(), "age".into(), ">".into(), "18".into()),
/// ]);
///
/// assert_eq!(filters.filters.len(), 2);
/// assert_eq!(filters.sql, " WHERE name = 'John' OR age > 18");
/// ```
///
#[derive(Debug, Clone)]
pub struct Filtering {
    pub filters: Vec<FilteringRule>,
    pub sql: String,
}

/// The Filtering structure is used to define a set of filtering rules
///
/// The Filtering structure contains the following fields:
///
/// * filters: A vector of FilteringRule structures
/// * sql: The SQL string generated from the filtering rules
///
/// # Examples
///
/// ```rust
/// use pg_filters::filtering::{Filtering, FilteringRule};
///
/// let filters = Filtering::new(vec![
///   FilteringRule::new("and".into(), "name".into(), "=".into(), "John".into()),
///  FilteringRule::new("or".into(), "age".into(), ">".into(), "18".into()),
/// ]);
///
/// assert_eq!(filters.filters.len(), 2);
/// assert_eq!(filters.sql, " WHERE name = 'John' OR age > 18");
/// ```
///
impl Filtering {
    pub fn new(rules: Vec<FilteringRule>) -> Filtering {
        let mut sql = if !rules.is_empty() {
            " WHERE ".to_string()
        } else {
            "".to_string()
        };
        let mut first = true;

        for rule in rules.iter() {
            if first {
                first = false;
            } else {
                match rule.conditional_operator {
                    ConditionalOperator::And => {
                        sql.push_str(" AND ");
                    }
                    ConditionalOperator::Or => {
                        sql.push_str(" OR ");
                    }
                }
            }
            sql.push_str(&rule.column);
            sql.push(' ');
            match rule.filter_operator {
                FilterOperator::Equal => {
                    sql.push_str("= ");
                }
                FilterOperator::NotEqual => {
                    sql.push_str("!= ");
                }
                FilterOperator::GreaterThan => {
                    sql.push_str("> ");
                }
                FilterOperator::GreaterThanOrEqual => {
                    sql.push_str(">= ");
                }
                FilterOperator::LessThan => {
                    sql.push_str("< ");
                }
                FilterOperator::LessThanOrEqual => {
                    sql.push_str("<= ");
                }
                FilterOperator::Like => {
                    sql.push_str("LIKE ");
                }
                FilterOperator::NotLike => {
                    sql.push_str("NOT LIKE ");
                }
                FilterOperator::In => {
                    sql.push_str("IN ");
                }
                FilterOperator::NotIn => {
                    sql.push_str("NOT IN ");
                }
                FilterOperator::IsNull => {
                    sql.push_str("IS NULL ");
                }
                FilterOperator::IsNotNull => {
                    sql.push_str("IS NOT NULL ");
                }
            }
            let filter_value = match &rule.value {
                FilterValue::String(value) => format!("'{}'", value),
                FilterValue::Int(value) => value.to_string(),
                FilterValue::Float(value) => value.to_string(),
                FilterValue::Bool(value) => value.to_string(),
            };

            if rule.filter_operator == FilterOperator::In
                || rule.filter_operator == FilterOperator::NotIn
            {
                // remove the single quotes from the start and end of the string if present
                let filter_value = filter_value.trim_matches('\'');
                sql.push_str(filter_value);
            } else if rule.filter_operator == FilterOperator::Like
                || rule.filter_operator == FilterOperator::NotLike
            {
                // add the % sign both at start and end of string
                let filter_value = filter_value.trim_matches('\'');
                let filter_value = format!("'%{}%'", filter_value);

                sql.push_str(&filter_value);
            } else if rule.filter_operator != FilterOperator::IsNull
                && rule.filter_operator != FilterOperator::IsNotNull
            {
                sql.push_str(&filter_value);
            }
        }

        Filtering {
            filters: rules,
            sql,
        }
    }
}
