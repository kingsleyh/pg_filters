use pg_filters::{filtering::{
    ColumnName, ConditionalOperator, FilterOperator, FilterValue, Filtering, FilteringRule
}, FilteringOptions};

#[test]
fn test_filtering() {
    let filtering = Filtering::new(vec![
        FilteringRule {
            column: ColumnName::String("name"),
            filter_operator: FilterOperator::Equal,
            conditional_operator: ConditionalOperator::And,
            value: FilterValue::String("John".to_string()),
        },
        FilteringRule {
            column: ColumnName::Int("age"),
            filter_operator: FilterOperator::GreaterThan,
            conditional_operator: ConditionalOperator::Or,
            value: FilterValue::Int(18),
        },
    ],  true);
    assert_eq!(filtering.filters.len(), 2);
    assert_eq!(filtering.sql, " WHERE name = 'John' OR age > 18");
}

#[test]
fn test_case_sensitive_filtering() {
    let filtering = Filtering::new(vec![
        FilteringRule {
            column: ColumnName::String("name"),
            filter_operator: FilterOperator::Equal,
            conditional_operator: ConditionalOperator::And,
            value: FilterValue::String("John".to_string()),
        },
        FilteringRule {
            column: ColumnName::Int("age"),
            filter_operator: FilterOperator::GreaterThan,
            conditional_operator: ConditionalOperator::Or,
            value: FilterValue::Int(18),
        },
    ], false);
    assert_eq!(filtering.filters.len(), 2);
    assert_eq!(filtering.sql, " WHERE name = 'John' OR age > 18");
}

#[test]
fn test_filtering_with_bool() {
    let filtering = Filtering::new(vec![
        FilteringRule {
            column: ColumnName::String("name"),
            filter_operator: FilterOperator::Equal,
            conditional_operator: ConditionalOperator::And,
            value: FilterValue::String("John".to_string()),
        },
        FilteringRule {
            column: ColumnName::Bool("completed"),
            filter_operator: FilterOperator::Equal,
            conditional_operator: ConditionalOperator::Or,
            value: FilterValue::Bool(true),
        },
    ], true);
    assert_eq!(filtering.filters.len(), 2);
    assert_eq!(filtering.sql, " WHERE name = 'John' OR completed = true");
}

#[test]
fn test_filtering_with_float() {
    let filtering = Filtering::new(vec![
        FilteringRule {
            column: ColumnName::String("name"),
            filter_operator: FilterOperator::Equal,
            conditional_operator: ConditionalOperator::And,
            value: FilterValue::String("John".to_string()),
        },
        FilteringRule {
            column: ColumnName::Float("value"),
            filter_operator: FilterOperator::LessThanOrEqual,
            conditional_operator: ConditionalOperator::Or,
            value: FilterValue::Float(1.1),
        },
    ], true);
    assert_eq!(filtering.filters.len(), 2);
    assert_eq!(filtering.sql, " WHERE name = 'John' OR value <= 1.1");
}

#[test]
fn test_filtering_with_duplicate_columns() {
    let filtering = Filtering::new(vec![
        FilteringRule {
            column: ColumnName::String("name"),
            filter_operator: FilterOperator::Equal,
            conditional_operator: ConditionalOperator::And,
            value: FilterValue::String("John".to_string()),
        },
        FilteringRule {
            column: ColumnName::String("name"),
            filter_operator: FilterOperator::Equal,
            conditional_operator: ConditionalOperator::Or,
            value: FilterValue::String("Doe".to_string()),
        },
        ], true);
    assert_eq!(filtering.filters.len(), 2);
    assert_eq!(filtering.sql, " WHERE name = 'John' OR name = 'Doe'");
}

#[test]
fn test_filtering_with_single_rule() {
    let filtering = Filtering::new(vec![FilteringRule {
        column: ColumnName::String("name"),
        filter_operator: FilterOperator::Equal,
        conditional_operator: ConditionalOperator::And,
        value: FilterValue::String("John".to_string()),
    } ], true);
    assert_eq!(filtering.filters.len(), 1);
    assert_eq!(filtering.sql, " WHERE name = 'John'");
}

#[test]
fn test_filtering_with_empty_rules() {
    let filtering = Filtering::new(vec![], true);
    assert_eq!(filtering.filters.len(), 0);
    assert_eq!(filtering.sql, "");
}

