#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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
        let mut rules = rules;
        rules.sort_by(|a, b| a.column.cmp(&b.column));
        rules.dedup_by(|a, b| a.column == b.column);

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
