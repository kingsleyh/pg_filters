use pg_filters::filtering::{ConditionalOperator, FilterOperator, FilterValue, Filtering, FilteringRule};

#[test]
fn test_filtering() {
    let filtering = Filtering::new(vec![
        FilteringRule {
            column: "name".to_string(),
            filter_operator: FilterOperator::Equal,
            conditional_operator: ConditionalOperator::And,
            value: FilterValue::String("John".to_string()),
        },
        FilteringRule {
            column: "age".to_string(),
            filter_operator: FilterOperator::GreaterThan,
            conditional_operator: ConditionalOperator::Or,
            value: FilterValue::Int(18),
        },
    ]);
    assert_eq!(filtering.filters.len(), 2);
    assert_eq!(filtering.sql, " WHERE age > 18 AND name = 'John'");
}

#[test]
fn test_filtering_with_duplicate_columns() {
    let filtering = Filtering::new(vec![
        FilteringRule {
            column: "name".to_string(),
            filter_operator: FilterOperator::Equal,
            conditional_operator: ConditionalOperator::And,
            value: FilterValue::String("John".to_string()),
        },
        FilteringRule {
            column: "name".to_string(),
            filter_operator: FilterOperator::Equal,
            conditional_operator: ConditionalOperator::Or,
            value: FilterValue::String("Doe".to_string()),
        },
    ]);
    assert_eq!(filtering.filters.len(), 1);
    assert_eq!(filtering.sql, " WHERE name = 'John'");
}

#[test]
fn test_filtering_with_single_rule() {
    let filtering = Filtering::new(vec![FilteringRule {
        column: "name".to_string(),
        filter_operator: FilterOperator::Equal,
        conditional_operator: ConditionalOperator::And,
        value: FilterValue::String("John".to_string()),
    }]);
    assert_eq!(filtering.filters.len(), 1);
    assert_eq!(filtering.sql, " WHERE name = 'John'");
}

#[test]
fn test_filtering_with_empty_rules() {
    let filtering = Filtering::new(vec![]);
    assert_eq!(filtering.filters.len(), 0);
    assert_eq!(filtering.sql, "");
}

#[test]
fn test_filtering_with_multiple_rules() {
    let filtering = Filtering::new(vec![
        FilteringRule {
            column: "name".to_string(),
            filter_operator: FilterOperator::Equal,
            conditional_operator: ConditionalOperator::And,
            value: FilterValue::String("John".to_string()),
        },
        FilteringRule {
            column: "age".to_string(),
            filter_operator: FilterOperator::GreaterThan,
            conditional_operator: ConditionalOperator::Or,
            value: FilterValue::Int(18),
        },
    ]);
    assert_eq!(filtering.filters.len(), 2);
    assert_eq!(filtering.sql, " WHERE age > 18 AND name = 'John'");
}

#[test]
fn test_filtering_with_multiple_rules_swapped() {
    let filtering = Filtering::new(vec![
        FilteringRule {
            column: "name".to_string(),
            filter_operator: FilterOperator::Equal,
            conditional_operator: ConditionalOperator::Or,
            value: FilterValue::String("John".to_string()),
        },
        FilteringRule {
            column: "age".to_string(),
            filter_operator: FilterOperator::GreaterThan,
            conditional_operator: ConditionalOperator::And,
            value: FilterValue::Int(18),
        },
    ]);
    assert_eq!(filtering.filters.len(), 2);
    assert_eq!(filtering.sql, " WHERE age > 18 OR name = 'John'");
}

#[test]
fn test_filtering_with_multiple_rules_and_different_operators() {
    let filtering = Filtering::new(vec![
        FilteringRule {
            column: "name".to_string(),
            filter_operator: FilterOperator::Equal,
            conditional_operator: ConditionalOperator::And,
            value: FilterValue::String("John".to_string()),
        },
        FilteringRule {
            column: "age".to_string(),
            filter_operator: FilterOperator::GreaterThan,
            conditional_operator: ConditionalOperator::Or,
            value: FilterValue::Int(18),
        },
        FilteringRule {
            column: "city".to_string(),
            filter_operator: FilterOperator::Equal,
            conditional_operator: ConditionalOperator::And,
            value: FilterValue::String("New York".to_string()),
        },
    ]);
    assert_eq!(filtering.filters.len(), 3);
    assert_eq!(filtering.sql, " WHERE age > 18 AND city = 'New York' AND name = 'John'");
}

