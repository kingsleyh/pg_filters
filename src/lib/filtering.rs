#[derive(Debug, Clone, PartialEq)]
pub enum FilterValue {
    String(String),
    Int(i64),
    Float(f64),
    Bool(bool),
}

#[derive(Debug, Clone)]
pub struct FilteringRule {
    pub column: String,
    pub filter_operator: FilterOperator,
    pub conditional_operator: ConditionalOperator,
    pub value: FilterValue,
}

impl FilteringRule {
    pub fn new(
        column: String,
        filter_operator: String,
        conditional_operator: String,
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
}

#[derive(Debug, Clone)]
pub struct Filtering {
    pub filters: Vec<FilteringRule>,
    pub sql: String,
}

impl Filtering {
    pub fn new(rules: Vec<FilteringRule>) -> Filtering {
        let mut sql = if rules.len() > 0 {
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
            sql.push_str(" ");
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

            // if rule.operator == FilterOperator::In || rule.operator == FilterOperator::NotIn then wrap the value in parentheses
            // if rule.operator == FilterOperator::Null || rule.operator == FilterOperator::NotNull then don't add the value
            if rule.filter_operator == FilterOperator::In
                || rule.filter_operator == FilterOperator::NotIn
            {
                // remove the single quotes from the start and end of the string if present
                let filter_value = filter_value.trim_matches('\'');
                sql.push_str(&filter_value);
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