#[test]
fn test_filtering_with_multiple_rules() {
    let filtering = Filtering::new(vec![
        FilteringRule {
            column: ColumnName::String("name"),
            filter_operator: FilterOperator::Equal,
            conditional_operator: ConditionalOperator::And,
            value: FilterValue::String("John".to_string()),
        },
        FilteringRule {
            column: ColumnName::Int("age"),
            filter_operator: FilterOperator::GreaterThan,
            conditional_operator: ConditionalOperator::Or,
            value: FilterValue::Int(18),
        },
        ], true);
    assert_eq!(filtering.filters.len(), 2);
    assert_eq!(filtering.sql, " WHERE name = 'John' OR age > 18");
}

#[test]
fn test_filtering_with_multiple_rules_swapped() {
    let filtering = Filtering::new(vec![
        FilteringRule {
            column: ColumnName::String("name"),
            filter_operator: FilterOperator::Equal,
            conditional_operator: ConditionalOperator::Or,
            value: FilterValue::String("John".to_string()),
        },
        FilteringRule {
            column: ColumnName::Int("age"),
            filter_operator: FilterOperator::GreaterThan,
            conditional_operator: ConditionalOperator::And,
            value: FilterValue::Int(18),
        },
        ], true);
    assert_eq!(filtering.filters.len(), 2);
    assert_eq!(filtering.sql, " WHERE name = 'John' AND age > 18");
}

#[test]
fn test_filtering_with_multiple_rules_and_different_operators() {
    let filtering = Filtering::new(vec![
        FilteringRule {
            column: ColumnName::String("name"),
            filter_operator: FilterOperator::Equal,
            conditional_operator: ConditionalOperator::And,
            value: FilterValue::String("John".to_string()),
        },
        FilteringRule {
            column: ColumnName::Int("age"),
            filter_operator: FilterOperator::GreaterThan,
            conditional_operator: ConditionalOperator::Or,
            value: FilterValue::Int(18),
        },
        FilteringRule {
           column: ColumnName::String("city"),
            filter_operator: FilterOperator::Equal,
            conditional_operator: ConditionalOperator::And,
            value: FilterValue::String("New York".to_string()),
        },
        ], true);
    assert_eq!(filtering.filters.len(), 3);
    assert_eq!(
        filtering.sql,
        " WHERE name = 'John' OR age > 18 AND city = 'New York'"
    );
}

#[test]
fn test_filtering_with_multiple_rules_and_different_operators_swapped() {
    let filtering = Filtering::new(vec![
        FilteringRule {
            column: ColumnName::String("name"),
            filter_operator: FilterOperator::Equal,
            conditional_operator: ConditionalOperator::Or,
            value: FilterValue::String("John".to_string()),
        },
        FilteringRule {
            column: ColumnName::Int("age"),
            filter_operator: FilterOperator::GreaterThan,
            conditional_operator: ConditionalOperator::And,
            value: FilterValue::Int(18),
        },
        FilteringRule {
           column: ColumnName::String("city"),
            filter_operator: FilterOperator::Equal,
            conditional_operator: ConditionalOperator::Or,
            value: FilterValue::String("New York".to_string()),
        },
        ], true);
    assert_eq!(filtering.filters.len(), 3);
    assert_eq!(
        filtering.sql,
        " WHERE name = 'John' AND age > 18 OR city = 'New York'"
    );
}

#[test]
fn test_filtering_with_multiple_rules_and_different_operators_swapped_and_mixed() {
    let filtering = Filtering::new(vec![
        FilteringRule {
            column: ColumnName::String("name"),
            filter_operator: FilterOperator::Equal,
            conditional_operator: ConditionalOperator::And,
            value: FilterValue::String("John".to_string()),
        },
        FilteringRule {
            column: ColumnName::Int("age"),
            filter_operator: FilterOperator::GreaterThan,
            conditional_operator: ConditionalOperator::And,
            value: FilterValue::Int(18),
        },
        FilteringRule {
           column: ColumnName::String("city"),
            filter_operator: FilterOperator::Equal,
            conditional_operator: ConditionalOperator::Or,
            value: FilterValue::String("New York".to_string()),
        },
        ], true);
    assert_eq!(filtering.filters.len(), 3);
    assert_eq!(
        filtering.sql,
        " WHERE name = 'John' AND age > 18 OR city = 'New York'"
    );
}