#[test]
fn test_filtering_with_multiple_rules_and_different_operators_swapped() {
    let filtering = Filtering::new(vec![
        FilteringRule {
            column: "name".to_string(),
            filter_operator: FilterOperator::Equal,
            conditional_operator: ConditionalOperator::Or,
            value: FilterValue::String("John".to_string()),
        },
        FilteringRule {
            column: "age".to_string(),
            filter_operator: FilterOperator::GreaterThan,
            conditional_operator: ConditionalOperator::And,
            value: FilterValue::Int(18),
        },
        FilteringRule {
            column: "city".to_string(),
            filter_operator: FilterOperator::Equal,
            conditional_operator: ConditionalOperator::Or,
            value: FilterValue::String("New York".to_string()),
        },
    ]);
    assert_eq!(filtering.filters.len(), 3);
    assert_eq!(filtering.sql, " WHERE age > 18 OR city = 'New York' OR name = 'John'");
}

#[test]
fn test_filtering_with_multiple_rules_and_different_operators_swapped_and_mixed() {
    let filtering = Filtering::new(vec![
        FilteringRule {
            column: "name".to_string(),
            filter_operator: FilterOperator::Equal,
            conditional_operator: ConditionalOperator::And,
            value: FilterValue::String("John".to_string()),
        },
        FilteringRule {
            column: "age".to_string(),
            filter_operator: FilterOperator::GreaterThan,
            conditional_operator: ConditionalOperator::And,
            value: FilterValue::Int(18),
        },
        FilteringRule {
            column: "city".to_string(),
            filter_operator: FilterOperator::Equal,
            conditional_operator: ConditionalOperator::Or,
            value: FilterValue::String("New York".to_string()),
        },
    ]);
    assert_eq!(filtering.filters.len(), 3);
    assert_eq!(filtering.sql, " WHERE age > 18 OR city = 'New York' AND name = 'John'");
}

#[test]
fn test_filtering_with_equal_to_rule() {
    let filtering = Filtering::new(vec![FilteringRule {
        column: "name".to_string(),
        filter_operator: FilterOperator::Equal,
        conditional_operator: ConditionalOperator::And,
        value: FilterValue::String("John".to_string()),
    }]);
    assert_eq!(filtering.filters.len(), 1);
    assert_eq!(filtering.sql, " WHERE name = 'John'");
}

#[test]
fn test_filtering_with_not_equal_to_rule() {
    let filtering = Filtering::new(vec![FilteringRule {
        column: "name".to_string(),
        filter_operator: FilterOperator::NotEqual,
        conditional_operator: ConditionalOperator::And,
        value: FilterValue::String("John".to_string()),
    }]);
    assert_eq!(filtering.filters.len(), 1);
    assert_eq!(filtering.sql, " WHERE name != 'John'");
}

#[test]
fn test_filtering_with_greater_than_rule() {
    let filtering = Filtering::new(vec![FilteringRule {
        column: "age".to_string(),
        filter_operator: FilterOperator::GreaterThan,
        conditional_operator: ConditionalOperator::And,
        value: FilterValue::Int(18),
    }]);
    assert_eq!(filtering.filters.len(), 1);
    assert_eq!(filtering.sql, " WHERE age > 18");
}

#[test]
fn test_filtering_with_greater_than_or_equal_to_rule() {
    let filtering = Filtering::new(vec![FilteringRule {
        column: "age".to_string(),
        filter_operator: FilterOperator::GreaterThanOrEqual,
        conditional_operator: ConditionalOperator::And,
        value: FilterValue::Int(18),
    }]);
    assert_eq!(filtering.filters.len(), 1);
    assert_eq!(filtering.sql, " WHERE age >= 18");
}

#[test]
fn test_filtering_with_less_than_rule() {
    let filtering = Filtering::new(vec![FilteringRule {
        column: "age".to_string(),
        filter_operator: FilterOperator::LessThan,
        conditional_operator: ConditionalOperator::And,
        value: FilterValue::Int(18),
    }]);
    assert_eq!(filtering.filters.len(), 1);
    assert_eq!(filtering.sql, " WHERE age < 18");
}

