use crate::filtering::{
    FilterBuilder, FilterCondition, FilterExpression, FilterOperator, LogicalOperator,
};
use crate::pagination::Paginate;
use crate::sorting::{SortedColumn, Sorting};
use eyre::Result;

pub mod filtering;
pub mod pagination;
pub mod sorting;

#[derive(Debug, Clone)]
pub enum ColumnDef {
    // Numeric Types
    SmallInt(&'static str),
    Integer(&'static str),
    BigInt(&'static str),
    Decimal(&'static str),
    Real(&'static str),
    DoublePrecision(&'static str),

    // Character Types
    Text(&'static str),
    Varchar(&'static str),
    Char(&'static str),

    // Date/Time Types
    Date(&'static str),
    Time(&'static str),
    TimeTz(&'static str),
    Timestamp(&'static str),
    TimestampTz(&'static str),
    Interval(&'static str),

    // Boolean Type
    Boolean(&'static str),

    // Network Address Types
    Inet(&'static str),
    Cidr(&'static str),
    MacAddr(&'static str),
    MacAddr8(&'static str),

    // UUID Type
    Uuid(&'static str),

    // JSON Types
    Json(&'static str),
    Jsonb(&'static str),

    // Binary Data
    ByteA(&'static str),

    // Money
    Money(&'static str),

    // XML
    Xml(&'static str),
}

impl ColumnDef {
    pub fn to_filter_condition(&self, operator: &str, value: &str) -> Result<FilterCondition> {
        let op = match operator.to_uppercase().as_str() {
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
            _ => return Err(eyre::eyre!("Invalid operator: {}", operator)),
        };

        if op == FilterOperator::In || op == FilterOperator::NotIn {
            // Parse comma-separated values into a list
            let values = value
                .split(',')
                .map(|v| v.trim().to_string())
                .collect::<Vec<String>>();

            return Ok(FilterCondition::InValues {
                column: self.get_column_name(),
                operator: op,
                values,
            });
        }

        match self {
            // Character Types
            ColumnDef::Text(name) | ColumnDef::Varchar(name) | ColumnDef::Char(name) => {
                Ok(FilterCondition::TextValue {
                    column: name.to_string(),
                    operator: op,
                    value: if operator == "IS NULL" || operator == "IS NOT NULL" {
                        None
                    } else {
                        Some(value.to_string())
                    },
                })
            }

            // Numeric Types
            ColumnDef::SmallInt(name) => Ok(FilterCondition::SmallIntValue {
                column: name.to_string(),
                operator: op,
                value: if operator == "IS NULL" || operator == "IS NOT NULL" {
                    None
                } else {
                    Some(
                        value
                            .parse::<i16>()
                            .map_err(|_| eyre::eyre!("Invalid small integer value: {}", value))?,
                    )
                },
            }),
            ColumnDef::Integer(name) => Ok(FilterCondition::IntegerValue {
                column: name.to_string(),
                operator: op,
                value: if operator == "IS NULL" || operator == "IS NOT NULL" {
                    None
                } else {
                    Some(
                        value
                            .parse::<i32>()
                            .map_err(|_| eyre::eyre!("Invalid integer value: {}", value))?,
                    )
                },
            }),
            ColumnDef::BigInt(name) => Ok(FilterCondition::BigIntValue {
                column: name.to_string(),
                operator: op,
                value: if operator == "IS NULL" || operator == "IS NOT NULL" {
                    None
                } else {
                    Some(
                        value
                            .parse::<i64>()
                            .map_err(|_| eyre::eyre!("Invalid big integer value: {}", value))?,
                    )
                },
            }),
            ColumnDef::Decimal(name) | ColumnDef::Real(name) | ColumnDef::DoublePrecision(name) => {
                Ok(FilterCondition::DecimalValue {
                    column: name.to_string(),
                    operator: op,
                    value: if operator == "IS NULL" || operator == "IS NOT NULL" {
                        None
                    } else {
                        Some(
                            value
                                .parse::<f64>()
                                .map_err(|_| eyre::eyre!("Invalid decimal value: {}", value))?,
                        )
                    },
                })
            }

            // Boolean Type
            ColumnDef::Boolean(name) => Ok(FilterCondition::BooleanValue {
                column: name.to_string(),
                operator: op,
                value: if operator == "IS NULL" || operator == "IS NOT NULL" {
                    None
                } else {
                    Some(
                        value
                            .parse::<bool>()
                            .map_err(|_| eyre::eyre!("Invalid boolean value: {}", value))?,
                    )
                },
            }),

            // Date/Time Types
            ColumnDef::Date(name)
            | ColumnDef::Time(name)
            | ColumnDef::TimeTz(name)
            | ColumnDef::Timestamp(name)
            | ColumnDef::TimestampTz(name)
            | ColumnDef::Interval(name) => Ok(FilterCondition::TimestampValue {
                column: name.to_string(),
                operator: op,
                value: if operator == "IS NULL" || operator == "IS NOT NULL" {
                    None
                } else {
                    Some(value.to_string())
                },
            }),

            // UUID Type
            ColumnDef::Uuid(name) => Ok(FilterCondition::UuidValue {
                column: name.to_string(),
                operator: op,
                value: if operator == "IS NULL" || operator == "IS NOT NULL" {
                    None
                } else {
                    Some(value.to_string())
                },
            }),

            // JSON Types
            ColumnDef::Json(name) | ColumnDef::Jsonb(name) => Ok(FilterCondition::JsonbValue {
                column: name.to_string(),
                operator: op,
                value: if operator == "IS NULL" || operator == "IS NOT NULL" {
                    None
                } else {
                    Some(value.to_string())
                },
            }),

            // Network Address Types
            ColumnDef::Inet(name)
            | ColumnDef::Cidr(name)
            | ColumnDef::MacAddr(name)
            | ColumnDef::MacAddr8(name) => Ok(FilterCondition::TextValue {
                column: name.to_string(),
                operator: op,
                value: if operator == "IS NULL" || operator == "IS NOT NULL" {
                    None
                } else {
                    Some(value.to_string())
                },
            }),

            // Binary Data
            ColumnDef::ByteA(name) => Ok(FilterCondition::TextValue {
                column: name.to_string(),
                operator: op,
                value: if operator == "IS NULL" || operator == "IS NOT NULL" {
                    None
                } else {
                    Some(value.to_string())
                },
            }),

            // Money Type
            ColumnDef::Money(name) => Ok(FilterCondition::TextValue {
                column: name.to_string(),
                operator: op,
                value: if operator == "IS NULL" || operator == "IS NOT NULL" {
                    None
                } else {
                    Some(value.to_string())
                },
            }),

            // XML Type
            ColumnDef::Xml(name) => Ok(FilterCondition::TextValue {
                column: name.to_string(),
                operator: op,
                value: if operator == "IS NULL" || operator == "IS NOT NULL" {
                    None
                } else {
                    Some(value.to_string())
                },
            }),
        }
    }

    fn get_column_name(&self) -> String {
        match self {
            ColumnDef::Text(name)
            | ColumnDef::Varchar(name)
            | ColumnDef::Char(name)
            | ColumnDef::SmallInt(name)
            | ColumnDef::Integer(name)
            | ColumnDef::BigInt(name)
            | ColumnDef::Decimal(name)
            | ColumnDef::Real(name)
            | ColumnDef::DoublePrecision(name)
            | ColumnDef::Date(name)
            | ColumnDef::Time(name)
            | ColumnDef::TimeTz(name)
            | ColumnDef::Timestamp(name)
            | ColumnDef::TimestampTz(name)
            | ColumnDef::Interval(name)
            | ColumnDef::Boolean(name)
            | ColumnDef::Inet(name)
            | ColumnDef::Cidr(name)
            | ColumnDef::MacAddr(name)
            | ColumnDef::MacAddr8(name)
            | ColumnDef::Uuid(name)
            | ColumnDef::Json(name)
            | ColumnDef::Jsonb(name)
            | ColumnDef::ByteA(name)
            | ColumnDef::Money(name)
            | ColumnDef::Xml(name) => name.to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PaginationOptions {
    pub current_page: i64,
    pub per_page: i64,
    pub per_page_limit: i64,
    pub total_records: i64,
}

impl PaginationOptions {
    pub fn new(current_page: i64, per_page: i64, per_page_limit: i64, total_records: i64) -> Self {
        Self {
            current_page,
            per_page,
            per_page_limit,
            total_records,
        }
    }
}

#[derive(Clone)]
pub struct FilteringOptions {
    pub expressions: Vec<FilterExpression>,
    pub case_insensitive: bool,
}

impl FilteringOptions {
    pub fn new(expressions: Vec<FilterExpression>) -> Self {
        Self {
            expressions,
            case_insensitive: true,
        }
    }

    pub fn case_sensitive(expressions: Vec<FilterExpression>) -> Self {
        Self {
            expressions,
            case_insensitive: false,
        }
    }

    pub fn to_filter_builder(&self) -> Result<FilterBuilder> {
        let mut builder = FilterBuilder::new().case_insensitive(self.case_insensitive);

        // If there are multiple expressions, wrap them in a group with AND operator
        if self.expressions.len() > 1 {
            builder = builder.group(LogicalOperator::And, self.expressions.clone());
        } else if let Some(expr) = self.expressions.first() {
            builder = builder.add_expression(expr.clone());
        }

        Ok(builder)
    }

    pub fn try_from_expressions(
        expressions: Vec<Result<FilterExpression, eyre::Error>>,
    ) -> Result<Option<Self>> {
        let expressions: Result<Vec<_>, _> = expressions.into_iter().collect();
        match expressions {
            Ok(exprs) if !exprs.is_empty() => Ok(Some(Self::new(exprs))),
            Ok(_) => Ok(None),
            Err(e) => Err(e),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PgFilters {
    pub pagination: Option<Paginate>,
    pub sorting: Option<Sorting>,
    pub filters: Option<FilterBuilder>,
}

impl PgFilters {
    pub fn new(
        pagination: Option<PaginationOptions>,
        sorting_columns: Vec<SortedColumn>,
        filtering_options: Option<FilteringOptions>,
    ) -> Result<PgFilters> {
        let pagination = pagination.map(|pagination| {
            Paginate::new(
                pagination.current_page,
                pagination.per_page,
                pagination.per_page_limit,
                pagination.total_records,
            )
        });

        let sorting = Sorting::new(sorting_columns);

        let filters = if let Some(filtering_options) = filtering_options {
            Some(filtering_options.to_filter_builder()?)
        } else {
            None
        };

        Ok(PgFilters {
            pagination,
            sorting: Some(sorting),
            filters,
        })
    }

    pub fn sql(&self) -> Result<String> {
        let mut sql = String::new();

        if let Some(filters) = &self.filters {
            sql.push_str(&filters.build()?);
        }

        if let Some(sorting) = &self.sorting {
            sql.push_str(&sorting.sql);
        }

        if let Some(pagination) = &self.pagination {
            sql.push(' ');
            sql.push_str(&pagination.sql);
        }

        Ok(sql)
    }

    pub fn count_sql(&self, schema: &str, table: &str) -> Result<String> {
        let mut sql = format!("SELECT COUNT(*) FROM {}.{}", schema, table);
        if let Some(filters) = &self.filters {
            sql.push_str(&filters.build()?);
        }
        Ok(sql)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pg_filters_with_complex_conditions() -> Result<()> {
        // Create test conditions
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

        // Create a complex filter group: (name = 'John' AND age > 18) OR city IN ('New York', 'London')
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
            vec![
                SortedColumn::new("age", "desc"),
                SortedColumn::new("name", "asc"),
            ],
            Some(FilteringOptions::new(vec![filter_group])),
        )?;

        let sql = filters.sql()?;
        assert_eq!(
            sql,
            " WHERE ((LOWER(name) = LOWER('John') AND age > 18) OR city IN ('New York', 'London')) ORDER BY age DESC, name ASC LIMIT 10 OFFSET 0"
        );
        Ok(())
    }

    #[test]
    fn test_column_def_to_filter_condition() -> Result<()> {
        // Test text column
        let text_condition = ColumnDef::Text("name")
            .to_filter_condition("=", "John")?
            .to_sql(true)?;
        assert_eq!(text_condition, "LOWER(name) = LOWER('John')");

        // Test integer column
        let int_condition = ColumnDef::Integer("age")
            .to_filter_condition(">", "18")?
            .to_sql(true)?;
        assert_eq!(int_condition, "age > 18");

        // Test IN condition
        let in_condition = ColumnDef::Text("city")
            .to_filter_condition("IN", "New York,London")?
            .to_sql(true)?;
        assert_eq!(in_condition, "city IN ('New York', 'London')");

        // Test NULL condition
        let null_condition = ColumnDef::Text("description")
            .to_filter_condition("IS NULL", "")?
            .to_sql(true)?;
        assert_eq!(null_condition, "description IS NULL");

        // Test LIKE condition
        let like_condition = ColumnDef::Text("name")
            .to_filter_condition("STARTS WITH", "Jo")?
            .to_sql(true)?;
        assert_eq!(like_condition, "LOWER(name) LIKE LOWER('Jo%')");

        Ok(())
    }

    #[test]
    fn test_pg_filters_without_conditions() -> Result<()> {
        let filters = PgFilters::new(
            Some(PaginationOptions {
                current_page: 1,
                per_page: 10,
                per_page_limit: 10,
                total_records: 1000,
            }),
            vec![SortedColumn::new("name", "asc")],
            None,
        )?;

        let sql = filters.sql()?;
        assert_eq!(sql, " ORDER BY name ASC LIMIT 10 OFFSET 0");
        Ok(())
    }

    #[test]
    fn test_pg_filters_with_only_pagination() -> Result<()> {
        let filters = PgFilters::new(
            Some(PaginationOptions {
                current_page: 2,
                per_page: 15,
                per_page_limit: 20,
                total_records: 1000,
            }),
            vec![],
            None,
        )?;

        let sql = filters.sql()?;
        assert_eq!(sql, " LIMIT 15 OFFSET 15");
        Ok(())
    }
}