#[test]
fn test_filtering_with_equal_to_rule() {
    let filtering = Filtering::new(vec![FilteringRule {
        column: ColumnName::String("name"),
        filter_operator: FilterOperator::Equal,
        conditional_operator: ConditionalOperator::And,
        value: FilterValue::String("John".to_string()),
    } ], true);
    assert_eq!(filtering.filters.len(), 1);
    assert_eq!(filtering.sql, " WHERE name = 'John'");
}

#[test]
fn test_filtering_with_not_equal_to_rule() {
    let filtering = Filtering::new(vec![FilteringRule {
        column: ColumnName::String("name"),
        filter_operator: FilterOperator::NotEqual,
        conditional_operator: ConditionalOperator::And,
        value: FilterValue::String("John".to_string()),
    } ], true);
    assert_eq!(filtering.filters.len(), 1);
    assert_eq!(filtering.sql, " WHERE name != 'John'");
}

#[test]
fn test_filtering_with_greater_than_rule() {
    let filtering = Filtering::new(vec![FilteringRule {
        column: ColumnName::Int("age"),
        filter_operator: FilterOperator::GreaterThan,
        conditional_operator: ConditionalOperator::And,
        value: FilterValue::Int(18),
    } ], true);
    assert_eq!(filtering.filters.len(), 1);
    assert_eq!(filtering.sql, " WHERE age > 18");
}

#[test]
fn test_filtering_with_greater_than_or_equal_to_rule() {
    let filtering = Filtering::new(vec![FilteringRule {
        column: ColumnName::Int("age"),
        filter_operator: FilterOperator::GreaterThanOrEqual,
        conditional_operator: ConditionalOperator::And,
        value: FilterValue::Int(18),
    } ], true);
    assert_eq!(filtering.filters.len(), 1);
    assert_eq!(filtering.sql, " WHERE age >= 18");
}

#[test]
fn test_filtering_with_less_than_rule() {
    let filtering = Filtering::new(vec![FilteringRule {
        column: ColumnName::Int("age"),
        filter_operator: FilterOperator::LessThan,
        conditional_operator: ConditionalOperator::And,
        value: FilterValue::Int(18),
    } ], true);
    assert_eq!(filtering.filters.len(), 1);
    assert_eq!(filtering.sql, " WHERE age < 18");
}

#[test]
fn test_filtering_with_less_than_or_equal_to_rule() {
    let filtering = Filtering::new(vec![FilteringRule {
        column: ColumnName::Int("age"),
        filter_operator: FilterOperator::LessThanOrEqual,
        conditional_operator: ConditionalOperator::And,
        value: FilterValue::Int(18),
    } ], true);
    assert_eq!(filtering.filters.len(), 1);
    assert_eq!(filtering.sql, " WHERE age <= 18");
}

#[test]
fn test_filtering_with_like_rule() {
    let filtering = Filtering::new(vec![FilteringRule {
        column: ColumnName::String("name"),
        filter_operator: FilterOperator::Like,
        conditional_operator: ConditionalOperator::And,
        value: FilterValue::String("John".to_string()),
    } ], true);
    assert_eq!(filtering.filters.len(), 1);
    assert_eq!(filtering.sql, " WHERE name LIKE '%John%'");
}

#[test]
fn test_filtering_with_not_like_rule() {
    let filtering = Filtering::new(vec![FilteringRule {
        column: ColumnName::String("name"),
        filter_operator: FilterOperator::NotLike,
        conditional_operator: ConditionalOperator::And,
        value: FilterValue::String("John".to_string()),
    } ], true);
    assert_eq!(filtering.filters.len(), 1);
    assert_eq!(filtering.sql, " WHERE name NOT LIKE '%John%'");
}

#[test]
fn test_filtering_with_is_null() {
    let filtering = Filtering::new(vec![FilteringRule {
        column: ColumnName::String("name"),
        filter_operator: FilterOperator::IsNull,
        conditional_operator: ConditionalOperator::And,
        value: FilterValue::String("".to_string()),
    } ], true);
    assert_eq!(filtering.filters.len(), 1);
    assert_eq!(filtering.sql, " WHERE name IS NULL ");
}

#[test]
fn test_filtering_with_is_not_null() {
    let filtering = Filtering::new(vec![FilteringRule {
        column: ColumnName::String("name"),
        filter_operator: FilterOperator::IsNotNull,
        conditional_operator: ConditionalOperator::And,
        value: FilterValue::String("".to_string()),
    } ], true);
    assert_eq!(filtering.filters.len(), 1);
    assert_eq!(filtering.sql, " WHERE name IS NOT NULL ");
}

