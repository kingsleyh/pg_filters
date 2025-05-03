use eyre::Result;
use pg_filters::filtering::{FilterBuilder, FilterCondition, FilterOperator};

#[test]
fn test_filtering() -> Result<()> {
    let sql = FilterBuilder::new()
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

    assert_eq!(sql, " WHERE (LOWER(name) = LOWER('John') AND age > 18)");
    Ok(())
}

#[test]
fn test_filtering_case_sensitive() -> Result<()> {
    let sql = FilterBuilder::new()
        .case_insensitive(false)
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

    assert_eq!(sql, " WHERE (name = 'John' AND age > 18)");
    Ok(())
}

#[test]
fn test_filtering_with_bool() -> Result<()> {
    let sql = FilterBuilder::new()
        .case_insensitive(true)
        .add_condition(FilterCondition::TextValue {
            column: "name".to_string(),
            operator: FilterOperator::Equal,
            value: Some("John".to_string()),
        })
        .add_condition(FilterCondition::BooleanValue {
            column: "completed".to_string(),
            operator: FilterOperator::Equal,
            value: Some(true),
        })
        .build()?;

    assert_eq!(
        sql,
        " WHERE (LOWER(name) = LOWER('John') AND completed = true)"
    );
    Ok(())
}

#[test]
fn test_filtering_with_float() -> Result<()> {
    let sql = FilterBuilder::new()
        .case_insensitive(true)
        .add_condition(FilterCondition::TextValue {
            column: "name".to_string(),
            operator: FilterOperator::Equal,
            value: Some("John".to_string()),
        })
        .add_condition(FilterCondition::RealValue {
            column: "value".to_string(),
            operator: FilterOperator::LessThanOrEqual,
            value: Some(1.1),
        })
        .build()?;

    assert_eq!(sql, " WHERE (LOWER(name) = LOWER('John') AND value <= 1.1)");
    Ok(())
}

#[test]
fn test_filtering_with_uuid() -> Result<()> {
    let sql = FilterBuilder::new()
        .case_insensitive(true)
        .add_condition(FilterCondition::UuidValue {
            column: "id".to_string(),
            operator: FilterOperator::Equal,
            value: Some("30cbcd23-2660-44fa-a051-a13c4e2aa63a".to_string()),
        })
        .build()?;

    assert_eq!(sql, " WHERE id = '30cbcd23-2660-44fa-a051-a13c4e2aa63a'");
    Ok(())
}

#[test]
fn test_filtering_with_duplicate_columns() -> Result<()> {
    let sql = FilterBuilder::new()
        .case_insensitive(true)
        .add_condition(FilterCondition::TextValue {
            column: "name".to_string(),
            operator: FilterOperator::Equal,
            value: Some("John".to_string()),
        })
        .add_condition(FilterCondition::TextValue {
            column: "name".to_string(),
            operator: FilterOperator::Equal,
            value: Some("Doe".to_string()),
        })
        .build()?;

    assert_eq!(
        sql,
        " WHERE (LOWER(name) = LOWER('John') AND LOWER(name) = LOWER('Doe'))"
    );
    Ok(())
}

#[test]
fn test_filtering_with_single_rule() -> Result<()> {
    let sql = FilterBuilder::new()
        .case_insensitive(true)
        .add_condition(FilterCondition::TextValue {
            column: "name".to_string(),
            operator: FilterOperator::Equal,
            value: Some("John".to_string()),
        })
        .build()?;

    assert_eq!(sql, " WHERE LOWER(name) = LOWER('John')");
    Ok(())
}

#[test]
fn test_filtering_with_empty_rules() -> Result<()> {
    let sql = FilterBuilder::new().build()?;
    assert_eq!(sql, "");
    Ok(())
}

#[test]
fn test_filtering_with_equal_to_rule() -> Result<()> {
    let sql = FilterBuilder::new()
        .case_insensitive(true)
        .add_condition(FilterCondition::TextValue {
            column: "name".to_string(),
            operator: FilterOperator::Equal,
            value: Some("John".to_string()),
        })
        .build()?;

    assert_eq!(sql, " WHERE LOWER(name) = LOWER('John')");
    Ok(())
}