#[test]
fn test_filtering_with_less_than_or_equal_to_rule() {
    let filtering = Filtering::new(vec![FilteringRule {
        column: "age".to_string(),
        filter_operator: FilterOperator::LessThanOrEqual,
        conditional_operator: ConditionalOperator::And,
        value: FilterValue::Int(18),
    }]);
    assert_eq!(filtering.filters.len(), 1);
    assert_eq!(filtering.sql, " WHERE age <= 18");
}

#[test]
fn test_filtering_with_like_rule() {
    let filtering = Filtering::new(vec![FilteringRule {
        column: "name".to_string(),
        filter_operator: FilterOperator::Like,
        conditional_operator: ConditionalOperator::And,
        value: FilterValue::String("John".to_string()),
    }]);
    assert_eq!(filtering.filters.len(), 1);
    assert_eq!(filtering.sql, " WHERE name LIKE 'John'");
}

#[test]
fn test_filtering_with_not_like_rule() {
    let filtering = Filtering::new(vec![FilteringRule {
        column: "name".to_string(),
        filter_operator: FilterOperator::NotLike,
        conditional_operator: ConditionalOperator::And,
        value: FilterValue::String("John".to_string()),
    }]);
    assert_eq!(filtering.filters.len(), 1);
    assert_eq!(filtering.sql, " WHERE name NOT LIKE 'John'");
}

#[test]
fn test_filtering_with_is_null() {
    let filtering = Filtering::new(vec![FilteringRule {
        column: "name".to_string(),
        filter_operator: FilterOperator::IsNull,
        conditional_operator: ConditionalOperator::And,
        value: FilterValue::String("".to_string()),
    }]);
    assert_eq!(filtering.filters.len(), 1);
    assert_eq!(filtering.sql, " WHERE name IS NULL ");
}

#[test]
fn test_filtering_with_is_not_null() {
    let filtering = Filtering::new(vec![FilteringRule {
        column: "name".to_string(),
        filter_operator: FilterOperator::IsNotNull,
        conditional_operator: ConditionalOperator::And,
        value: FilterValue::String("".to_string()),
    }]);
    assert_eq!(filtering.filters.len(), 1);
    assert_eq!(filtering.sql, " WHERE name IS NOT NULL ");
}

#[test]
fn test_filtering_with_in() {
    let filtering = Filtering::new(vec![FilteringRule {
        column: "name".to_string(),
        filter_operator: FilterOperator::In,
        conditional_operator: ConditionalOperator::And,
        value: FilterValue::String("('John', 'Doe')".to_string()),
    }]);
    assert_eq!(filtering.filters.len(), 1);
    assert_eq!(filtering.sql, " WHERE name IN ('John', 'Doe')");
}

#[test]
fn test_filtering_with_in_as_int() {
    let filtering = Filtering::new(vec![FilteringRule {
        column: "age".to_string(),
        filter_operator: FilterOperator::In,
        conditional_operator: ConditionalOperator::And,
        value: FilterValue::String("(21, 22, 23)".to_string()),
    }]);
    assert_eq!(filtering.filters.len(), 1);
    assert_eq!(filtering.sql, " WHERE age IN (21, 22, 23)");
}

#[test]
fn test_filtering_with_not_in() {
    let filtering = Filtering::new(vec![FilteringRule {
        column: "name".to_string(),
        filter_operator: FilterOperator::NotIn,
        conditional_operator: ConditionalOperator::And,
        value: FilterValue::String("('John', 'Doe')".to_string()),
    }]);
    assert_eq!(filtering.filters.len(), 1);
    assert_eq!(filtering.sql, " WHERE name NOT IN ('John', 'Doe')");
}

#[test]
fn test_filtering_with_multiple_rules_and_different_operators_and_values() {
    let filtering = Filtering::new(vec![
        FilteringRule {
            column: "name".to_string(),
            filter_operator: FilterOperator::Equal,
            conditional_operator: ConditionalOperator::And,
            value: FilterValue::String("John".to_string()),
        },
        FilteringRule {
            column: "age".to_string(),
            filter_operator: FilterOperator::GreaterThan,
            conditional_operator: ConditionalOperator::Or,
            value: FilterValue::Int(18),
        },
        FilteringRule {
            column: "city".to_string(),
            filter_operator: FilterOperator::Like,
            conditional_operator: ConditionalOperator::And,
            value: FilterValue::String("New York".to_string()),
        },
    ]);
    assert_eq!(filtering.filters.len(), 3);
    assert_eq!(filtering.sql, " WHERE age > 18 AND city LIKE 'New York' AND name = 'John'");
}

