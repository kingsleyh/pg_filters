use crate::ColumnDef;
use eyre::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::fmt::Debug;

#[derive(Debug, Clone, PartialEq)]
pub enum LogicalOperator {
    And,
    Or,
}

impl LogicalOperator {
    pub fn as_sql(&self) -> &'static str {
        match self {
            LogicalOperator::And => "AND",
            LogicalOperator::Or => "OR",
        }
    }
}

impl fmt::Display for LogicalOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let operator_str = match self {
            LogicalOperator::And => "AND",
            LogicalOperator::Or => "OR",
        };
        write!(f, "{}", operator_str)
    }
}

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
    StartsWith,
    EndsWith,
    Contains,
    Overlaps,
    DateEqual,
    DateRange,
    RelativeDate,
}

impl FilterOperator {
    pub fn as_sql(&self) -> &'static str {
        match self {
            FilterOperator::Equal => "=",
            FilterOperator::NotEqual => "!=",
            FilterOperator::GreaterThan => ">",
            FilterOperator::GreaterThanOrEqual => ">=",
            FilterOperator::LessThan => "<",
            FilterOperator::LessThanOrEqual => "<=",
            FilterOperator::Like => "LIKE",
            FilterOperator::NotLike => "NOT LIKE",
            FilterOperator::In => "IN",
            FilterOperator::NotIn => "NOT IN",
            FilterOperator::IsNull => "IS NULL",
            FilterOperator::IsNotNull => "IS NOT NULL",
            FilterOperator::StartsWith => "LIKE",
            FilterOperator::EndsWith => "LIKE",
            FilterOperator::Contains => "@>",
            FilterOperator::Overlaps => "&&",
            FilterOperator::DateEqual => "=",
            FilterOperator::DateRange => "BETWEEN",
            FilterOperator::RelativeDate => ">",
        }
    }

    pub fn format_value(&self, value: &str) -> String {
        match self {
            FilterOperator::StartsWith => format!("{}%", value),
            FilterOperator::EndsWith => format!("%{}", value),
            _ => value.to_string(),
        }
    }

    pub fn format_values<T: ToString>(&self, values: &[T]) -> String {
        values
            .iter()
            .map(|v| format!("'{}'", v.to_string().replace('\'', "''")))
            .collect::<Vec<String>>()
            .join(", ")
    }
}