#[test]
fn test_filtering_with_in() {
    let filtering = Filtering::new(vec![FilteringRule {
        column: ColumnName::String("name"),
        filter_operator: FilterOperator::In,
        conditional_operator: ConditionalOperator::And,
        value: FilterValue::String("('John', 'Doe')".to_string()),
    } ], true);
    assert_eq!(filtering.filters.len(), 1);
    assert_eq!(filtering.sql, " WHERE name IN ('John', 'Doe')");
}

#[test]
fn test_filtering_with_in_as_int() {
    let filtering = Filtering::new(vec![FilteringRule {
        column: ColumnName::Int("age"),
        filter_operator: FilterOperator::In,
        conditional_operator: ConditionalOperator::And,
        value: FilterValue::String("(21, 22, 23)".to_string()),
    } ], true);
    assert_eq!(filtering.filters.len(), 1);
    assert_eq!(filtering.sql, " WHERE age IN (21, 22, 23)");
}

#[test]
fn test_filtering_with_not_in() {
    let filtering = Filtering::new(vec![FilteringRule {
        column: ColumnName::String("name"),
        filter_operator: FilterOperator::NotIn,
        conditional_operator: ConditionalOperator::And,
        value: FilterValue::String("('John', 'Doe')".to_string()),
    }], true);
    assert_eq!(filtering.filters.len(), 1);
    assert_eq!(filtering.sql, " WHERE name NOT IN ('John', 'Doe')");
}

#[test]
fn test_filtering_with_starts_with() {
    let filtering = Filtering::new(vec![FilteringRule {
        column: ColumnName::String("name"),
        filter_operator: FilterOperator::StartsWith,
        conditional_operator: ConditionalOperator::And,
        value: FilterValue::String("John".to_string()),
    }], true);
    assert_eq!(filtering.filters.len(), 1);
    assert_eq!(filtering.sql, " WHERE name LIKE 'John%'");
}

#[test]
fn test_filtering_with_ends_with() {
    let filtering = Filtering::new(vec![FilteringRule {
        column: ColumnName::String("name"),
        filter_operator: FilterOperator::EndsWith,
        conditional_operator: ConditionalOperator::And,
        value: FilterValue::String("John".to_string()),
    }], true);
    assert_eq!(filtering.filters.len(), 1);
    assert_eq!(filtering.sql, " WHERE name LIKE '%John'");
}

#[test]
fn test_filtering_with_multiple_rules_and_different_operators_and_values() {
    let filtering = Filtering::new(vec![
        FilteringRule {
            column: ColumnName::String("name"),
            filter_operator: FilterOperator::Equal,
            conditional_operator: ConditionalOperator::And,
            value: FilterValue::String("John".to_string()),
        },
        FilteringRule {
            column: ColumnName::Int("age"),
            filter_operator: FilterOperator::GreaterThan,
            conditional_operator: ConditionalOperator::Or,
            value: FilterValue::Int(18),
        },
        FilteringRule {
           column: ColumnName::String("city"),
            filter_operator: FilterOperator::Like,
            conditional_operator: ConditionalOperator::And,
            value: FilterValue::String("New York".to_string()),
        },
    ], true);
    assert_eq!(filtering.filters.len(), 3);
    assert_eq!(
        filtering.sql,
        " WHERE name = 'John' OR age > 18 AND city LIKE '%New York%'"
    );
}

#[test]
fn test_filtering_with_multiple_rules_and_different_operators_and_values_swapped() {
    let filtering = Filtering::new(vec![
        FilteringRule {
            column: ColumnName::String("name"),
            filter_operator: FilterOperator::Equal,
            conditional_operator: ConditionalOperator::Or,
            value: FilterValue::String("John".to_string()),
        },
        FilteringRule {
            column: ColumnName::Int("age"),
            filter_operator: FilterOperator::GreaterThan,
            conditional_operator: ConditionalOperator::And,
            value: FilterValue::Int(18),
        },
        FilteringRule {
           column: ColumnName::String("city"),
            filter_operator: FilterOperator::Like,
            conditional_operator: ConditionalOperator::Or,
            value: FilterValue::String("New York".to_string()),
        },
    ], true);
    assert_eq!(filtering.filters.len(), 3);
    assert_eq!(
        filtering.sql,
        " WHERE name = 'John' AND age > 18 OR city LIKE '%New York%'"
    );
}