#[test]
fn test_filtering_with_not_equal_to_rule() -> Result<()> {
    let sql = FilterBuilder::new()
        .case_insensitive(true)
        .add_condition(FilterCondition::TextValue {
            column: "name".to_string(),
            operator: FilterOperator::NotEqual,
            value: Some("John".to_string()),
        })
        .build()?;

    assert_eq!(sql, " WHERE LOWER(name) != LOWER('John')");
    Ok(())
}

#[test]
fn test_filtering_with_greater_than_rule() -> Result<()> {
    let sql = FilterBuilder::new()
        .add_condition(FilterCondition::IntegerValue {
            column: "age".to_string(),
            operator: FilterOperator::GreaterThan,
            value: Some(18),
        })
        .build()?;

    assert_eq!(sql, " WHERE age > 18");
    Ok(())
}

#[test]
fn test_filtering_with_greater_than_or_equal_to_rule() -> Result<()> {
    let sql = FilterBuilder::new()
        .add_condition(FilterCondition::IntegerValue {
            column: "age".to_string(),
            operator: FilterOperator::GreaterThanOrEqual,
            value: Some(18),
        })
        .build()?;

    assert_eq!(sql, " WHERE age >= 18");
    Ok(())
}

#[test]
fn test_filtering_with_less_than_rule() -> Result<()> {
    let sql = FilterBuilder::new()
        .add_condition(FilterCondition::IntegerValue {
            column: "age".to_string(),
            operator: FilterOperator::LessThan,
            value: Some(18),
        })
        .build()?;

    assert_eq!(sql, " WHERE age < 18");
    Ok(())
}

#[test]
fn test_filtering_with_less_than_or_equal_to_rule() -> Result<()> {
    let sql = FilterBuilder::new()
        .add_condition(FilterCondition::IntegerValue {
            column: "age".to_string(),
            operator: FilterOperator::LessThanOrEqual,
            value: Some(18),
        })
        .build()?;

    assert_eq!(sql, " WHERE age <= 18");
    Ok(())
}

#[test]
fn test_filtering_with_like_rule() -> Result<()> {
    let sql = FilterBuilder::new()
        .case_insensitive(true)
        .add_condition(FilterCondition::TextValue {
            column: "name".to_string(),
            operator: FilterOperator::Like,
            value: Some("%John%".to_string()),
        })
        .build()?;

    assert_eq!(sql, " WHERE LOWER(name) LIKE LOWER('%John%')");
    Ok(())
}

#[test]
fn test_filtering_with_not_like_rule() -> Result<()> {
    let sql = FilterBuilder::new()
        .case_insensitive(true)
        .add_condition(FilterCondition::TextValue {
            column: "name".to_string(),
            operator: FilterOperator::NotLike,
            value: Some("%John%".to_string()),
        })
        .build()?;

    assert_eq!(sql, " WHERE LOWER(name) NOT LIKE LOWER('%John%')");
    Ok(())
}

#[test]
fn test_filtering_with_is_null() -> Result<()> {
    let sql = FilterBuilder::new()
        .add_condition(FilterCondition::TextValue {
            column: "name".to_string(),
            operator: FilterOperator::IsNull,
            value: None,
        })
        .build()?;

    assert_eq!(sql, " WHERE name IS NULL");
    Ok(())
}

#[test]
fn test_filtering_with_is_not_null() -> Result<()> {
    let sql = FilterBuilder::new()
        .add_condition(FilterCondition::TextValue {
            column: "name".to_string(),
            operator: FilterOperator::IsNotNull,
            value: None,
        })
        .build()?;

    assert_eq!(sql, " WHERE name IS NOT NULL");
    Ok(())
}

#[test]
fn test_filtering_with_json_filter_in_operator() -> Result<()> {
    use std::collections::HashMap;
    use pg_filters::{ColumnDef, filtering::JsonFilter};
    
    // Setup test column definitions
    let mut column_defs = HashMap::new();
    column_defs.insert("status", ColumnDef::Text("status"));
    column_defs.insert("age", ColumnDef::Integer("age"));
    
    // Create JSON filters with IN operator
    let json_filters = vec![
        JsonFilter {
            n: "status".to_string(),
            f: "in".to_string(),  // lowercase "in"
            v: "active,pending,approved".to_string(), // comma-separated values
            c: None,
        },
    ];
    
    // Build filter from JSON
    let filter_builder = FilterBuilder::from_json_filters(&json_filters, true, &column_defs)?;
    let sql = filter_builder.build()?;
    
    // Verify the SQL contains an IN clause
    assert_eq!(sql, " WHERE LOWER(status) IN (LOWER('active'), LOWER('pending'), LOWER('approved'))");
    Ok(())
}

