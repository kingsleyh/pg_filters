use eyre::Result;
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
        let operator_str = match self {
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
        };
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
    DecimalValue {
        column: String,
        operator: FilterOperator,
        value: Option<f64>,
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
    /// Helper function to format SQL conditions
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

    /// Helper function to format string-like values with proper quoting
    fn format_string_value(column: &str, operator: &FilterOperator, value: Option<&str>) -> String {
        match value {
            Some(v) => format!(
                "{} {} '{}'",
                column,
                operator.as_sql(),
                v.replace('\'', "''") // Escape single quotes
            ),
            None => format!("{} {}", column, operator.as_sql()),
        }
    }

    /// SQL formatting for filter conditions
    pub fn to_sql(&self, case_insensitive: bool) -> Result<String> {
        match self {
            // Character Types
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

            // Multi-value conditions
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

            // Numeric Types
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
            FilterCondition::DecimalValue {
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

            // Date/Time Types
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

            // Boolean Type
            FilterCondition::BooleanValue {
                column,
                operator,
                value,
            } => Ok(Self::format_value(column, operator, *value)),

            // UUID and String-Like Types
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

impl Default for FilterBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_filter() -> Result<()> {
        let filter = FilterBuilder::new()
            .case_insensitive(true)
            .add_condition(FilterCondition::TextValue {
                column: "name".to_string(),
                operator: FilterOperator::Equal,
                value: Some("John".to_string()),
            })
            .build()?;

        assert_eq!(filter, " WHERE LOWER(name) = LOWER('John')");
        Ok(())
    }

    #[test]
    fn test_multiple_and_conditions() -> Result<()> {
        let filter = FilterBuilder::new()
            .case_insensitive(true)
            .add_condition(FilterCondition::TextValue {
                column: "name".to_string(),
                operator: FilterOperator::Equal,
                value: Some("John".to_string()),
            })
            .add_condition(FilterCondition::IntegerValue {
                column: "age".to_string(),
                operator: FilterOperator::GreaterThan,
                value: Some(18),
            })
            .build()?;

        assert_eq!(filter, " WHERE (LOWER(name) = LOWER('John') AND age > 18)");
        Ok(())
    }

    #[test]
    fn test_complex_conditions() -> Result<()> {
        let name_condition = FilterCondition::TextValue {
            column: "name".to_string(),
            operator: FilterOperator::Equal,
            value: Some("John".to_string()),
        };

        let age_condition = FilterCondition::IntegerValue {
            column: "age".to_string(),
            operator: FilterOperator::GreaterThan,
            value: Some(18),
        };

        let city_condition = FilterCondition::InValues {
            column: "city".to_string(),
            operator: FilterOperator::In,
            values: vec!["New York".to_string(), "London".to_string()],
        };

        // Create a filter: (name = 'John' AND age > 18) OR city IN ('New York', 'London')
        let filter = FilterBuilder::new()
            .case_insensitive(true)
            .group(
                LogicalOperator::Or,
                vec![
                    FilterExpression::Group {
                        operator: LogicalOperator::And,
                        expressions: vec![
                            FilterExpression::Condition(name_condition),
                            FilterExpression::Condition(age_condition),
                        ],
                    },
                    FilterExpression::Condition(city_condition),
                ],
            )
            .build()?;

        assert_eq!(
            filter,
            " WHERE ((LOWER(name) = LOWER('John') AND age > 18) OR city IN ('New York', 'London'))"
        );
        Ok(())
    }

    #[test]
    fn test_null_conditions() -> Result<()> {
        let filter = FilterBuilder::new()
            .add_condition(FilterCondition::TextValue {
                column: "name".to_string(),
                operator: FilterOperator::IsNull,
                value: None,
            })
            .build()?;

        assert_eq!(filter, " WHERE name IS NULL");
        Ok(())
    }

    #[test]
    fn test_like_conditions() -> Result<()> {
        let filter = FilterBuilder::new()
            .case_insensitive(true)
            .add_condition(FilterCondition::TextValue {
                column: "name".to_string(),
                operator: FilterOperator::StartsWith,
                value: Some("Jo".to_string()),
            })
            .build()?;

        assert_eq!(filter, " WHERE LOWER(name) LIKE LOWER('Jo%')");
        Ok(())
    }

    #[test]
    fn test_empty_filter() -> Result<()> {
        let filter = FilterBuilder::new().build()?;
        assert_eq!(filter, "");
        Ok(())
    }

    #[test]
    fn test_nested_groups() -> Result<()> {
        let filter = FilterBuilder::new()
            .case_insensitive(true)
            .group(
                LogicalOperator::Or,
                vec![
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
            )
            .build()?;

        assert_eq!(
            filter,
            " WHERE ((LOWER(name) = LOWER('John') AND age > 18) OR (LOWER(name) = LOWER('Jane') AND age < 25))"
        );
        Ok(())
    }
}