#[test]
fn test_filtering_with_multiple_rules_and_different_operators_and_values_swapped_and_mixed() {
    let filtering = Filtering::new(vec![
        FilteringRule {
            column: ColumnName::String("name"),
            filter_operator: FilterOperator::Equal,
            conditional_operator: ConditionalOperator::And,
            value: FilterValue::String("John".to_string()),
        },
        FilteringRule {
            column: ColumnName::Int("age"),
            filter_operator: FilterOperator::GreaterThan,
            conditional_operator: ConditionalOperator::And,
            value: FilterValue::Int(18),
        },
        FilteringRule {
           column: ColumnName::String("city"),
            filter_operator: FilterOperator::Like,
            conditional_operator: ConditionalOperator::Or,
            value: FilterValue::String("New York".to_string()),
        },
        ], true);
    assert_eq!(filtering.filters.len(), 3);
    assert_eq!(
        filtering.sql,
        " WHERE name = 'John' AND age > 18 OR city LIKE '%New York%'"
    );
}

#[test]
fn test_filtering_with_multiple_rules_and_different_operators_and_values_swapped_and_mixed_and_repeated(
) {
    let filtering = Filtering::new(vec![
        FilteringRule {
            column: ColumnName::String("name"),
            filter_operator: FilterOperator::Equal,
            conditional_operator: ConditionalOperator::And,
            value: FilterValue::String("John".to_string()),
        },
        FilteringRule {
            column: ColumnName::Int("age"),
            filter_operator: FilterOperator::GreaterThan,
            conditional_operator: ConditionalOperator::And,
            value: FilterValue::Int(18),
        },
        FilteringRule {
           column: ColumnName::String("city"),
            filter_operator: FilterOperator::Like,
            conditional_operator: ConditionalOperator::Or,
            value: FilterValue::String("New York".to_string()),
        },
        FilteringRule {
            column: ColumnName::String("name"),
            filter_operator: FilterOperator::Equal,
            conditional_operator: ConditionalOperator::Or,
            value: FilterValue::String("Doe".to_string()),
        },
        ], true);
    assert_eq!(filtering.filters.len(), 4);
    assert_eq!(
        filtering.sql,
        " WHERE name = 'John' AND age > 18 OR city LIKE '%New York%' OR name = 'Doe'"
    );
}

#[test]
fn test_filtering_with_many_rules_and_conditions_with_no_duplicates_with_or_and_and() {
    let filtering = Filtering::new(vec![
        FilteringRule {
            column: ColumnName::String("name"),
            filter_operator: FilterOperator::Equal,
            conditional_operator: ConditionalOperator::Or,
            value: FilterValue::String("John".to_string()),
        },
        FilteringRule {
            column: ColumnName::Int("age"),
            filter_operator: FilterOperator::GreaterThan,
            conditional_operator: ConditionalOperator::And,
            value: FilterValue::Int(18),
        },
        FilteringRule {
           column: ColumnName::String("city"),
            filter_operator: FilterOperator::Like,
            conditional_operator: ConditionalOperator::Or,
            value: FilterValue::String("New York".to_string()),
        },
        FilteringRule {
            column: ColumnName::String("name"),
            filter_operator: FilterOperator::Equal,
            conditional_operator: ConditionalOperator::And,
            value: FilterValue::String("Doe".to_string()),
        },
        ], true);
    assert_eq!(filtering.filters.len(), 4);
    assert_eq!(
        filtering.sql,
        " WHERE name = 'John' AND age > 18 OR city LIKE '%New York%' AND name = 'Doe'"
    );
}

#[test]
fn test_filtering_options_case_insensitive() {
   let filtering_options = FilteringOptions::new(
    vec![
        FilteringRule {
            column: ColumnName::String("name"),
            filter_operator: FilterOperator::Equal,
            conditional_operator: ConditionalOperator::Or,
            value: FilterValue::String("John".to_string()),
        },]
   );

    assert_eq!(filtering_options.filtering_rules.len(), 1);
    assert_eq!(filtering_options.case_sensitive, false);
}

#[test]
fn test_filtering_options_case_sensitive() {
   let filtering_options = FilteringOptions::case_sensitive(
    vec![
        FilteringRule {
            column: ColumnName::String("name"),
            filter_operator: FilterOperator::Equal,
            conditional_operator: ConditionalOperator::Or,
            value: FilterValue::String("John".to_string()),
        },]
   );

    assert_eq!(filtering_options.filtering_rules.len(), 1);
    assert_eq!(filtering_options.case_sensitive, true);
}