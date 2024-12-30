use eyre::Result;
use std::collections::HashMap;

pub mod filtering;
pub mod pagination;
pub mod sorting;

use crate::filtering::{
    FilterBuilder, FilterCondition, FilterExpression, FilterOperator, JsonFilter, LogicalOperator,
};
use crate::pagination::Paginate;
use crate::sorting::{SortedColumn, Sorting};

#[derive(Debug, Clone)]
pub enum ColumnDef {
    // Numeric Types
    SmallInt(&'static str),
    Integer(&'static str),
    BigInt(&'static str),
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

    // Text Array
    TextArray(&'static str),

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
            "CONTAINS" => FilterOperator::Contains,
            "OVERLAPS" => FilterOperator::Overlaps,
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

            // Text Array types
            ColumnDef::TextArray(name) => match op {
                FilterOperator::Contains => Ok(FilterCondition::ArrayContains {
                    column: name.to_string(),
                    operator: op,
                    value: value.to_string(),
                }),
                FilterOperator::Overlaps => Ok(FilterCondition::ArrayOverlap {
                    column: name.to_string(),
                    operator: op,
                    values: value.split(',').map(|s| s.trim().to_string()).collect(),
                }),
                _ => Ok(FilterCondition::ArrayContains {
                    column: name.to_string(),
                    operator: FilterOperator::Contains,
                    value: value.to_string(),
                }),
            },

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
            ColumnDef::Real(name) | ColumnDef::DoublePrecision(name) => {
                Ok(FilterCondition::DoublePrecisionValue {
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
            | ColumnDef::TextArray(name)
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
    pub column_defs: HashMap<&'static str, ColumnDef>,
}

impl FilteringOptions {
    pub fn new(
        expressions: Vec<FilterExpression>,
        column_defs: HashMap<&'static str, ColumnDef>,
    ) -> Self {
        Self {
            expressions,
            case_insensitive: true,
            column_defs,
        }
    }

    pub fn case_sensitive(
        expressions: Vec<FilterExpression>,
        column_defs: HashMap<&'static str, ColumnDef>,
    ) -> Self {
        Self {
            expressions,
            case_insensitive: false,
            column_defs,
        }
    }

    pub fn from_json_filters(
        filters: &[JsonFilter],
        column_defs: HashMap<&'static str, ColumnDef>,
    ) -> Result<Option<Self>> {
        if filters.is_empty() {
            return Ok(None);
        }

        let filter_builder = FilterBuilder::from_json_filters(filters, true, &column_defs)?;
        Ok(filter_builder
            .root
            .map(|root| Self::new(vec![root], column_defs)))
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
        column_defs: HashMap<&'static str, ColumnDef>,
    ) -> Result<Option<Self>> {
        let expressions: Result<Vec<_>, _> = expressions.into_iter().collect();
        match expressions {
            Ok(exprs) if !exprs.is_empty() => Ok(Some(Self::new(exprs, column_defs))),
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
    pub column_defs: HashMap<&'static str, ColumnDef>,
}

impl PgFilters {
    pub fn new(
        pagination: Option<PaginationOptions>,
        sorting_columns: Vec<SortedColumn>,
        filtering_options: Option<FilteringOptions>,
        column_defs: HashMap<&'static str, ColumnDef>,
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
            column_defs,
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
        columns.insert("field1", ColumnDef::Text("field1"));
        columns.insert("field2", ColumnDef::Text("field2"));
        columns.insert("field3", ColumnDef::Text("field3"));
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

        let filtering_options = FilteringOptions::from_json_filters(&filters, columns)?;
        let sql = filtering_options.unwrap().to_filter_builder()?.build()?;
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
                n: "field1".to_string(),
                f: "=".to_string(),
                v: "value1".to_string(),
                c: None,
            },
            JsonFilter {
                n: "field2".to_string(),
                f: "=".to_string(),
                v: "value2".to_string(),
                c: Some("OR".to_string()),
            },
            JsonFilter {
                n: "field3".to_string(),
                f: "=".to_string(),
                v: "value3".to_string(),
                c: Some("OR".to_string()),
            },
        ];

        let filtering_options = FilteringOptions::from_json_filters(&filters, columns)?;
        let sql = filtering_options.unwrap().to_filter_builder()?.build()?;
        assert_eq!(
            sql,
            " WHERE (LOWER(field1) = LOWER('value1') OR LOWER(field2) = LOWER('value2') OR LOWER(field3) = LOWER('value3'))"
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

        let filtering_options = FilteringOptions::from_json_filters(&filters, columns)?;
        let sql = filtering_options.unwrap().to_filter_builder()?.build()?;
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

        let filtering_options = FilteringOptions::from_json_filters(&filters, columns)?;
        let sql = filtering_options.unwrap().to_filter_builder()?.build()?;
        assert_eq!(
            sql,
            " WHERE (LOWER(status) = LOWER('active') AND (age > 21 OR LOWER(city) = LOWER('New York') OR LOWER(city) = LOWER('London') OR LOWER(department) = LOWER('Sales')))"
        );
        Ok(())
    }

    #[test]
    fn test_empty_filters() -> Result<()> {
        let columns = setup_test_columns();
        let filters: Vec<JsonFilter> = vec![];
        let filtering_options = FilteringOptions::from_json_filters(&filters, columns)?;
        assert!(filtering_options.is_none());
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

        let filtering_options = FilteringOptions::from_json_filters(&filters, columns)?;
        let sql = filtering_options.unwrap().to_filter_builder()?.build()?;
        assert_eq!(sql, " WHERE LOWER(name) LIKE LOWER('%John%')");
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

        let filtering_options = FilteringOptions::from_json_filters(&filters, columns)?;
        let sql = filtering_options.unwrap().to_filter_builder()?.build()?;
        assert_eq!(sql, " WHERE (age > 25 OR salary < 50000)");
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
        let filtering_options = FilteringOptions::case_sensitive(
            vec![FilterExpression::Condition(FilterCondition::text(
                "name",
                FilterOperator::Like,
                Some("%John%"),
            ))],
            columns.clone(),
        );
        let sql = filtering_options.to_filter_builder()?.build()?;
        assert_eq!(sql, " WHERE name LIKE '%John%'");

        // Test case insensitive
        let filtering_options = FilteringOptions::from_json_filters(&filters, columns)?;
        let sql = filtering_options.unwrap().to_filter_builder()?.build()?;
        assert_eq!(
            sql,
            " WHERE (LOWER(name) LIKE LOWER('%John%') OR LOWER(email) LIKE LOWER('%gmail.com'))"
        );
        Ok(())
    }

    #[test]
    fn test_uuid_filter() -> Result<()> {
        let columns = setup_test_columns();
        let uuid = "123e4567-e89b-12d3-a456-426614174000";
        let filters = vec![JsonFilter {
            n: "id".to_string(),
            f: "=".to_string(),
            v: uuid.to_string(),
            c: None,
        }];

        let filtering_options = FilteringOptions::from_json_filters(&filters, columns)?;
        let sql = filtering_options.unwrap().to_filter_builder()?.build()?;
        assert_eq!(sql, format!(" WHERE id = '{}'", uuid));
        Ok(())
    }
}