#[test]
fn test_filtering_with_multiple_rules_and_different_operators_and_values_swapped() {
    let filtering = Filtering::new(vec![
        FilteringRule {
            column: "name".to_string(),
            filter_operator: FilterOperator::Equal,
            conditional_operator: ConditionalOperator::Or,
            value: FilterValue::String("John".to_string()),
        },
        FilteringRule {
            column: "age".to_string(),
            filter_operator: FilterOperator::GreaterThan,
            conditional_operator: ConditionalOperator::And,
            value: FilterValue::Int(18),
        },
        FilteringRule {
            column: "city".to_string(),
            filter_operator: FilterOperator::Like,
            conditional_operator: ConditionalOperator::Or,
            value: FilterValue::String("New York".to_string()),
        },
    ]);
    assert_eq!(filtering.filters.len(), 3);
    assert_eq!(filtering.sql, " WHERE age > 18 OR city LIKE 'New York' OR name = 'John'");
}

#[test]
fn test_filtering_with_multiple_rules_and_different_operators_and_values_swapped_and_mixed() {
    let filtering = Filtering::new(vec![
        FilteringRule {
            column: "name".to_string(),
            filter_operator: FilterOperator::Equal,
            conditional_operator: ConditionalOperator::And,
            value: FilterValue::String("John".to_string()),
        },
        FilteringRule {
            column: "age".to_string(),
            filter_operator: FilterOperator::GreaterThan,
            conditional_operator: ConditionalOperator::And,
            value: FilterValue::Int(18),
        },
        FilteringRule {
            column: "city".to_string(),
            filter_operator: FilterOperator::Like,
            conditional_operator: ConditionalOperator::Or,
            value: FilterValue::String("New York".to_string()),
        },
    ]);
    assert_eq!(filtering.filters.len(), 3);
    assert_eq!(filtering.sql, " WHERE age > 18 OR city LIKE 'New York' AND name = 'John'");
}

#[test]
fn test_filtering_with_multiple_rules_and_different_operators_and_values_swapped_and_mixed_and_repeated() {
    let filtering = Filtering::new(vec![
        FilteringRule {
            column: "name".to_string(),
            filter_operator: FilterOperator::Equal,
            conditional_operator: ConditionalOperator::And,
            value: FilterValue::String("John".to_string()),
        },
        FilteringRule {
            column: "age".to_string(),
            filter_operator: FilterOperator::GreaterThan,
            conditional_operator: ConditionalOperator::And,
            value: FilterValue::Int(18),
        },
        FilteringRule {
            column: "city".to_string(),
            filter_operator: FilterOperator::Like,
            conditional_operator: ConditionalOperator::Or,
            value: FilterValue::String("New York".to_string()),
        },
        FilteringRule {
            column: "name".to_string(),
            filter_operator: FilterOperator::Equal,
            conditional_operator: ConditionalOperator::Or,
            value: FilterValue::String("Doe".to_string()),
        },
    ]);
    assert_eq!(filtering.filters.len(), 3);
    assert_eq!(filtering.sql, " WHERE age > 18 OR city LIKE 'New York' AND name = 'John'");
}

#[test]
fn test_filtering_with_many_rules_and_conditions_with_no_duplicates_with_or_and_and() {
    let filtering = Filtering::new(vec![
        FilteringRule {
            column: "name".to_string(),
            filter_operator: FilterOperator::Equal,
            conditional_operator: ConditionalOperator::Or,
            value: FilterValue::String("John".to_string()),
        },
        FilteringRule {
            column: "age".to_string(),
            filter_operator: FilterOperator::GreaterThan,
            conditional_operator: ConditionalOperator::And,
            value: FilterValue::Int(18),
        },
        FilteringRule {
            column: "city".to_string(),
            filter_operator: FilterOperator::Like,
            conditional_operator: ConditionalOperator::Or,
            value: FilterValue::String("New York".to_string()),
        },
        FilteringRule {
            column: "name".to_string(),
            filter_operator: FilterOperator::Equal,
            conditional_operator: ConditionalOperator::And,
            value: FilterValue::String("Doe".to_string()),
        },
    ]);
    assert_eq!(filtering.filters.len(), 3);
    assert_eq!(filtering.sql, " WHERE age > 18 OR city LIKE 'New York' OR name = 'John'");
  
}