impl fmt::Display for FilterOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let operator_str = self.as_sql();
        write!(f, "{}", operator_str)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum FilterExpression {
    Condition(FilterCondition),
    Group {
        operator: LogicalOperator,
        expressions: Vec<FilterExpression>,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonFilter {
    pub n: String,         // name/column
    pub f: String,         // filter operator
    pub v: String,         // value
    pub c: Option<String>, // optional connector (AND/OR)
}

impl FilterExpression {
    pub fn to_sql(&self, case_insensitive: bool) -> Result<String> {
        match self {
            FilterExpression::Condition(condition) => condition.to_sql(case_insensitive),
            FilterExpression::Group {
                operator,
                expressions,
            } => {
                if expressions.is_empty() {
                    return Ok(String::new());
                }

                let conditions: Result<Vec<String>> = expressions
                    .iter()
                    .map(|expr| expr.to_sql(case_insensitive))
                    .collect();

                let conditions = conditions?;
                Ok(format!(
                    "({})",
                    conditions.join(&format!(" {} ", operator.as_sql()))
                ))
            }
        }
    }

    pub fn and(expressions: Vec<FilterExpression>) -> Self {
        FilterExpression::Group {
            operator: LogicalOperator::And,
            expressions,
        }
    }

    pub fn or(expressions: Vec<FilterExpression>) -> Self {
        FilterExpression::Group {
            operator: LogicalOperator::Or,
            expressions,
        }
    }
}

impl fmt::Display for FilterExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FilterExpression::Condition(condition) => write!(f, "{}", condition),
            FilterExpression::Group {
                operator,
                expressions,
            } => {
                write!(
                    f,
                    "({} {})",
                    operator,
                    expressions
                        .iter()
                        .map(|e| e.to_string())
                        .collect::<Vec<_>>()
                        .join(&format!(" {} ", operator))
                )
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum DateRangeType {
    /// Exact timestamp match
    Exact(String),
    /// Match entire day
    DateOnly(String),
    /// Custom date range
    Range { start: String, end: String },
    /// Relative date expression
    Relative(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum FilterCondition {
    // Character Types
    TextValue {
        column: String,
        operator: FilterOperator,
        value: Option<String>,
    },
    VarcharValue {
        column: String,
        operator: FilterOperator,
        value: Option<String>,
    },
    CharValue {
        column: String,
        operator: FilterOperator,
        value: Option<String>,
    },

    // Date ranges
    DateRange {
        column: String,
        range_type: DateRangeType,
    },

    // Multi-value conditions for IN/NOT IN
    InValues {
        column: String,
        operator: FilterOperator,
        values: Vec<String>,
    },

    // Numeric Types
    SmallIntValue {
        column: String,
        operator: FilterOperator,
        value: Option<i16>,
    },
    IntegerValue {
        column: String,
        operator: FilterOperator,
        value: Option<i32>,
    },
    BigIntValue {
        column: String,
        operator: FilterOperator,
        value: Option<i64>,
    },
    RealValue {
        column: String,
        operator: FilterOperator,
        value: Option<f32>,
    },
    DoublePrecisionValue {
        column: String,
        operator: FilterOperator,
        value: Option<f64>,
    },

    // Date/Time Types
    DateValue {
        column: String,
        operator: FilterOperator,
        value: Option<String>,
    },
    TimeValue {
        column: String,
        operator: FilterOperator,
        value: Option<String>,
    },
    TimeTzValue {
        column: String,
        operator: FilterOperator,
        value: Option<String>,
    },
    TimestampValue {
        column: String,
        operator: FilterOperator,
        value: Option<String>,
    },
    TimestampTzValue {
        column: String,
        operator: FilterOperator,
        value: Option<String>,
    },
    IntervalValue {
        column: String,
        operator: FilterOperator,
        value: Option<String>,
    },

    // Boolean Type
    BooleanValue {
        column: String,
        operator: FilterOperator,
        value: Option<bool>,
    },

    // Network Address Types
    InetValue {
        column: String,
        operator: FilterOperator,
        value: Option<String>,
    },
    CidrValue {
        column: String,
        operator: FilterOperator,
        value: Option<String>,
    },
    MacAddrValue {
        column: String,
        operator: FilterOperator,
        value: Option<String>,
    },
    MacAddr8Value {
        column: String,
        operator: FilterOperator,
        value: Option<String>,
    },

    // UUID Type
    UuidValue {
        column: String,
        operator: FilterOperator,
        value: Option<String>,
    },

    // JSON Types
    JsonValue {
        column: String,
        operator: FilterOperator,
        value: Option<String>,
    },
    JsonbValue {
        column: String,
        operator: FilterOperator,
        value: Option<String>,
    },

    // Text Array Types
    ArrayContains {
        column: String,
        operator: FilterOperator,
        value: String,
    },
    ArrayOverlap {
        column: String,
        operator: FilterOperator,
        values: Vec<String>,
    },

    // Binary Data
    ByteAValue {
        column: String,
        operator: FilterOperator,
        value: Option<String>,
    },

    // Money
    MoneyValue {
        column: String,
        operator: FilterOperator,
        value: Option<String>,
    },

    // XML
    XmlValue {
        column: String,
        operator: FilterOperator,
        value: Option<String>,
    },
}

impl FilterCondition {
    fn format_value<T: fmt::Display>(
        column: &str,
        operator: &FilterOperator,
        value: Option<T>,
    ) -> String {
        match value {
            Some(v) => format!("{} {} {}", column, operator.as_sql(), v),
            None => format!("{} {}", column, operator.as_sql()),
        }
    }

    fn format_string_value(column: &str, operator: &FilterOperator, value: Option<&str>) -> String {
        match value {
            Some(v) => format!(
                "{} {} '{}'",
                column,
                operator.as_sql(),
                v.replace('\'', "''")
            ),
            None => format!("{} {}", column, operator.as_sql()),
        }
    }

    pub fn to_sql(&self, case_insensitive: bool) -> Result<String> {
        match self {
            FilterCondition::TextValue {
                column,
                operator,
                value,
            }
            | FilterCondition::VarcharValue {
                column,
                operator,
                value,
            }
            | FilterCondition::CharValue {
                column,
                operator,
                value,
            } => match value {
                Some(v) => {
                    let formatted_value = operator.format_value(&v.replace('\'', "''"));
                    if case_insensitive {
                        Ok(format!(
                            "LOWER({}) {} LOWER('{}')",
                            column,
                            operator.as_sql(),
                            formatted_value
                        ))
                    } else {
                        Ok(format!(
                            "{} {} '{}'",
                            column,
                            operator.as_sql(),
                            formatted_value
                        ))
                    }
                }
                None => Ok(format!("{} {}", column, operator.as_sql())),
            },

            FilterCondition::DateRange { column, range_type } => match range_type {
                DateRangeType::Exact(timestamp) => Ok(format!("{} = '{}'", column, timestamp)),
                DateRangeType::DateOnly(date) => Ok(format!(
                    "{} >= '{} 00:00:00' AND {} < ('{}')::date + interval '1 day'",
                    column, date, column, date
                )),
                DateRangeType::Range { start, end } => {
                    Ok(format!("{} BETWEEN '{}' AND '{}'", column, start, end))
                }
                DateRangeType::Relative(expr) => Ok(format!("{} {} {}", column, ">", expr)),
            },

            FilterCondition::ArrayContains {
                column,
                operator: _,
                value,
            } => {
                let values = value
                    .split(',')
                    .map(|v| format!("'{}'", v.trim().replace('\'', "''")))
                    .collect::<Vec<_>>()
                    .join(",");
                Ok(format!("{} @> ARRAY[{}]::text[]", column, values))
            }
            FilterCondition::ArrayOverlap {
                column,
                operator: _,
                values,
            } => {
                let formatted_values = values
                    .iter()
                    .map(|v| format!("'{}'", v.replace('\'', "''")))
                    .collect::<Vec<_>>()
                    .join(",");
                Ok(format!("{} && ARRAY[{}]::text[]", column, formatted_values))
            }

            // Never apply case sensitivity to non-text types
            FilterCondition::UuidValue {
                column,
                operator,
                value,
            } => match value {
                Some(v) => Ok(format!(
                    "{} {} '{}'",
                    column,
                    operator.as_sql(),
                    v.replace('\'', "''")
                )),
                None => Ok(format!("{} {}", column, operator.as_sql())),
            },

            FilterCondition::InValues {
                column,
                operator,
                values,
            } => {
                let formatted_values = values
                    .iter()
                    .map(|v| format!("'{}'", v.replace('\'', "''")))
                    .collect::<Vec<_>>()
                    .join(", ");
                Ok(format!(
                    "{} {} ({})",
                    column,
                    operator.as_sql(),
                    formatted_values
                ))
            }

            // Numeric types
            FilterCondition::SmallIntValue {
                column,
                operator,
                value,
            } => Ok(Self::format_value(column, operator, *value)),
            FilterCondition::IntegerValue {
                column,
                operator,
                value,
            } => Ok(Self::format_value(column, operator, *value)),
            FilterCondition::BigIntValue {
                column,
                operator,
                value,
            } => Ok(Self::format_value(column, operator, *value)),
            FilterCondition::RealValue {
                column,
                operator,
                value,
            } => Ok(Self::format_value(column, operator, *value)),
            FilterCondition::DoublePrecisionValue {
                column,
                operator,
                value,
            } => Ok(Self::format_value(column, operator, *value)),

            // Date/Time types
            FilterCondition::DateValue {
                column,
                operator,
                value,
            }
            | FilterCondition::TimeValue {
                column,
                operator,
                value,
            }
            | FilterCondition::TimeTzValue {
                column,
                operator,
                value,
            }
            | FilterCondition::TimestampValue {
                column,
                operator,
                value,
            }
            | FilterCondition::TimestampTzValue {
                column,
                operator,
                value,
            }
            | FilterCondition::IntervalValue {
                column,
                operator,
                value,
            } => Ok(Self::format_string_value(
                column,
                operator,
                value.as_deref(),
            )),

            // Boolean values
            FilterCondition::BooleanValue {
                column,
                operator,
                value,
            } => Ok(Self::format_value(column, operator, *value)),

            // Other types
            FilterCondition::JsonValue {
                column,
                operator,
                value,
            }
            | FilterCondition::JsonbValue {
                column,
                operator,
                value,
            }
            | FilterCondition::InetValue {
                column,
                operator,
                value,
            }
            | FilterCondition::CidrValue {
                column,
                operator,
                value,
            }
            | FilterCondition::MacAddrValue {
                column,
                operator,
                value,
            }
            | FilterCondition::MacAddr8Value {
                column,
                operator,
                value,
            }
            | FilterCondition::ByteAValue {
                column,
                operator,
                value,
            }
            | FilterCondition::MoneyValue {
                column,
                operator,
                value,
            }
            | FilterCondition::XmlValue {
                column,
                operator,
                value,
            } => Ok(Self::format_string_value(
                column,
                operator,
                value.as_deref(),
            )),
        }
    }
    pub fn text(column: &str, operator: FilterOperator, value: Option<&str>) -> Self {
        FilterCondition::TextValue {
            column: column.to_string(),
            operator,
            value: value.map(ToString::to_string),
        }
    }

    pub fn varchar(column: &str, operator: FilterOperator, value: Option<&str>) -> Self {
        FilterCondition::VarcharValue {
            column: column.to_string(),
            operator,
            value: value.map(ToString::to_string),
        }
    }

    pub fn double(column: &str, operator: FilterOperator, value: Option<f64>) -> Self {
        FilterCondition::DoublePrecisionValue {
            column: column.to_string(),
            operator,
            value,
        }
    }

    pub fn integer(column: &str, operator: FilterOperator, value: Option<i32>) -> Self {
        FilterCondition::IntegerValue {
            column: column.to_string(),
            operator,
            value,
        }
    }

    pub fn timestamp(column: &str, operator: FilterOperator, value: Option<&str>) -> Self {
        FilterCondition::TimestampValue {
            column: column.to_string(),
            operator,
            value: value.map(ToString::to_string),
        }
    }

    pub fn boolean(column: &str, operator: FilterOperator, value: Option<bool>) -> Self {
        FilterCondition::BooleanValue {
            column: column.to_string(),
            operator,
            value,
        }
    }

    pub fn in_values(column: &str, operator: FilterOperator, values: Vec<&str>) -> Self {
        FilterCondition::InValues {
            column: column.to_string(),
            operator,
            values: values.into_iter().map(ToString::to_string).collect(),
        }
    }

    pub fn uuid(column: &str, operator: FilterOperator, value: Option<&str>) -> Self {
        FilterCondition::UuidValue {
            column: column.to_string(),
            operator,
            value: value.map(ToString::to_string),
        }
    }

    pub fn date_exact(column: &str, timestamp: &str) -> Self {
        FilterCondition::DateRange {
            column: column.to_string(),
            range_type: DateRangeType::Exact(timestamp.to_string()),
        }
    }

    pub fn date_only(column: &str, date: &str) -> Self {
        FilterCondition::DateRange {
            column: column.to_string(),
            range_type: DateRangeType::DateOnly(date.to_string()),
        }
    }

    pub fn date_range(column: &str, start: &str, end: &str) -> Self {
        FilterCondition::DateRange {
            column: column.to_string(),
            range_type: DateRangeType::Range {
                start: start.to_string(),
                end: end.to_string(),
            },
        }
    }

    pub fn relative_date(column: &str, expr: &str) -> Self {
        FilterCondition::DateRange {
            column: column.to_string(),
            range_type: DateRangeType::Relative(expr.to_string()),
        }
    }
}

impl fmt::Display for FilterCondition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        match self.to_sql(false) {
            Ok(sql) => write!(f, "{}", sql),
            Err(err) => write!(f, "Error: {}", err),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FilterBuilder {
    pub root: Option<FilterExpression>,
    pub case_insensitive: bool,
}

impl FilterBuilder {
    pub fn new() -> Self {
        Self {
            root: None,
            case_insensitive: false,
        }
    }

    pub fn case_insensitive(mut self, value: bool) -> Self {
        self.case_insensitive = value;
        self
    }

    pub fn add_condition(self, condition: FilterCondition) -> Self {
        self.add_expression(FilterExpression::Condition(condition))
    }

    pub fn add_expression(mut self, expression: FilterExpression) -> Self {
        match &self.root {
            None => {
                self.root = Some(expression);
            }
            Some(existing) => {
                self.root = Some(FilterExpression::Group {
                    operator: LogicalOperator::And,
                    expressions: vec![existing.clone(), expression],
                });
            }
        }
        self
    }

    pub fn group(mut self, operator: LogicalOperator, expressions: Vec<FilterExpression>) -> Self {
        let group = FilterExpression::Group {
            operator,
            expressions,
        };
        match &self.root {
            None => {
                self.root = Some(group);
            }
            Some(existing) => {
                self.root = Some(FilterExpression::Group {
                    operator: LogicalOperator::And,
                    expressions: vec![existing.clone(), group],
                });
            }
        }
        self
    }

    pub fn from_json_filters(
        filters: &[JsonFilter],
        case_insensitive: bool,
        column_defs: &HashMap<&str, ColumnDef>,
    ) -> Result<Self> {
        if filters.is_empty() {
            return Ok(Self::new());
        }

        fn create_condition(
            filter: &JsonFilter,
            column_defs: &HashMap<&str, ColumnDef>,
        ) -> FilterCondition {
            match column_defs.get(filter.n.as_str()) {
                Some(ColumnDef::TextArray(_)) => match filter.f.to_uppercase().as_str() {
                    "CONTAINS" => FilterCondition::ArrayContains {
                        column: filter.n.clone(),
                        operator: FilterOperator::Contains,
                        value: filter.v.clone(),
                    },
                    "OVERLAPS" => FilterCondition::ArrayOverlap {
                        column: filter.n.clone(),
                        operator: FilterOperator::Overlaps,
                        values: filter.v.split(',').map(|s| s.trim().to_string()).collect(),
                    },
                    _ => FilterCondition::ArrayContains {
                        column: filter.n.clone(),
                        operator: FilterOperator::Contains,
                        value: filter.v.clone(),
                    },
                },
                Some(ColumnDef::Timestamp(name)) => {
                    // Handle special date filter formats
                    match filter.f.to_uppercase().as_str() {
                        "DATE_ONLY" => FilterCondition::date_only(name, &filter.v),
                        "DATE_RANGE" => {
                            // Expect format: "start,end"
                            let parts: Vec<&str> = filter.v.split(',').collect();
                            if parts.len() == 2 {
                                FilterCondition::date_range(name, parts[0], parts[1])
                            } else {
                                FilterCondition::date_exact(name, &filter.v)
                            }
                        }
                        "RELATIVE" => FilterCondition::relative_date(name, &filter.v),
                        // Default timestamp handling for standard operators
                        _ => FilterCondition::timestamp(
                            name,
                            parse_operator(&filter.f),
                            Some(&filter.v),
                        ),
                    }
                }
                Some(ColumnDef::Uuid(_)) => {
                    FilterCondition::uuid(&filter.n, parse_operator(&filter.f), Some(&filter.v))
                }
                Some(ColumnDef::Integer(_)) => {
                    if let Ok(num) = filter.v.parse::<i32>() {
                        FilterCondition::integer(&filter.n, parse_operator(&filter.f), Some(num))
                    } else {
                        FilterCondition::text(&filter.n, parse_operator(&filter.f), Some(&filter.v))
                    }
                }
                Some(ColumnDef::BigInt(_)) => {
                    if let Ok(num) = filter.v.parse::<i64>() {
                        FilterCondition::BigIntValue {
                            column: filter.n.clone(),
                            operator: parse_operator(&filter.f),
                            value: Some(num),
                        }
                    } else {
                        FilterCondition::text(&filter.n, parse_operator(&filter.f), Some(&filter.v))
                    }
                }
                Some(ColumnDef::DoublePrecision(_)) => {
                    if let Ok(num) = filter.v.parse::<f64>() {
                        FilterCondition::double(&filter.n, parse_operator(&filter.f), Some(num))
                    } else {
                        FilterCondition::text(&filter.n, parse_operator(&filter.f), Some(&filter.v))
                    }
                }
                Some(ColumnDef::Boolean(_)) => {
                    if let Ok(bool_val) = filter.v.parse::<bool>() {
                        FilterCondition::boolean(
                            &filter.n,
                            parse_operator(&filter.f),
                            Some(bool_val),
                        )
                    } else {
                        FilterCondition::text(&filter.n, parse_operator(&filter.f), Some(&filter.v))
                    }
                }
                Some(ColumnDef::Text(_)) | Some(ColumnDef::Varchar(_)) => {
                    FilterCondition::text(&filter.n, parse_operator(&filter.f), Some(&filter.v))
                }
                _ => FilterCondition::text(&filter.n, parse_operator(&filter.f), Some(&filter.v)),
            }
        }

        // Always start with the first filter as the base condition
        let first_condition =
            FilterExpression::Condition(create_condition(&filters[0], column_defs));

        // If only one filter, just return it
        if filters.len() == 1 {
            let mut builder = Self::new().case_insensitive(case_insensitive);
            builder.root = Some(first_condition);
            return Ok(builder);
        }

        // Process subsequent filters
        let mut result: Vec<FilterExpression> = vec![first_condition];
        let mut current_or_group: Vec<FilterExpression> = Vec::new();

        for filter in &filters[1..] {
            let condition = FilterExpression::Condition(create_condition(filter, column_defs));

            match filter.c.as_deref() {
                Some("OR") => {
                    // Start or continue OR group
                    if current_or_group.is_empty() {
                        // First OR condition, include the last result
                        current_or_group.push(result.pop().unwrap());
                    }
                    current_or_group.push(condition);
                }
                _ => {
                    // Handle any accumulated OR group
                    if !current_or_group.is_empty() {
                        current_or_group.push(condition);
                        result.push(FilterExpression::Group {
                            operator: LogicalOperator::Or,
                            expressions: current_or_group,
                        });
                        current_or_group = Vec::new();
                    } else {
                        // Regular AND condition
                        result.push(condition);
                    }
                }
            }
        }

        // Handle any remaining OR group
        if !current_or_group.is_empty() {
            result.push(FilterExpression::Group {
                operator: LogicalOperator::Or,
                expressions: current_or_group,
            });
        }

        let mut builder = Self::new().case_insensitive(case_insensitive);

        if result.len() == 1 {
            builder.root = result.pop();
        } else {
            builder.root = Some(FilterExpression::Group {
                operator: LogicalOperator::And,
                expressions: result,
            });
        }

        Ok(builder)
    }

    pub fn build(&self) -> Result<String> {
        match &self.root {
            None => Ok(String::new()),
            Some(expression) => {
                let sql = expression.to_sql(self.case_insensitive)?;
                if sql.is_empty() {
                    Ok(String::new())
                } else {
                    Ok(format!(" WHERE {}", sql))
                }
            }
        }
    }
}

fn parse_operator(op: &str) -> FilterOperator {
    match op.to_uppercase().as_str() {
        "LIKE" => FilterOperator::Like,
        "=" => FilterOperator::Equal,
        "!=" => FilterOperator::NotEqual,
        ">" => FilterOperator::GreaterThan,
        ">=" => FilterOperator::GreaterThanOrEqual,
        "<" => FilterOperator::LessThan,
        "<=" => FilterOperator::LessThanOrEqual,
        "IN" => FilterOperator::In,
        "NOT IN" => FilterOperator::NotIn,
        "IS NULL" => FilterOperator::IsNull,
        "IS NOT NULL" => FilterOperator::IsNotNull,
        "STARTS WITH" => FilterOperator::StartsWith,
        "ENDS WITH" => FilterOperator::EndsWith,
        "CONTAINS" => FilterOperator::Contains,
        "OVERLAPS" => FilterOperator::Overlaps,
        "DATE_ONLY" => FilterOperator::DateEqual,
        "DATE_RANGE" => FilterOperator::DateRange,
        "RELATIVE" => FilterOperator::RelativeDate,
        _ => FilterOperator::Equal,
    }
}

impl Default for FilterBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_test_columns() -> HashMap<&'static str, ColumnDef> {
        let mut columns = HashMap::new();
        columns.insert("id", ColumnDef::Uuid("id"));
        columns.insert(
            "property_full_address",
            ColumnDef::Text("property_full_address"),
        );
        columns.insert("client_name", ColumnDef::Text("client_name"));
        columns.insert("name", ColumnDef::Text("name"));
        columns.insert("email", ColumnDef::Text("email"));
        columns.insert("age", ColumnDef::Integer("age"));
        columns.insert("salary", ColumnDef::Integer("salary"));
        columns.insert("status", ColumnDef::Text("status"));
        columns.insert("city", ColumnDef::Text("city"));
        columns.insert("department", ColumnDef::Text("department"));
        columns.insert("created_at", ColumnDef::Timestamp("created_at"));
        columns.insert("is_active", ColumnDef::Boolean("is_active"));
        columns
    }

    #[test]
    fn test_or_conditions() -> Result<()> {
        let columns = setup_test_columns();
        let filters = vec![
            JsonFilter {
                n: "property_full_address".to_string(),
                f: "LIKE".to_string(),
                v: "%James%".to_string(),
                c: None,
            },
            JsonFilter {
                n: "client_name".to_string(),
                f: "LIKE".to_string(),
                v: "%James%".to_string(),
                c: Some("OR".to_string()),
            },
        ];

        let sql = FilterBuilder::from_json_filters(&filters, true, &columns)?.build()?;
        assert_eq!(
            sql,
            " WHERE (LOWER(property_full_address) LIKE LOWER('%James%') OR LOWER(client_name) LIKE LOWER('%James%'))"
        );
        Ok(())
    }

    #[test]
    fn test_multiple_or_conditions() -> Result<()> {
        let columns = setup_test_columns();
        let filters = vec![
            JsonFilter {
                n: "name".to_string(),
                f: "=".to_string(),
                v: "value1".to_string(),
                c: None,
            },
            JsonFilter {
                n: "email".to_string(),
                f: "=".to_string(),
                v: "value2".to_string(),
                c: Some("OR".to_string()),
            },
            JsonFilter {
                n: "client_name".to_string(),
                f: "=".to_string(),
                v: "value3".to_string(),
                c: Some("OR".to_string()),
            },
        ];

        let sql = FilterBuilder::from_json_filters(&filters, true, &columns)?.build()?;
        assert_eq!(
            sql,
            " WHERE (LOWER(name) = LOWER('value1') OR LOWER(email) = LOWER('value2') OR LOWER(client_name) = LOWER('value3'))"
        );
        Ok(())
    }

    #[test]
    fn test_mixed_and_or_conditions() -> Result<()> {
        let columns = setup_test_columns();
        let filters = vec![
            JsonFilter {
                n: "name".to_string(),
                f: "LIKE".to_string(),
                v: "%John%".to_string(),
                c: None,
            },
            JsonFilter {
                n: "age".to_string(),
                f: ">".to_string(),
                v: "18".to_string(),
                c: Some("AND".to_string()),
            },
            JsonFilter {
                n: "city".to_string(),
                f: "LIKE".to_string(),
                v: "%York%".to_string(),
                c: Some("OR".to_string()),
            },
        ];

        let sql = FilterBuilder::from_json_filters(&filters, true, &columns)?.build()?;
        assert_eq!(
            sql,
            " WHERE (LOWER(name) LIKE LOWER('%John%') AND (age > 18 OR LOWER(city) LIKE LOWER('%York%')))"
        );
        Ok(())
    }

    #[test]
    fn test_complex_and_or_pattern() -> Result<()> {
        let columns = setup_test_columns();
        let filters = vec![
            JsonFilter {
                n: "status".to_string(),
                f: "=".to_string(),
                v: "active".to_string(),
                c: None,
            },
            JsonFilter {
                n: "age".to_string(),
                f: ">".to_string(),
                v: "21".to_string(),
                c: Some("AND".to_string()),
            },
            JsonFilter {
                n: "city".to_string(),
                f: "=".to_string(),
                v: "New York".to_string(),
                c: Some("OR".to_string()),
            },
            JsonFilter {
                n: "city".to_string(),
                f: "=".to_string(),
                v: "London".to_string(),
                c: Some("OR".to_string()),
            },
            JsonFilter {
                n: "department".to_string(),
                f: "=".to_string(),
                v: "Sales".to_string(),
                c: Some("AND".to_string()),
            },
        ];

        let sql = FilterBuilder::from_json_filters(&filters, true, &columns)?.build()?;
        assert_eq!(
            sql,
            " WHERE (LOWER(status) = LOWER('active') AND (age > 21 OR LOWER(city) = LOWER('New York') OR LOWER(city) = LOWER('London') OR LOWER(department) = LOWER('Sales')))"
        );
        Ok(())
    }

    #[test]
    fn test_case_sensitivity() -> Result<()> {
        let columns = setup_test_columns();
        let filters = vec![
            JsonFilter {
                n: "name".to_string(),
                f: "LIKE".to_string(),
                v: "%John%".to_string(),
                c: None,
            },
            JsonFilter {
                n: "email".to_string(),
                f: "LIKE".to_string(),
                v: "%gmail.com".to_string(),
                c: Some("OR".to_string()),
            },
        ];

        // Test case sensitive
        let sql_sensitive = FilterBuilder::from_json_filters(&filters, false, &columns)?.build()?;
        assert_eq!(
            sql_sensitive,
            " WHERE (name LIKE '%John%' OR email LIKE '%gmail.com')"
        );

        // Test case insensitive
        let sql_insensitive =
            FilterBuilder::from_json_filters(&filters, true, &columns)?.build()?;
        assert_eq!(
            sql_insensitive,
            " WHERE (LOWER(name) LIKE LOWER('%John%') OR LOWER(email) LIKE LOWER('%gmail.com'))"
        );
        Ok(())
    }

    #[test]
    fn test_numeric_conditions() -> Result<()> {
        let columns = setup_test_columns();
        let filters = vec![
            JsonFilter {
                n: "age".to_string(),
                f: ">".to_string(),
                v: "25".to_string(),
                c: None,
            },
            JsonFilter {
                n: "salary".to_string(),
                f: "<".to_string(),
                v: "50000".to_string(),
                c: Some("OR".to_string()),
            },
        ];

        let sql = FilterBuilder::from_json_filters(&filters, false, &columns)?.build()?;
        assert_eq!(sql, " WHERE (age > 25 OR salary < 50000)");
        Ok(())
    }

    #[test]
    fn test_single_filter() -> Result<()> {
        let columns = setup_test_columns();
        let filters = vec![JsonFilter {
            n: "name".to_string(),
            f: "LIKE".to_string(),
            v: "%John%".to_string(),
            c: None,
        }];

        let sql = FilterBuilder::from_json_filters(&filters, true, &columns)?.build()?;
        assert_eq!(sql, " WHERE LOWER(name) LIKE LOWER('%John%')");
        Ok(())
    }

    #[test]
    fn test_empty_filters() -> Result<()> {
        let columns = setup_test_columns();
        let filters: Vec<JsonFilter> = vec![];
        let sql = FilterBuilder::from_json_filters(&filters, true, &columns)?.build()?;
        assert_eq!(sql, "");
        Ok(())
    }

    // New tests for UUID and mixed type handling
    #[test]
    fn test_uuid_conditions() -> Result<()> {
        let columns = setup_test_columns();
        let filters = vec![JsonFilter {
            n: "id".to_string(),
            f: "=".to_string(),
            v: "123e4567-e89b-12d3-a456-426614174000".to_string(),
            c: None,
        }];

        // Case sensitivity should be ignored for UUID
        let sql = FilterBuilder::from_json_filters(&filters, true, &columns)?.build()?;
        assert_eq!(sql, " WHERE id = '123e4567-e89b-12d3-a456-426614174000'");
        Ok(())
    }

    #[test]
    fn test_mixed_type_conditions() -> Result<()> {
        let columns = setup_test_columns();
        let filters = vec![
            JsonFilter {
                n: "id".to_string(),
                f: "=".to_string(),
                v: "123e4567-e89b-12d3-a456-426614174000".to_string(),
                c: None,
            },
            JsonFilter {
                n: "name".to_string(),
                f: "LIKE".to_string(),
                v: "%John%".to_string(),
                c: Some("AND".to_string()),
            },
            JsonFilter {
                n: "age".to_string(),
                f: ">".to_string(),
                v: "25".to_string(),
                c: Some("AND".to_string()),
            },
        ];

        let sql = FilterBuilder::from_json_filters(&filters, true, &columns)?.build()?;
        assert_eq!(
            sql,
            " WHERE (id = '123e4567-e89b-12d3-a456-426614174000' AND LOWER(name) LIKE LOWER('%John%') AND age > 25)"
        );
        Ok(())
    }

    #[test]
    fn test_boolean_conditions() -> Result<()> {
        let columns = setup_test_columns();
        let filters = vec![JsonFilter {
            n: "is_active".to_string(),
            f: "=".to_string(),
            v: "true".to_string(),
            c: None,
        }];

        let sql = FilterBuilder::from_json_filters(&filters, true, &columns)?.build()?;
        assert_eq!(sql, " WHERE is_active = true");
        Ok(())
    }

    #[test]
    fn test_timestamp_conditions() -> Result<()> {
        let columns = setup_test_columns();
        let filters = vec![JsonFilter {
            n: "created_at".to_string(),
            f: ">".to_string(),
            v: "2024-01-01 00:00:00".to_string(),
            c: None,
        }];

        let sql = FilterBuilder::from_json_filters(&filters, true, &columns)?.build()?;
        assert_eq!(sql, " WHERE created_at > '2024-01-01 00:00:00'");
        Ok(())
    }

    #[test]
    fn test_unknown_column_type() -> Result<()> {
        let columns = setup_test_columns();
        let filters = vec![JsonFilter {
            n: "unknown_column".to_string(),
            f: "=".to_string(),
            v: "test".to_string(),
            c: None,
        }];

        // Should default to text handling for unknown columns
        let sql = FilterBuilder::from_json_filters(&filters, true, &columns)?.build()?;
        assert_eq!(sql, " WHERE LOWER(unknown_column) = LOWER('test')");
        Ok(())
    }

    #[test]
    fn test_text_array_contains() -> Result<()> {
        let mut columns = setup_test_columns();
        columns.insert("services", ColumnDef::TextArray("services"));

        // Test single value contains
        let filters = vec![JsonFilter {
            n: "services".to_string(),
            f: "CONTAINS".to_string(),
            v: "EPC".to_string(),
            c: None,
        }];

        let sql = FilterBuilder::from_json_filters(&filters, true, &columns)?.build()?;
        assert_eq!(sql, " WHERE services @> ARRAY['EPC']::text[]");

        // Test multiple values contains
        let filters = vec![JsonFilter {
            n: "services".to_string(),
            f: "CONTAINS".to_string(),
            v: "EPC,Search".to_string(),
            c: None,
        }];

        let sql = FilterBuilder::from_json_filters(&filters, true, &columns)?.build()?;
        assert_eq!(sql, " WHERE services @> ARRAY['EPC','Search']::text[]");

        Ok(())
    }

    #[test]
    fn test_text_array_overlaps() -> Result<()> {
        let mut columns = setup_test_columns();
        columns.insert("services", ColumnDef::TextArray("services"));

        // Test single value overlaps
        let filters = vec![JsonFilter {
            n: "services".to_string(),
            f: "OVERLAPS".to_string(),
            v: "EPC".to_string(),
            c: None,
        }];

        let sql = FilterBuilder::from_json_filters(&filters, true, &columns)?.build()?;
        assert_eq!(sql, " WHERE services && ARRAY['EPC']::text[]");

        // Test multiple values overlaps
        let filters = vec![JsonFilter {
            n: "services".to_string(),
            f: "OVERLAPS".to_string(),
            v: "EPC,Search".to_string(),
            c: None,
        }];

        let sql = FilterBuilder::from_json_filters(&filters, true, &columns)?.build()?;
        assert_eq!(sql, " WHERE services && ARRAY['EPC','Search']::text[]");

        Ok(())
    }

    #[test]
    fn test_text_array_with_complex_conditions() -> Result<()> {
        let mut columns = setup_test_columns();
        columns.insert("services", ColumnDef::TextArray("services"));

        let filters = vec![
            JsonFilter {
                n: "services".to_string(),
                f: "CONTAINS".to_string(),
                v: "EPC".to_string(),
                c: None,
            },
            JsonFilter {
                n: "status".to_string(),
                f: "=".to_string(),
                v: "active".to_string(),
                c: Some("AND".to_string()),
            },
            JsonFilter {
                n: "services".to_string(),
                f: "OVERLAPS".to_string(),
                v: "Search,Valuation".to_string(),
                c: Some("OR".to_string()),
            },
        ];

        let sql = FilterBuilder::from_json_filters(&filters, true, &columns)?.build()?;
        assert_eq!(
            sql,
            " WHERE (services @> ARRAY['EPC']::text[] AND (LOWER(status) = LOWER('active') OR services && ARRAY['Search','Valuation']::text[]))"
        );

        Ok(())
    }

    #[test]
    fn test_text_array_with_special_characters() -> Result<()> {
        let mut columns = setup_test_columns();
        columns.insert("services", ColumnDef::TextArray("services"));

        let filters = vec![JsonFilter {
            n: "services".to_string(),
            f: "CONTAINS".to_string(),
            v: "EPC's,Search & Valuation".to_string(),
            c: None,
        }];

        let sql = FilterBuilder::from_json_filters(&filters, true, &columns)?.build()?;
        assert_eq!(
            sql,
            " WHERE services @> ARRAY['EPC''s','Search & Valuation']::text[]"
        );

        Ok(())
    }

    #[test]
    fn test_text_array_empty_value() -> Result<()> {
        let mut columns = setup_test_columns();
        columns.insert("services", ColumnDef::TextArray("services"));

        let filters = vec![JsonFilter {
            n: "services".to_string(),
            f: "CONTAINS".to_string(),
            v: "".to_string(),
            c: None,
        }];

        let sql = FilterBuilder::from_json_filters(&filters, true, &columns)?.build()?;
        assert_eq!(sql, " WHERE services @> ARRAY['']::text[]");

        Ok(())
    }

    #[test]
    fn test_date_exact_filter() -> Result<()> {
        let mut columns = setup_test_columns();
        columns.insert("created_at", ColumnDef::Timestamp("created_at"));

        let filters = vec![JsonFilter {
            n: "created_at".to_string(),
            f: "=".to_string(),
            v: "2024-12-29 15:30:00".to_string(),
            c: None,
        }];

        let sql = FilterBuilder::from_json_filters(&filters, true, &columns)?.build()?;
        assert_eq!(sql, " WHERE created_at = '2024-12-29 15:30:00'");
        Ok(())
    }

    #[test]
    fn test_date_only_filter() -> Result<()> {
        let mut columns = setup_test_columns();
        columns.insert("created_at", ColumnDef::Timestamp("created_at"));

        let filters = vec![JsonFilter {
            n: "created_at".to_string(),
            f: "DATE_ONLY".to_string(),
            v: "2024-12-29".to_string(),
            c: None,
        }];

        let sql = FilterBuilder::from_json_filters(&filters, true, &columns)?.build()?;
        assert_eq!(
            sql,
            " WHERE created_at >= '2024-12-29 00:00:00' AND created_at < ('2024-12-29')::date + interval '1 day'"
        );
        Ok(())
    }

    #[test]
    fn test_date_range_filter() -> Result<()> {
        let mut columns = setup_test_columns();
        columns.insert("created_at", ColumnDef::Timestamp("created_at"));

        let filters = vec![JsonFilter {
            n: "created_at".to_string(),
            f: "DATE_RANGE".to_string(),
            v: "2024-12-29 00:00:00,2024-12-29 23:59:59".to_string(),
            c: None,
        }];

        let sql = FilterBuilder::from_json_filters(&filters, true, &columns)?.build()?;
        assert_eq!(
            sql,
            " WHERE created_at BETWEEN '2024-12-29 00:00:00' AND '2024-12-29 23:59:59'"
        );
        Ok(())
    }

    #[test]
    fn test_relative_date_filter() -> Result<()> {
        let mut columns = setup_test_columns();
        columns.insert("created_at", ColumnDef::Timestamp("created_at"));

        let filters = vec![JsonFilter {
            n: "created_at".to_string(),
            f: "RELATIVE".to_string(),
            v: "now() - interval '1 day'".to_string(),
            c: None,
        }];

        let sql = FilterBuilder::from_json_filters(&filters, true, &columns)?.build()?;
        assert_eq!(sql, " WHERE created_at > now() - interval '1 day'");
        Ok(())
    }

    #[test]
    fn test_combined_date_filters() -> Result<()> {
        let mut columns = setup_test_columns();
        columns.insert("created_at", ColumnDef::Timestamp("created_at"));
        columns.insert("updated_at", ColumnDef::Timestamp("updated_at"));

        let filters = vec![
            JsonFilter {
                n: "created_at".to_string(),
                f: "DATE_ONLY".to_string(),
                v: "2024-12-29".to_string(),
                c: None,
            },
            JsonFilter {
                n: "updated_at".to_string(),
                f: "RELATIVE".to_string(),
                v: "now() - interval '1 hour'".to_string(),
                c: Some("AND".to_string()),
            },
        ];

        let sql = FilterBuilder::from_json_filters(&filters, true, &columns)?.build()?;
        assert_eq!(
            sql,
            " WHERE (created_at >= '2024-12-29 00:00:00' AND created_at < ('2024-12-29')::date + interval '1 day' AND updated_at > now() - interval '1 hour')"
        );
        Ok(())
    }

    #[test]
    fn test_case_insensitive_operators() -> Result<()> {
        let mut columns = setup_test_columns();

        // Test different case variations of LIKE
        let operators = vec!["LIKE", "like", "Like", "LiKe"];

        for op in operators {
            let filters = vec![JsonFilter {
                n: "name".to_string(),
                f: op.to_string(),
                v: "%John%".to_string(),
                c: None,
            }];

            let sql = FilterBuilder::from_json_filters(&filters, true, &columns)?.build()?;
            assert_eq!(sql, " WHERE LOWER(name) LIKE LOWER('%John%')");
        }

        // Test different case variations of CONTAINS for array
        let operators = vec!["CONTAINS", "contains", "Contains", "CoNtAiNs"];
        columns.insert("services", ColumnDef::TextArray("services"));

        for op in operators {
            let filters = vec![JsonFilter {
                n: "services".to_string(),
                f: op.to_string(),
                v: "EPC".to_string(),
                c: None,
            }];

            let sql = FilterBuilder::from_json_filters(&filters, true, &columns)?.build()?;
            assert_eq!(sql, " WHERE services @> ARRAY['EPC']::text[]");
        }

        Ok(())
    }
}
