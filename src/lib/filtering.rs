use std::fmt::Debug;
use eyre::Result;

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
            FilterOperator::StartsWith => format!("{}%", value), // Append `%` for "starts with"
            FilterOperator::EndsWith => format!("%{}", value),   // Prepend `%` for "ends with"
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

#[derive(Debug, Clone)]
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
        values: Vec<String>, // Use String to represent values generically
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
    pub fn to_sql(&self, case_insensitive: bool) -> Result<String> {
        match self {
            // Character Types
            FilterCondition::TextValue { column, operator, value }
            | FilterCondition::VarcharValue { column, operator, value }
            | FilterCondition::CharValue { column, operator, value } => match value {
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

            // SmallInt Type
            FilterCondition::SmallIntValue { column, operator, value } => match value {
                Some(v) => Ok(format!("{} {} {}", column, operator.as_sql(), v)),
                None => Ok(format!("{} {}", column, operator.as_sql())),
            },

            // Integer Type
            FilterCondition::IntegerValue { column, operator, value } => match value {
                Some(v) => {
                    Ok(format!("{} {} {}", column, operator.as_sql(), v))
                }
                None => Ok(format!("{} {}", column, operator.as_sql())),
            },

            // IN/NOT IN conditions
            FilterCondition::InValues { column, operator, values } => {
                let formatted_values = operator.format_values(values);
                Ok(format!("{} {} ({})", column, operator.as_sql(), formatted_values))
            }

            // BigInt Type
            FilterCondition::BigIntValue { column, operator, value } => match value {
                Some(v) => Ok(format!("{} {} {}", column, operator.as_sql(), v)),
                None => Ok(format!("{} {}", column, operator.as_sql())),
            },

            // Real Type
            FilterCondition::RealValue { column, operator, value } => match value {
                Some(v) => Ok(format!("{} {} {}", column, operator.as_sql(), v)),
                None => Ok(format!("{} {}", column, operator.as_sql())),
            },

            // DoublePrecision Type
            FilterCondition::DoublePrecisionValue { column, operator, value } => match value {
                Some(v) => Ok(format!("{} {} {}", column, operator.as_sql(), v)),
                None => Ok(format!("{} {}", column, operator.as_sql())),
            },

            // Decimal Type
            FilterCondition::DecimalValue { column, operator, value } => match value {
                Some(v) => Ok(format!("{} {} {}", column, operator.as_sql(), v)),
                None => Ok(format!("{} {}", column, operator.as_sql())),
            },

            // Date/Time Types
            FilterCondition::DateValue { column, operator, value }
            | FilterCondition::TimeValue { column, operator, value }
            | FilterCondition::TimeTzValue { column, operator, value }
            | FilterCondition::TimestampValue { column, operator, value }
            | FilterCondition::TimestampTzValue { column, operator, value }
            | FilterCondition::IntervalValue { column, operator, value } => match value {
                Some(v) => Ok(format!("{} {} '{}'", column, operator.as_sql(), v)),
                None => Ok(format!("{} {}", column, operator.as_sql())),
            },

            // Boolean Type
            FilterCondition::BooleanValue { column, operator, value } => match value {
                Some(v) => Ok(format!("{} {} {}", column, operator.as_sql(), v)),
                None => Ok(format!("{} {}", column, operator.as_sql())),
            },

            // UUID Type
            FilterCondition::UuidValue { column, operator, value } => match value {
                Some(v) => Ok(format!("{} {} '{}'", column, operator.as_sql(), v)),
                None => Ok(format!("{} {}", column, operator.as_sql())),
            },

            // JSON Types
            FilterCondition::JsonValue { column, operator, value }
            | FilterCondition::JsonbValue { column, operator, value } => match value {
                Some(v) => Ok(format!("{} {} '{}'", column, operator.as_sql(), v)),
                None => Ok(format!("{} {}", column, operator.as_sql())),
            },

            // Network Address Types
            FilterCondition::InetValue { column, operator, value }
            | FilterCondition::CidrValue { column, operator, value }
            | FilterCondition::MacAddrValue { column, operator, value }
            | FilterCondition::MacAddr8Value { column, operator, value } => match value {
                Some(v) => Ok(format!("{} {} '{}'", column, operator.as_sql(), v)),
                None => Ok(format!("{} {}", column, operator.as_sql())),
            },

            // Binary Data
            FilterCondition::ByteAValue { column, operator, value } => match value {
                Some(v) => Ok(format!("{} {} '{}'", column, operator.as_sql(), v)),
                None => Ok(format!("{} {}", column, operator.as_sql())),
            },

            // Money Type
            FilterCondition::MoneyValue { column, operator, value } => match value {
                Some(v) => Ok(format!("{} {} '{}'", column, operator.as_sql(), v)),
                None => Ok(format!("{} {}", column, operator.as_sql())),
            },

            // XML
            FilterCondition::XmlValue { column, operator, value } => match value {
                Some(v) => Ok(format!("{} {} '{}'", column, operator.as_sql(), v)),
                None => Ok(format!("{} {}", column, operator.as_sql())),
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct FilterBuilder {
    pub conditions: Vec<FilterCondition>,
    pub case_insensitive: bool,
}

impl FilterBuilder {
    pub fn new() -> Self {
        Self {
            conditions: Vec::new(),
            case_insensitive: false,
        }
    }

    pub fn case_insensitive(mut self, value: bool) -> Self {
        self.case_insensitive = value;
        self
    }

    pub fn add_condition(mut self, condition: FilterCondition) -> Self {
        self.conditions.push(condition);
        self
    }

    pub fn build(&self) -> Result<String> {
        if self.conditions.is_empty() {
            return Ok(String::new());
        }

        let mut sql = String::from(" WHERE ");
        let mut first = true;

        for condition in &self.conditions {
            if !first {
                sql.push_str(" AND ");
            }
            sql.push_str(&condition.to_sql(self.case_insensitive)?);
            first = false;
        }

        Ok(sql)
    }
}