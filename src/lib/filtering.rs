use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq)]
pub enum FilterValue {
    String(String),
    Int(i64),
    Float(f64),
    Bool(bool),
    StringList(Vec<String>),
    IntList(Vec<i64>),
    FloatList(Vec<f64>),
    BoolList(Vec<bool>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ColumnName {
    String(&'static str),
    Int(&'static str),
    Float(&'static str),
    Bool(&'static str),
}

#[derive(Debug, Clone, PartialEq)]
pub enum FilterColumn {
    String(&'static str, String),
    Int(&'static str, i64),
    Float(&'static str, f64),
    Bool(&'static str, bool),
    StringList(&'static str, Vec<String>),
    IntList(&'static str, Vec<i64>),
    FloatList(&'static str, Vec<f64>),
    BoolList(&'static str, Vec<bool>),
}

impl Display for FilterColumn {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            FilterColumn::String(c, v) => write!(f, "{}: {}", c, v),
            FilterColumn::Int(c, v) => write!(f, "{}: {}", c, v),
            FilterColumn::Float(c, v) => write!(f, "{}: {}", c, v),
            FilterColumn::Bool(c, v) => write!(f, "{}: {}", c, v),
            FilterColumn::StringList(c, v) => write!(f, "{}: {:?}", c, v),
            FilterColumn::IntList(c, v) => write!(f, "{}: {:?}", c, v),
            FilterColumn::FloatList(c, v) => write!(f, "{}: {:?}", c, v),
            FilterColumn::BoolList(c, v) => write!(f, "{}: {:?}", c, v),
        }
    }
}

impl Display for FilterValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            FilterValue::String(v) => write!(f, "{}", v),
            FilterValue::Int(v) => write!(f, "{}", v),
            FilterValue::Float(v) => write!(f, "{}", v),
            FilterValue::Bool(v) => write!(f, "{}", v),
            FilterValue::StringList(v) => write!(f, "{:?}", v),
            FilterValue::IntList(v) => write!(f, "{:?}", v),
            FilterValue::FloatList(v) => write!(f, "{:?}", v),
            FilterValue::BoolList(v) => write!(f, "{:?}", v),
        }
    }
}

#[derive(Debug, Clone)]
pub struct FilteringRule {
    /// The name of the column to filter and the value to filter by
    pub filter_column: FilterColumn,
    /// The operator to use for filtering
    pub filter_operator: FilterOperator,
    /// The operator to use to combine this rule with the next rule
    pub conditional_operator: ConditionalOperator,
}

impl<'a> From<&'a FilteringRule> for FilteringRule {
    fn from(rule: &'a FilteringRule) -> Self {
        FilteringRule {
            filter_column: rule.filter_column.clone(),
            filter_operator: rule.filter_operator.clone(),
            conditional_operator: rule.conditional_operator.clone(),
        }
    }
}

impl FilteringRule {
    pub fn new(
        conditional_operator: &str,
        column: ColumnName,
        filter_operator: &str,
        value: &str,
    ) -> eyre::Result<FilteringRule> {
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
            "STARTS WITH" => FilterOperator::StartsWith,
            "ENDS WITH" => FilterOperator::EndsWith,
            _ => FilterOperator::Equal,
        };

        let conditional_operator = match conditional_operator.to_uppercase().as_str() {
            "AND" => ConditionalOperator::And,
            "OR" => ConditionalOperator::Or,
            _ => ConditionalOperator::And,
        };

        let value = match filter_operator {
            FilterOperator::In | FilterOperator::NotIn => {
                let values = value
                    .split(',')
                    .map(|v| v.trim().to_string())
                    .collect::<Vec<String>>();
                match column {
                    ColumnName::String(_c) => FilterValue::StringList(values),
                    ColumnName::Int(c) => {
                        let values = values
                            .into_iter()
                            .flat_map(|v| v.parse::<i64>().ok())
                            .collect::<Vec<i64>>();
                        if values.is_empty() {
                            return Err(eyre::eyre!(
                                "No valid Int values found for IN/NOT IN filter for column '{}'",
                                c
                            ));
                        }
                        FilterValue::IntList(values)
                    }
                    ColumnName::Float(c) => {
                        let values = values
                            .into_iter()
                            .flat_map(|v| v.parse::<f64>().ok())
                            .collect::<Vec<f64>>();
                        if values.is_empty() {
                            return Err(eyre::eyre!(
                                "No valid Float values found for IN/NOT IN filter for column: '{}'",
                                c
                            ));
                        }
                        FilterValue::FloatList(values)
                    }
                    ColumnName::Bool(c) => {
                        let values = values
                            .into_iter()
                            .flat_map(|v| v.parse::<bool>().ok())
                            .collect::<Vec<bool>>();
                        if values.is_empty() {
                            return Err(eyre::eyre!(
                                "No valid Bool values found for IN/NOT IN filter for column: '{}'",
                                c
                            ));
                        }
                        FilterValue::BoolList(values)
                    }
                }
            }
            FilterOperator::IsNull | FilterOperator::IsNotNull => {
                FilterValue::String("".to_string())
            }
            _ => match column {
                ColumnName::String(_c) => FilterValue::String(value.into()),
                ColumnName::Int(c) => {
                    let value = value.parse::<i64>().map_err(|_| {
                        eyre::eyre!("Invalid value: '{}' for column: '{}' of type Int", value, c)
                    })?;
                    FilterValue::Int(value)
                }
                ColumnName::Float(c) => {
                    let value = value.parse::<f64>().map_err(|_| {
                        eyre::eyre!(
                            "Invalid value: '{}' for column: '{}' of type Float",
                            value,
                            c
                        )
                    })?;
                    FilterValue::Float(value)
                }
                ColumnName::Bool(c) => {
                    let value = value.parse::<bool>().map_err(|_| {
                        eyre::eyre!(
                            "Invalid value: '{}' for column: '{}' of type Bool",
                            value,
                            c
                        )
                    })?;
                    FilterValue::Bool(value)
                }
            },
        };

        let filter_column =
            Filtering::filter_column(column.clone(), filter_operator.clone(), value.clone())?;

        Ok(FilteringRule {
            filter_column,
            filter_operator,
            conditional_operator,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ConditionalOperator {
    And,
    Or,
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

#[derive(Debug, Clone)]
pub struct Filtering {
    pub filters: Vec<FilteringRule>,
    pub case_insensitive: bool,
    pub sql: String,
}

impl Filtering {
    fn filter_column(
        column: ColumnName,
        operator: FilterOperator,
        value: FilterValue,
    ) -> eyre::Result<FilterColumn> {
        match column {
            ColumnName::String(c) => {
                if let Some(value) = Self::handle_null(operator.clone(), c) {
                    return value;
                }
                match value {
                    FilterValue::String(v) => {
                        if let Some(value) = Self::handle_like(operator.clone(), c, v.clone()) {
                            return value;
                        }
                        if let Some(value) =
                            Self::handle_starts_with(operator.clone(), c, v.clone())
                        {
                            return value;
                        }
                        if let Some(value) = Self::handle_ends_with(operator.clone(), c, v.clone())
                        {
                            return value;
                        }
                        Ok(FilterColumn::String(c, format!("'{}'", v)))
                    }
                    FilterValue::StringList(v) => {
                        let v = v
                            .iter()
                            .map(|v| format!("'{}'", v))
                            .collect::<Vec<String>>();
                        Ok(FilterColumn::StringList(c, v))
                    }
                    _ => Err(eyre::eyre!(
                        "Invalid value: '{}' for column: '{}' of type String",
                        value,
                        c
                    )),
                }
            }
            ColumnName::Int(c) => {
                if let Some(value) = Self::handle_null(operator, c) {
                    return value;
                }
                match value {
                    FilterValue::Int(v) => Ok(FilterColumn::Int(c, v)),
                    FilterValue::IntList(v) => Ok(FilterColumn::IntList(c, v)),
                    _ => Err(eyre::eyre!(
                        "Invalid value: '{}' for column: '{}' of type Int",
                        value.to_string(),
                        c
                    )),
                }
            }
            ColumnName::Float(c) => {
                if let Some(value) = Self::handle_null(operator, c) {
                    return value;
                }
                match value {
                    FilterValue::Float(v) => Ok(FilterColumn::Float(c, v)),
                    FilterValue::FloatList(v) => Ok(FilterColumn::FloatList(c, v)),
                    _ => Err(eyre::eyre!(
                        "Invalid value: '{}' for column: '{}' of type Float",
                        value.to_string(),
                        c
                    )),
                }
            }
            ColumnName::Bool(c) => {
                if let Some(value) = Self::handle_null(operator, c) {
                    return value;
                }
                match value {
                    FilterValue::Bool(v) => Ok(FilterColumn::Bool(c, v)),
                    FilterValue::BoolList(v) => Ok(FilterColumn::BoolList(c, v)),
                    _ => Err(eyre::eyre!(
                        "Invalid value: '{}' for column: '{}' of type Bool",
                        value.to_string(),
                        c
                    )),
                }
            }
        }
    }

    fn handle_null(
        operator: FilterOperator,
        c: &'static str,
    ) -> Option<eyre::Result<FilterColumn>> {
        if operator == FilterOperator::IsNull || operator == FilterOperator::IsNotNull {
            return Some(Ok(FilterColumn::String(c, "".to_string())));
        };
        None
    }

    fn handle_like(
        operator: FilterOperator,
        c: &'static str,
        v: String,
    ) -> Option<eyre::Result<FilterColumn>> {
        if operator == FilterOperator::Like || operator == FilterOperator::NotLike {
            return Some(Ok(FilterColumn::String(c, format!("'%{}%'", v))));
        };
        None
    }

    fn handle_starts_with(
        operator: FilterOperator,
        c: &'static str,
        v: String,
    ) -> Option<eyre::Result<FilterColumn>> {
        if operator == FilterOperator::StartsWith {
            return Some(Ok(FilterColumn::String(c, format!("'{}%'", v))));
        };
        None
    }

    fn handle_ends_with(
        operator: FilterOperator,
        c: &'static str,
        v: String,
    ) -> Option<eyre::Result<FilterColumn>> {
        if operator == FilterOperator::EndsWith {
            return Some(Ok(FilterColumn::String(c, format!("'%{}'", v))));
        };
        None
    }

    fn sql_str(column: &str, operator: &str, value: String) -> String {
        format!("{} {} {}", column, operator, value)
    }

    fn sql_str_i(case_insensitive: bool, column: &str, operator: &str, value: String) -> String {
        if case_insensitive {
            format!("LOWER({}) {} LOWER({})", column, operator, value)
        } else {
            format!("{} {} {}", column, operator, value)
        }
    }

    fn sql_str_in(
        case_insensitive: bool,
        column: &str,
        operator: &str,
        value: Vec<String>,
    ) -> String {
        if case_insensitive {
            format!(
                "LOWER({}) {} ({})",
                column,
                operator,
                value
                    .into_iter()
                    .map(|v| format!("LOWER('{}')", v))
                    .collect::<Vec<String>>()
                    .join(", ")
            )
        } else {
            format!(
                "{} {} ({})",
                column,
                operator,
                value
                    .into_iter()
                    .map(|v| format!("'{}'", v))
                    .collect::<Vec<String>>()
                    .join(", ")
            )
        }
    }

    fn sql_str_in_int(column: &str, operator: &str, value: Vec<i64>) -> String {
        format!(
            "{} {} ({})",
            column,
            operator,
            value
                .into_iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        )
    }

    fn sql_str_in_float(column: &str, operator: &str, value: Vec<f64>) -> String {
        format!(
            "{} {} ({})",
            column,
            operator,
            value
                .into_iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        )
    }

    fn sql_str_in_bool(column: &str, operator: &str, value: Vec<bool>) -> String {
        format!(
            "{} {} ({})",
            column,
            operator,
            value
                .into_iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        )
    }

    fn sql_str_null(column: &str, operator: &str) -> String {
        format!("{} {}", column, operator)
    }

    fn operators(
        cs: bool,
        filter_operator: FilterOperator,
        filter_column: FilterColumn,
    ) -> eyre::Result<String> {
        match filter_operator {
            FilterOperator::Equal => match filter_column {
                FilterColumn::String(c, v) => Ok(Filtering::sql_str_i(cs, c, "=", v)),
                FilterColumn::Int(c, v) => Ok(Filtering::sql_str(c, "=", v.to_string())),
                FilterColumn::Float(c, v) => Ok(Filtering::sql_str(c, "=", v.to_string())),
                FilterColumn::Bool(c, v) => Ok(Filtering::sql_str(c, "=", v.to_string())),
                _ => Err(eyre::eyre!(
                    "Invalid column type '{}' for filter operator Equal",
                    filter_column.to_string()
                )),
            },
            FilterOperator::NotEqual => match filter_column {
                FilterColumn::String(c, v) => Ok(Filtering::sql_str_i(cs, c, "!=", v)),
                FilterColumn::Int(c, v) => Ok(Filtering::sql_str(c, "!=", v.to_string())),
                FilterColumn::Float(c, v) => Ok(Filtering::sql_str(c, "!=", v.to_string())),
                FilterColumn::Bool(c, v) => Ok(Filtering::sql_str(c, "!=", v.to_string())),
                _ => Err(eyre::eyre!(
                    "Invalid column type '{}' for filter operator NotEqual",
                    filter_column.to_string()
                )),
            },
            FilterOperator::GreaterThan => match filter_column {
                FilterColumn::String(c, v) => Ok(Filtering::sql_str_i(cs, c, ">", v.to_string())),
                FilterColumn::Int(c, v) => Ok(Filtering::sql_str(c, ">", v.to_string())),
                FilterColumn::Float(c, v) => Ok(Filtering::sql_str(c, ">", v.to_string())),
                FilterColumn::Bool(c, v) => {
                    Ok(Filtering::sql_str_i(cs, c, ">", format!("'{}'", v)))
                }

                _ => Err(eyre::eyre!(
                    "Invalid column type '{}' for filter operator GreaterThan",
                    filter_column.to_string()
                )),
            },
            FilterOperator::GreaterThanOrEqual => match filter_column {
                FilterColumn::String(c, v) => Ok(Filtering::sql_str_i(cs, c, ">=", v.to_string())),
                FilterColumn::Int(c, v) => Ok(Filtering::sql_str(c, ">=", v.to_string())),
                FilterColumn::Float(c, v) => Ok(Filtering::sql_str(c, ">=", v.to_string())),
                FilterColumn::Bool(c, v) => {
                    Ok(Filtering::sql_str_i(cs, c, ">=", format!("'{}'", v)))
                }

                _ => Err(eyre::eyre!(
                    "Invalid column type '{}' for filter operator GreaterThanOrEqual",
                    filter_column.to_string()
                )),
            },
            FilterOperator::LessThan => match filter_column {
                FilterColumn::String(c, v) => Ok(Filtering::sql_str_i(cs, c, "<", v.to_string())),
                FilterColumn::Int(c, v) => Ok(Filtering::sql_str(c, "<", v.to_string())),
                FilterColumn::Float(c, v) => Ok(Filtering::sql_str(c, "<", v.to_string())),
                FilterColumn::Bool(c, v) => {
                    Ok(Filtering::sql_str_i(cs, c, "<", format!("'{}'", v)))
                }

                _ => Err(eyre::eyre!(
                    "Invalid column type '{}' for filter operator LessThan",
                    filter_column.to_string()
                )),
            },
            FilterOperator::LessThanOrEqual => match filter_column {
                FilterColumn::String(c, v) => Ok(Filtering::sql_str_i(cs, c, "<=", v.to_string())),
                FilterColumn::Int(c, v) => Ok(Filtering::sql_str(c, "<=", v.to_string())),
                FilterColumn::Float(c, v) => Ok(Filtering::sql_str(c, "<=", v.to_string())),
                FilterColumn::Bool(c, v) => {
                    Ok(Filtering::sql_str_i(cs, c, "<=", format!("'{}'", v)))
                }

                _ => Err(eyre::eyre!(
                    "Invalid column type '{}' for filter operator LessThanOrEqual",
                    filter_column.to_string()
                )),
            },
            FilterOperator::Like => match filter_column {
                FilterColumn::String(c, v) => Ok(Filtering::sql_str_i(cs, c, "LIKE", v)),

                // try to convert the value to a valid type for the operator
                FilterColumn::Int(c, v) => {
                    Ok(Filtering::sql_str_i(cs, c, "LIKE", format!("'%{}%'", v)))
                }
                FilterColumn::Float(c, v) => {
                    Ok(Filtering::sql_str_i(cs, c, "LIKE", format!("'%{}%'", v)))
                }
                FilterColumn::Bool(c, v) => {
                    Ok(Filtering::sql_str_i(cs, c, "LIKE", format!("'%{}%'", v)))
                }

                _ => Err(eyre::eyre!(
                    "Invalid column type '{}' for filter operator Like",
                    filter_column.to_string()
                )),
            },
            FilterOperator::NotLike => match filter_column {
                FilterColumn::String(c, v) => Ok(Filtering::sql_str_i(cs, c, "NOT LIKE", v)),

                // try to convert the value to a valid type for the operator
                FilterColumn::Int(c, v) => Ok(Filtering::sql_str_i(
                    cs,
                    c,
                    "NOT LIKE",
                    format!("'%{}%'", v),
                )),
                FilterColumn::Float(c, v) => Ok(Filtering::sql_str_i(
                    cs,
                    c,
                    "NOT LIKE",
                    format!("'%{}%'", v),
                )),
                FilterColumn::Bool(c, v) => Ok(Filtering::sql_str_i(
                    cs,
                    c,
                    "NOT LIKE",
                    format!("'%{}%'", v),
                )),

                _ => Err(eyre::eyre!(
                    "Invalid column type '{}' for filter operator NotLike",
                    filter_column.to_string()
                )),
            },
            FilterOperator::In => match filter_column {
                FilterColumn::StringList(c, v) => Ok(Filtering::sql_str_in(cs, c, "IN", v)),
                FilterColumn::IntList(c, v) => Ok(Filtering::sql_str_in_int(c, "IN", v)),
                FilterColumn::FloatList(c, v) => Ok(Filtering::sql_str_in_float(c, "IN", v)),
                FilterColumn::BoolList(c, v) => Ok(Filtering::sql_str_in_bool(c, "IN", v)),
                _ => Err(eyre::eyre!(
                    "Invalid column type '{}' for filter operator In",
                    filter_column.to_string()
                )),
            },
            FilterOperator::NotIn => match filter_column {
                FilterColumn::StringList(c, v) => Ok(Filtering::sql_str_in(cs, c, "NOT IN", v)),
                FilterColumn::IntList(c, v) => Ok(Filtering::sql_str_in_int(c, "NOT IN", v)),
                FilterColumn::FloatList(c, v) => Ok(Filtering::sql_str_in_float(c, "NOT IN", v)),
                FilterColumn::BoolList(c, v) => Ok(Filtering::sql_str_in_bool(c, "NOT IN", v)),
                _ => Err(eyre::eyre!(
                    "Invalid column type '{}' for filter operator NotIn",
                    filter_column.to_string()
                )),
            },
            FilterOperator::IsNull => match filter_column {
                FilterColumn::String(c, _) => Ok(Filtering::sql_str_null(c, "IS NULL")),
                FilterColumn::Int(c, _) => Ok(Filtering::sql_str_null(c, "IS NULL")),
                FilterColumn::Float(c, _) => Ok(Filtering::sql_str_null(c, "IS NULL")),
                FilterColumn::Bool(c, _) => Ok(Filtering::sql_str_null(c, "IS NULL")),
                _ => Err(eyre::eyre!(
                    "Invalid column type '{}' for filter operator IsNull",
                    filter_column.to_string()
                )),
            },
            FilterOperator::IsNotNull => match filter_column {
                FilterColumn::String(c, _) => Ok(Filtering::sql_str_null(c, "IS NOT NULL")),
                FilterColumn::Int(c, _) => Ok(Filtering::sql_str_null(c, "IS NOT NULL")),
                FilterColumn::Float(c, _) => Ok(Filtering::sql_str_null(c, "IS NOT NULL")),
                FilterColumn::Bool(c, _) => Ok(Filtering::sql_str_null(c, "IS NOT NULL")),
                _ => Err(eyre::eyre!(
                    "Invalid column type '{}' for filter operator IsNotNull",
                    filter_column.to_string()
                )),
            },
            FilterOperator::StartsWith => match filter_column {
                FilterColumn::String(c, v) => Ok(Filtering::sql_str_i(cs, c, "LIKE", v)),

                // try to convert the value to a valid type for the operator
                FilterColumn::Int(c, v) => {
                    Ok(Filtering::sql_str_i(cs, c, "LIKE", format!("'{}%'", v)))
                }
                FilterColumn::Float(c, v) => {
                    Ok(Filtering::sql_str_i(cs, c, "LIKE", format!("'{}%'", v)))
                }
                FilterColumn::Bool(c, v) => {
                    Ok(Filtering::sql_str_i(cs, c, "LIKE", format!("'{}%'", v)))
                }

                _ => Err(eyre::eyre!(
                    "Invalid column type '{}' for filter operator Like",
                    filter_column.to_string()
                )),
            },
            FilterOperator::EndsWith => match filter_column {
                FilterColumn::String(c, v) => Ok(Filtering::sql_str_i(cs, c, "LIKE", v)),

                // try to convert the value to a valid type for the operator
                FilterColumn::Int(c, v) => {
                    Ok(Filtering::sql_str_i(cs, c, "LIKE", format!("'%{}'", v)))
                }
                FilterColumn::Float(c, v) => {
                    Ok(Filtering::sql_str_i(cs, c, "LIKE", format!("'%{}'", v)))
                }
                FilterColumn::Bool(c, v) => {
                    Ok(Filtering::sql_str_i(cs, c, "LIKE", format!("'%{}'", v)))
                }

                _ => Err(eyre::eyre!(
                    "Invalid column type '{}' for filter operator Like",
                    filter_column.to_string()
                )),
            },
        }
    }

    pub fn new(rules: &[eyre::Result<FilteringRule>], case_insensitive: bool) -> Filtering {
        // log out any invalid rules to the console
        for rule in rules.iter() {
            match rule {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("{}", e);
                }
            }
        }

        let rules = rules.iter().flatten().collect::<Vec<&FilteringRule>>();

        let mut sql = if !rules.is_empty() {
            " WHERE ".to_string()
        } else {
            "".to_string()
        };
        let mut first = true;

        for rule in rules.iter() {
            let mut conditional_sql = "".to_string();
            if first {
                first = false;
            } else {
                match rule.conditional_operator {
                    ConditionalOperator::And => {
                        conditional_sql.push_str(" AND ");
                    }
                    ConditionalOperator::Or => {
                        conditional_sql.push_str(" OR ");
                    }
                }
            }

            let sql_string = Filtering::operators(
                case_insensitive,
                rule.filter_operator.clone(),
                rule.filter_column.clone(),
            );
            match sql_string {
                Ok(v) => {
                    sql.push_str(conditional_sql.as_str());
                    sql.push_str(v.as_str())
                }
                Err(e) => eprintln!("{}", e),
            }
        }

        Filtering {
            filters: rules.into_iter().map(|r| r.into()).collect(),
            sql,
            case_insensitive,
        }
    }
}