#[test]
fn test_filtering_with_json_filter_multiple_in_operators() -> Result<()> {
    use std::collections::HashMap;
    use pg_filters::{ColumnDef, filtering::JsonFilter};
    
    // Setup test column definitions
    let mut column_defs = HashMap::new();
    column_defs.insert("status", ColumnDef::Text("status"));
    column_defs.insert("age", ColumnDef::Integer("age"));
    column_defs.insert("type", ColumnDef::Text("type"));
    
    // Create JSON filters with IN operator for different column types
    let json_filters = vec![
        JsonFilter {
            n: "status".to_string(),
            f: "in".to_string(),
            v: "active,pending".to_string(),
            c: None,
        },
        JsonFilter {
            n: "age".to_string(),
            f: "in".to_string(),
            v: "18,21,25".to_string(),
            c: Some("AND".to_string()),
        },
        JsonFilter {
            n: "type".to_string(),
            f: "NOT IN".to_string(), // Test uppercase with NOT IN
            v: "temp,test".to_string(),
            c: Some("AND".to_string()),
        },
    ];
    
    // Build filter from JSON
    let filter_builder = FilterBuilder::from_json_filters(&json_filters, false, &column_defs)?;
    let sql = filter_builder.build()?;
    
    // Verify the SQL contains multiple IN clauses
    assert_eq!(sql, " WHERE (status IN ('active', 'pending') AND age IN ('18', '21', '25') AND type NOT IN ('temp', 'test'))");
    Ok(())
}

#[test]
fn test_filtering_with_starts_with() -> Result<()> {
    let sql = FilterBuilder::new()
        .case_insensitive(true)
        .add_condition(FilterCondition::TextValue {
            column: "name".to_string(),
            operator: FilterOperator::StartsWith,
            value: Some("John".to_string()),
        })
        .build()?;

    assert_eq!(sql, " WHERE LOWER(name) LIKE LOWER('John%')");
    Ok(())
}

#[test]
fn test_filtering_with_ends_with() -> Result<()> {
    let sql = FilterBuilder::new()
        .case_insensitive(true)
        .add_condition(FilterCondition::TextValue {
            column: "name".to_string(),
            operator: FilterOperator::EndsWith,
            value: Some("John".to_string()),
        })
        .build()?;

    assert_eq!(sql, " WHERE LOWER(name) LIKE LOWER('%John')");
    Ok(())
}

#[test]
fn test_filtering_with_multiple_complex_conditions() -> Result<()> {
    let sql = FilterBuilder::new()
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
        .add_condition(FilterCondition::TextValue {
            column: "city".to_string(),
            operator: FilterOperator::Like,
            value: Some("%New York%".to_string()),
        })
        .build()?;

    assert_eq!(
        sql,
        " WHERE ((LOWER(name) = LOWER('John') AND age > 18) AND LOWER(city) LIKE LOWER('%New York%'))"
    );
    Ok(())
}

#[test]
fn test_filtering_with_type_casting() -> Result<()> {
    let sql = FilterBuilder::new()
        .case_insensitive(true)
        .add_condition(FilterCondition::TextValue {
            column: "name".to_string(),
            operator: FilterOperator::Like,
            value: Some("%John%".to_string()),
        })
        .add_condition(FilterCondition::IntegerValue {
            column: "code".to_string(),
            operator: FilterOperator::Equal,
            value: Some(12),
        })
        .add_condition(FilterCondition::RealValue {
            column: "price".to_string(),
            operator: FilterOperator::Equal,
            value: Some(10.4),
        })
        .add_condition(FilterCondition::BooleanValue {
            column: "active".to_string(),
            operator: FilterOperator::Equal,
            value: Some(true),
        })
        .build()?;

    assert_eq!(
        sql,
        " WHERE (((LOWER(name) LIKE LOWER('%John%') AND code = 12) AND price = 10.4) AND active = true)"
    );
    Ok(())
}
