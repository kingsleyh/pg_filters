use eyre::Result;
use serde::{Deserialize, Serialize};
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
    pub n: String,  // name/column
    pub f: String,  // filter operator
    pub v: String,  // value
    pub c: String,  // connector (AND/OR)
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

            FilterCondition::BooleanValue {
                column,
                operator,
                value,
            } => Ok(Self::format_value(column, operator, *value)),

            FilterCondition::UuidValue {
                column,
                operator,
                value,
            }
            | FilterCondition::JsonValue {
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

    pub fn from_json_filters(filters: &[JsonFilter], case_insensitive: bool) -> Result<Self> {
        if filters.is_empty() {
            return Ok(Self::new());
        }

        let mut expressions: Vec<FilterExpression> = Vec::new();
        let mut current_group: Vec<FilterExpression> = Vec::new();
        let mut last_connector = "AND";

        for filter in filters {
            let condition = FilterCondition::text(
                &filter.n,
                parse_operator(&filter.f),
                Some(&filter.v),
            );

            if filter.c == "OR" {
                // If this is an OR condition, create or add to OR group
                if last_connector == "OR" {
                    // Add to existing OR group
                    if let Some(FilterExpression::Group { operator: LogicalOperator::Or, expressions: ref mut group_expressions }) = expressions.last_mut() {
                        group_expressions.push(FilterExpression::Condition(condition));
                    }
                } else {
                    // Start new OR group
                    if !current_group.is_empty() {
                        expressions.extend(current_group.drain(..));
                    }
                    expressions.push(FilterExpression::or(vec![
                        expressions.clone().pop().unwrap_or(FilterExpression::Condition(condition.clone())),
                        FilterExpression::Condition(condition),
                    ]));
                }
            } else {
                // Add to AND group
                if last_connector == "OR" {
                    current_group.push(FilterExpression::Condition(condition));
                } else {
                    expressions.push(FilterExpression::Condition(condition));
                }
            }

            last_connector = &filter.c;
        }

        // Add any remaining conditions
        expressions.extend(current_group);

        // Create the final filter builder
        let mut builder = Self::new().case_insensitive(case_insensitive);

        if expressions.len() == 1 {
            builder.root = expressions.pop();
        } else if !expressions.is_empty() {
            builder.root = Some(FilterExpression::Group {
                operator: LogicalOperator::And,
                expressions,
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
    match op {
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

    #[test]
    fn test_or_conditions() -> Result<()> {
        let filters = vec![
            JsonFilter {
                n: "property_full_address".to_string(),
                f: "LIKE".to_string(),
                v: "%James%".to_string(),
                c: "AND".to_string(),
            },
            JsonFilter {
                n: "client_name".to_string(),
                f: "LIKE".to_string(),
                v: "%James%".to_string(),
                c: "OR".to_string(),
            },
        ];

        let sql = FilterBuilder::from_json_filters(&filters, true)?.build()?;
        assert_eq!(
            sql,
            " WHERE (LOWER(property_full_address) LIKE LOWER('%James%') AND (LOWER(property_full_address) LIKE LOWER('%James%') OR LOWER(client_name) LIKE LOWER('%James%')))"
        );
        Ok(())
    }

    #[test]
    fn test_mixed_and_or_conditions() -> Result<()> {
        let filters = vec![
            JsonFilter {
                n: "name".to_string(),
                f: "LIKE".to_string(),
                v: "%John%".to_string(),
                c: "AND".to_string(),
            },
            JsonFilter {
                n: "age".to_string(),
                f: ">".to_string(),
                v: "18".to_string(),
                c: "AND".to_string(),
            },
            JsonFilter {
                n: "city".to_string(),
                f: "LIKE".to_string(),
                v: "%York%".to_string(),
                c: "OR".to_string(),
            },
        ];

        let sql = FilterBuilder::from_json_filters(&filters, true)?.build()?;
        assert_eq!(
            sql,
            " WHERE (LOWER(name) LIKE LOWER('%John%') AND LOWER(age) > LOWER('18') AND (LOWER(age) > LOWER('18') OR LOWER(city) LIKE LOWER('%York%')))"
        );
        Ok(())
    }

    #[test]
    fn test_empty_filters() -> Result<()> {
        let filters: Vec<JsonFilter> = vec![];
        let sql = FilterBuilder::from_json_filters(&filters, true)?.build()?;
        assert_eq!(sql, "");
        Ok(())
    }

    #[test]
    fn test_single_filter() -> Result<()> {
        let filters = vec![JsonFilter {
            n: "name".to_string(),
            f: "LIKE".to_string(),
            v: "%John%".to_string(),
            c: "AND".to_string(),
        }];

        let sql = FilterBuilder::from_json_filters(&filters, true)?.build()?;
        assert_eq!(sql, " WHERE LOWER(name) LIKE LOWER('%John%')");
        Ok(())
    }
}