use pg_filters::filtering::FilterColumn;
use pg_filters::{
    filtering::{ConditionalOperator, FilterOperator, Filtering, FilteringRule},
    FilteringOptions,
};

#[test]
fn test_filtering() {
    let filtering = Filtering::new(
        &[
            Ok(FilteringRule {
                filter_column: FilterColumn::String("name", "'John'".to_string()),
                filter_operator: FilterOperator::Equal,
                conditional_operator: ConditionalOperator::And,
            }),
            Ok(FilteringRule {
                filter_column: FilterColumn::Int("age", 18),
                filter_operator: FilterOperator::GreaterThan,
                conditional_operator: ConditionalOperator::Or,
            }),
        ],
        true,
    );
    assert_eq!(filtering.filters.len(), 2);
    assert_eq!(
        filtering.sql,
        " WHERE LOWER(name) = LOWER('John') OR age > 18"
    );
}

#[test]
fn test_filtering_case_sensitive() {
    let filtering = Filtering::new(
        &[
            Ok(FilteringRule {
                filter_column: FilterColumn::String("name", "'John'".to_string()),
                filter_operator: FilterOperator::Equal,
                conditional_operator: ConditionalOperator::And,
            }),
            Ok(FilteringRule {
                filter_column: FilterColumn::Int("age", 18),
                filter_operator: FilterOperator::GreaterThan,
                conditional_operator: ConditionalOperator::Or,
            }),
        ],
        false,
    );
    assert_eq!(filtering.filters.len(), 2);
    assert_eq!(filtering.sql, " WHERE name = 'John' OR age > 18");
}

#[test]
fn test_filtering_with_bool() {
    let filtering = Filtering::new(
        &[
            Ok(FilteringRule {
                filter_column: FilterColumn::String("name", "'John'".to_string()),
                filter_operator: FilterOperator::Equal,
                conditional_operator: ConditionalOperator::And,
            }),
            Ok(FilteringRule {
                filter_column: FilterColumn::Bool("completed", true),
                filter_operator: FilterOperator::Equal,
                conditional_operator: ConditionalOperator::Or,
            }),
        ],
        true,
    );
    assert_eq!(filtering.filters.len(), 2);
    assert_eq!(
        filtering.sql,
        " WHERE LOWER(name) = LOWER('John') OR completed = true"
    );
}

#[test]
fn test_filtering_with_float() {
    let filtering = Filtering::new(
        &[
            Ok(FilteringRule {
                filter_column: FilterColumn::String("name", "'John'".to_string()),
                filter_operator: FilterOperator::Equal,
                conditional_operator: ConditionalOperator::And,
            }),
            Ok(FilteringRule {
                filter_column: FilterColumn::Float("value", 1.1),
                filter_operator: FilterOperator::LessThanOrEqual,
                conditional_operator: ConditionalOperator::Or,
            }),
        ],
        true,
    );
    assert_eq!(filtering.filters.len(), 2);
    assert_eq!(
        filtering.sql,
        " WHERE LOWER(name) = LOWER('John') OR value <= 1.1"
    );
}

#[test]
fn test_filtering_with_duplicate_columns() {
    let filtering = Filtering::new(
        &[
            Ok(FilteringRule {
                filter_column: FilterColumn::String("name", "'John'".to_string()),
                filter_operator: FilterOperator::Equal,
                conditional_operator: ConditionalOperator::And,
            }),
            Ok(FilteringRule {
                filter_column: FilterColumn::String("name", "'Doe'".to_string()),
                filter_operator: FilterOperator::Equal,
                conditional_operator: ConditionalOperator::Or,
            }),
        ],
        true,
    );
    assert_eq!(filtering.filters.len(), 2);
    assert_eq!(
        filtering.sql,
        " WHERE LOWER(name) = LOWER('John') OR LOWER(name) = LOWER('Doe')"
    );
}

#[test]
fn test_filtering_with_single_rule() {
    let filtering = Filtering::new(
        &[Ok(FilteringRule {
            filter_column: FilterColumn::String("name", "'John'".to_string()),
            filter_operator: FilterOperator::Equal,
            conditional_operator: ConditionalOperator::And,
        })],
        true,
    );
    assert_eq!(filtering.filters.len(), 1);
    assert_eq!(filtering.sql, " WHERE LOWER(name) = LOWER('John')");
}

#[test]
fn test_filtering_with_empty_rules() {
    let filtering = Filtering::new(&[], true);
    assert_eq!(filtering.filters.len(), 0);
    assert_eq!(filtering.sql, "");
}

#[test]
fn test_filtering_with_multiple_rules() {
    let filtering = Filtering::new(
        &[
            Ok(FilteringRule {
                filter_column: FilterColumn::String("name", "'John'".to_string()),
                filter_operator: FilterOperator::Equal,
                conditional_operator: ConditionalOperator::And,
            }),
            Ok(FilteringRule {
                filter_column: FilterColumn::Int("age", 18),
                filter_operator: FilterOperator::GreaterThan,
                conditional_operator: ConditionalOperator::Or,
            }),
        ],
        true,
    );
    assert_eq!(filtering.filters.len(), 2);
    assert_eq!(
        filtering.sql,
        " WHERE LOWER(name) = LOWER('John') OR age > 18"
    );
}

#[test]
fn test_filtering_with_multiple_rules_swapped() {
    let filtering = Filtering::new(
        &[
            Ok(FilteringRule {
                filter_column: FilterColumn::String("name", "'John'".to_string()),
                filter_operator: FilterOperator::Equal,
                conditional_operator: ConditionalOperator::Or,
            }),
            Ok(FilteringRule {
                filter_column: FilterColumn::Int("age", 18),
                filter_operator: FilterOperator::GreaterThan,
                conditional_operator: ConditionalOperator::And,
            }),
        ],
        true,
    );
    assert_eq!(filtering.filters.len(), 2);
    assert_eq!(
        filtering.sql,
        " WHERE LOWER(name) = LOWER('John') AND age > 18"
    );
}

#[test]
fn test_filtering_with_multiple_rules_and_different_operators() {
    let filtering = Filtering::new(
        &[
            Ok(FilteringRule {
                filter_column: FilterColumn::String("name", "'John'".to_string()),
                filter_operator: FilterOperator::Equal,
                conditional_operator: ConditionalOperator::And,
            }),
            Ok(FilteringRule {
                filter_column: FilterColumn::Int("age", 18),
                filter_operator: FilterOperator::GreaterThan,
                conditional_operator: ConditionalOperator::Or,
            }),
            Ok(FilteringRule {
                filter_column: FilterColumn::String("city", "'New York'".to_string()),
                filter_operator: FilterOperator::Equal,
                conditional_operator: ConditionalOperator::And,
            }),
        ],
        true,
    );
    assert_eq!(filtering.filters.len(), 3);
    assert_eq!(
        filtering.sql,
        " WHERE LOWER(name) = LOWER('John') OR age > 18 AND LOWER(city) = LOWER('New York')"
    );
}

#[test]
fn test_filtering_with_multiple_rules_and_different_operators_swapped() {
    let filtering = Filtering::new(
        &[
            Ok(FilteringRule {
                filter_column: FilterColumn::String("name", "'John'".to_string()),
                filter_operator: FilterOperator::Equal,
                conditional_operator: ConditionalOperator::Or,
            }),
            Ok(FilteringRule {
                filter_column: FilterColumn::Int("age", 18),
                filter_operator: FilterOperator::GreaterThan,
                conditional_operator: ConditionalOperator::And,
            }),
            Ok(FilteringRule {
                filter_column: FilterColumn::String("city", "'New York'".to_string()),
                filter_operator: FilterOperator::Equal,
                conditional_operator: ConditionalOperator::Or,
            }),
        ],
        true,
    );
    assert_eq!(filtering.filters.len(), 3);
    assert_eq!(
        filtering.sql,
        " WHERE LOWER(name) = LOWER('John') AND age > 18 OR LOWER(city) = LOWER('New York')"
    );
}

#[test]
fn test_filtering_with_multiple_rules_and_different_operators_swapped_and_mixed() {
    let filtering = Filtering::new(
        &[
            Ok(FilteringRule {
                filter_column: FilterColumn::String("name", "'John'".to_string()),
                filter_operator: FilterOperator::Equal,
                conditional_operator: ConditionalOperator::And,
            }),
            Ok(FilteringRule {
                filter_column: FilterColumn::Int("age", 18),
                filter_operator: FilterOperator::GreaterThan,
                conditional_operator: ConditionalOperator::And,
            }),
            Ok(FilteringRule {
                filter_column: FilterColumn::String("city", "'New York'".to_string()),
                filter_operator: FilterOperator::Equal,
                conditional_operator: ConditionalOperator::Or,
            }),
        ],
        true,
    );
    assert_eq!(filtering.filters.len(), 3);
    assert_eq!(
        filtering.sql,
        " WHERE LOWER(name) = LOWER('John') AND age > 18 OR LOWER(city) = LOWER('New York')"
    );
}

#[test]
fn test_filtering_with_equal_to_rule() {
    let filtering = Filtering::new(
        &[Ok(FilteringRule {
            filter_column: FilterColumn::String("name", "'John'".to_string()),
            filter_operator: FilterOperator::Equal,
            conditional_operator: ConditionalOperator::And,
        })],
        true,
    );
    assert_eq!(filtering.filters.len(), 1);
    assert_eq!(filtering.sql, " WHERE LOWER(name) = LOWER('John')");
}

#[test]
fn test_filtering_with_not_equal_to_rule() {
    let filtering = Filtering::new(
        &[Ok(FilteringRule {
            filter_column: FilterColumn::String("name", "'John'".to_string()),
            filter_operator: FilterOperator::NotEqual,
            conditional_operator: ConditionalOperator::And,
        })],
        true,
    );
    assert_eq!(filtering.filters.len(), 1);
    assert_eq!(filtering.sql, " WHERE LOWER(name) != LOWER('John')");
}

#[test]
fn test_filtering_with_greater_than_rule() {
    let filtering = Filtering::new(
        &[Ok(FilteringRule {
            filter_column: FilterColumn::Int("age", 18),
            filter_operator: FilterOperator::GreaterThan,
            conditional_operator: ConditionalOperator::And,
        })],
        true,
    );
    assert_eq!(filtering.filters.len(), 1);
    assert_eq!(filtering.sql, " WHERE age > 18");
}

#[test]
fn test_filtering_with_greater_than_or_equal_to_rule() {
    let filtering = Filtering::new(
        &[Ok(FilteringRule {
            filter_column: FilterColumn::Int("age", 18),
            filter_operator: FilterOperator::GreaterThanOrEqual,
            conditional_operator: ConditionalOperator::And,
        })],
        true,
    );
    assert_eq!(filtering.filters.len(), 1);
    assert_eq!(filtering.sql, " WHERE age >= 18");
}

#[test]
fn test_filtering_with_less_than_rule() {
    let filtering = Filtering::new(
        &[Ok(FilteringRule {
            filter_column: FilterColumn::Int("age", 18),
            filter_operator: FilterOperator::LessThan,
            conditional_operator: ConditionalOperator::And,
        })],
        true,
    );
    assert_eq!(filtering.filters.len(), 1);
    assert_eq!(filtering.sql, " WHERE age < 18");
}

#[test]
fn test_filtering_with_less_than_or_equal_to_rule() {
    let filtering = Filtering::new(
        &[Ok(FilteringRule {
            filter_column: FilterColumn::Int("age", 18),
            filter_operator: FilterOperator::LessThanOrEqual,
            conditional_operator: ConditionalOperator::And,
        })],
        true,
    );
    assert_eq!(filtering.filters.len(), 1);
    assert_eq!(filtering.sql, " WHERE age <= 18");
}

#[test]
fn test_filtering_with_like_rule() {
    let filtering = Filtering::new(
        &[Ok(FilteringRule {
            filter_column: FilterColumn::String("name", "'%John%'".to_string()),
            filter_operator: FilterOperator::Like,
            conditional_operator: ConditionalOperator::And,
        })],
        true,
    );
    assert_eq!(filtering.filters.len(), 1);
    assert_eq!(filtering.sql, " WHERE LOWER(name) LIKE LOWER('%John%')");
}

#[test]
fn test_filtering_with_not_like_rule() {
    let filtering = Filtering::new(
        &[Ok(FilteringRule {
            filter_column: FilterColumn::String("name", "'%John%'".to_string()),
            filter_operator: FilterOperator::NotLike,
            conditional_operator: ConditionalOperator::And,
        })],
        true,
    );
    assert_eq!(filtering.filters.len(), 1);
    assert_eq!(filtering.sql, " WHERE LOWER(name) NOT LIKE LOWER('%John%')");
}

#[test]
fn test_filtering_with_is_null() {
    let filtering = Filtering::new(
        &[Ok(FilteringRule {
            filter_column: FilterColumn::String("name", "".to_string()),
            filter_operator: FilterOperator::IsNull,
            conditional_operator: ConditionalOperator::And,
        })],
        true,
    );
    assert_eq!(filtering.filters.len(), 1);
    assert_eq!(filtering.sql, " WHERE name IS NULL");
}

#[test]
fn test_filtering_with_is_not_null() {
    let filtering = Filtering::new(
        &[Ok(FilteringRule {
            filter_column: FilterColumn::String("name", "".to_string()),
            filter_operator: FilterOperator::IsNotNull,
            conditional_operator: ConditionalOperator::And,
        })],
        true,
    );
    assert_eq!(filtering.filters.len(), 1);
    assert_eq!(filtering.sql, " WHERE name IS NOT NULL");
}

#[test]
fn test_filtering_with_in_as_string() {
    let filtering = Filtering::new(
        &[Ok(FilteringRule {
            filter_column: FilterColumn::StringList(
                "name",
                vec!["John".to_string(), "Doe".to_string()],
            ),
            filter_operator: FilterOperator::In,
            conditional_operator: ConditionalOperator::And,
        })],
        true,
    );
    assert_eq!(filtering.filters.len(), 1);
    assert_eq!(
        filtering.sql,
        " WHERE LOWER(name) IN (LOWER('John'), LOWER('Doe'))"
    );
}

#[test]
fn test_filtering_with_in_as_int() {
    let filtering = Filtering::new(
        &[Ok(FilteringRule {
            filter_column: FilterColumn::IntList("age", vec![21, 22, 23]),
            filter_operator: FilterOperator::In,
            conditional_operator: ConditionalOperator::And,
        })],
        true,
    );
    assert_eq!(filtering.filters.len(), 1);
    assert_eq!(filtering.sql, " WHERE age IN (21, 22, 23)");
}

#[test]
fn test_filtering_with_in_as_float() {
    let filtering = Filtering::new(
        &[Ok(FilteringRule {
            filter_column: FilterColumn::FloatList("age", vec![2.1, 2.2, 2.3]),
            filter_operator: FilterOperator::In,
            conditional_operator: ConditionalOperator::And,
        })],
        true,
    );
    assert_eq!(filtering.filters.len(), 1);
    assert_eq!(filtering.sql, " WHERE age IN (2.1, 2.2, 2.3)");
}

#[test]
fn test_filtering_with_in_as_bool() {
    let filtering = Filtering::new(
        &[Ok(FilteringRule {
            filter_column: FilterColumn::BoolList("age", vec![true, false]),
            filter_operator: FilterOperator::In,
            conditional_operator: ConditionalOperator::And,
        })],
        true,
    );
    assert_eq!(filtering.filters.len(), 1);
    assert_eq!(filtering.sql, " WHERE age IN (true, false)");
}

#[test]
fn test_filtering_with_not_in() {
    let filtering = Filtering::new(
        &[Ok(FilteringRule {
            filter_column: FilterColumn::StringList(
                "name",
                vec!["John".to_string(), "Doe".to_string()],
            ),
            filter_operator: FilterOperator::NotIn,
            conditional_operator: ConditionalOperator::And,
        })],
        true,
    );
    assert_eq!(filtering.filters.len(), 1);
    assert_eq!(
        filtering.sql,
        " WHERE LOWER(name) NOT IN (LOWER('John'), LOWER('Doe'))"
    );
}

#[test]
fn test_filtering_with_starts_with() {
    let filtering = Filtering::new(
        &[Ok(FilteringRule {
            filter_column: FilterColumn::String("name", "'John%'".to_string()),
            filter_operator: FilterOperator::StartsWith,
            conditional_operator: ConditionalOperator::And,
        })],
        true,
    );
    assert_eq!(filtering.filters.len(), 1);
    assert_eq!(filtering.sql, " WHERE LOWER(name) LIKE LOWER('John%')");
}

#[test]
fn test_filtering_with_ends_with() {
    let filtering = Filtering::new(
        &[Ok(FilteringRule {
            filter_column: FilterColumn::String("name", "'%John'".to_string()),
            filter_operator: FilterOperator::EndsWith,
            conditional_operator: ConditionalOperator::And,
        })],
        true,
    );
    assert_eq!(filtering.filters.len(), 1);
    assert_eq!(filtering.sql, " WHERE LOWER(name) LIKE LOWER('%John')");
}

#[test]
fn test_filtering_with_multiple_rules_and_different_operators_and_values() {
    let filtering = Filtering::new(
        &[
            Ok(FilteringRule {
                filter_column: FilterColumn::String("name", "'John'".to_string()),
                filter_operator: FilterOperator::Equal,
                conditional_operator: ConditionalOperator::And,
            }),
            Ok(FilteringRule {
                filter_column: FilterColumn::Int("age", 18),
                filter_operator: FilterOperator::GreaterThan,
                conditional_operator: ConditionalOperator::Or,
            }),
            Ok(FilteringRule {
                filter_column: FilterColumn::String("city", "'%New York%'".to_string()),
                filter_operator: FilterOperator::Like,
                conditional_operator: ConditionalOperator::And,
            }),
        ],
        true,
    );
    assert_eq!(filtering.filters.len(), 3);
    assert_eq!(
        filtering.sql,
        " WHERE LOWER(name) = LOWER('John') OR age > 18 AND LOWER(city) LIKE LOWER('%New York%')"
    );
}

#[test]
fn test_filtering_with_multiple_rules_and_different_operators_and_values_swapped() {
    let filtering = Filtering::new(
        &[
            Ok(FilteringRule {
                filter_column: FilterColumn::String("name", "'John'".to_string()),
                filter_operator: FilterOperator::Equal,
                conditional_operator: ConditionalOperator::Or,
            }),
            Ok(FilteringRule {
                filter_column: FilterColumn::Int("age", 18),
                filter_operator: FilterOperator::GreaterThan,
                conditional_operator: ConditionalOperator::And,
            }),
            Ok(FilteringRule {
                filter_column: FilterColumn::String("city", "'%New York%'".to_string()),
                filter_operator: FilterOperator::Like,
                conditional_operator: ConditionalOperator::Or,
            }),
        ],
        true,
    );
    assert_eq!(filtering.filters.len(), 3);
    assert_eq!(
        filtering.sql,
        " WHERE LOWER(name) = LOWER('John') AND age > 18 OR LOWER(city) LIKE LOWER('%New York%')"
    );
}

#[test]
fn test_filtering_with_multiple_rules_and_different_operators_and_values_swapped_and_mixed() {
    let filtering = Filtering::new(
        &[
            Ok(FilteringRule {
                filter_column: FilterColumn::String("name", "'John'".to_string()),
                filter_operator: FilterOperator::Equal,
                conditional_operator: ConditionalOperator::And,
            }),
            Ok(FilteringRule {
                filter_column: FilterColumn::Int("age", 18),
                filter_operator: FilterOperator::GreaterThan,
                conditional_operator: ConditionalOperator::And,
            }),
            Ok(FilteringRule {
                filter_column: FilterColumn::String("city", "'%New York%'".to_string()),
                filter_operator: FilterOperator::Like,
                conditional_operator: ConditionalOperator::Or,
            }),
        ],
        true,
    );
    assert_eq!(filtering.filters.len(), 3);
    assert_eq!(
        filtering.sql,
        " WHERE LOWER(name) = LOWER('John') AND age > 18 OR LOWER(city) LIKE LOWER('%New York%')"
    );
}

#[test]
fn test_filtering_auto_conversions_when_wrong_type_passed_for_non_strings() {
    let filtering = Filtering::new(
        &[
            Ok(FilteringRule {
                filter_column: FilterColumn::String("age", "'18'".to_string()),
                filter_operator: FilterOperator::GreaterThan,
                conditional_operator: ConditionalOperator::And,
            }),
            Ok(FilteringRule {
                filter_column: FilterColumn::String("height", "'20.3'".to_string()),
                filter_operator: FilterOperator::GreaterThanOrEqual,
                conditional_operator: ConditionalOperator::And,
            }),
            Ok(FilteringRule {
                filter_column: FilterColumn::String("age", "'18'".to_string()),
                filter_operator: FilterOperator::LessThan,
                conditional_operator: ConditionalOperator::And,
            }),
            Ok(FilteringRule {
                filter_column: FilterColumn::String("age", "'0.2'".to_string()),
                filter_operator: FilterOperator::LessThanOrEqual,
                conditional_operator: ConditionalOperator::And,
            }),
            Ok(FilteringRule {
                filter_column: FilterColumn::Int("code", 12),
                filter_operator: FilterOperator::Like,
                conditional_operator: ConditionalOperator::Or,
            }),
            Ok(FilteringRule {
                filter_column: FilterColumn::Float("count", 10.4),
                filter_operator: FilterOperator::Like,
                conditional_operator: ConditionalOperator::Or,
            }),
            Ok(FilteringRule {
                filter_column: FilterColumn::Bool("shipped", true),
                filter_operator: FilterOperator::Like,
                conditional_operator: ConditionalOperator::Or,
            }),
            Ok(FilteringRule {
                filter_column: FilterColumn::Int("code2", 12),
                filter_operator: FilterOperator::NotLike,
                conditional_operator: ConditionalOperator::Or,
            }),
            Ok(FilteringRule {
                filter_column: FilterColumn::Float("count2", 10.4),
                filter_operator: FilterOperator::NotLike,
                conditional_operator: ConditionalOperator::Or,
            }),
            Ok(FilteringRule {
                filter_column: FilterColumn::Bool("shipped2", true),
                filter_operator: FilterOperator::NotLike,
                conditional_operator: ConditionalOperator::Or,
            }),
        ],
        true,
    );
    assert_eq!(filtering.filters.len(), 10);
    assert_eq!(
        filtering.sql,
        " WHERE LOWER(age) > LOWER('18') AND LOWER(height) >= LOWER('20.3') AND LOWER(age) < LOWER('18') AND LOWER(age) <= LOWER('0.2') OR LOWER(code) LIKE LOWER('%12%') OR LOWER(count) LIKE LOWER('%10.4%') OR LOWER(shipped) LIKE LOWER('%true%') OR LOWER(code2) NOT LIKE LOWER('%12%') OR LOWER(count2) NOT LIKE LOWER('%10.4%') OR LOWER(shipped2) NOT LIKE LOWER('%true%')"
    );
}

#[test]
fn test_filtering_with_multiple_rules_and_different_operators_and_values_swapped_and_mixed_and_repeated(
) {
    let filtering = Filtering::new(
        &vec![
            Ok(FilteringRule {
                filter_column: FilterColumn::String("name", "'John'".to_string()),
                filter_operator: FilterOperator::Equal,
                conditional_operator: ConditionalOperator::And,
            }),
            Ok(FilteringRule {
                filter_column: FilterColumn::Int("age", 18),
                filter_operator: FilterOperator::GreaterThan,
                conditional_operator: ConditionalOperator::And,
            }),
            Ok(FilteringRule {
                filter_column: FilterColumn::String("city", "'%New York%'".to_string()),
                filter_operator: FilterOperator::Like,
                conditional_operator: ConditionalOperator::Or,
            }),
            Ok(FilteringRule {
                filter_column: FilterColumn::String("name", "'Doe'".to_string()),
                filter_operator: FilterOperator::Equal,
                conditional_operator: ConditionalOperator::Or,
            }),
        ],
        true,
    );
    assert_eq!(filtering.filters.len(), 4);
    assert_eq!(
        filtering.sql,
        " WHERE LOWER(name) = LOWER('John') AND age > 18 OR LOWER(city) LIKE LOWER('%New York%') OR LOWER(name) = LOWER('Doe')"
    );
}

#[test]
fn test_filtering_with_many_rules_and_conditions_with_no_duplicates_with_or_and_and() {
    let filtering = Filtering::new(
        &vec![
            Ok(FilteringRule {
                filter_column: FilterColumn::String("name", "'John'".to_string()),
                filter_operator: FilterOperator::Equal,
                conditional_operator: ConditionalOperator::Or,
            }),
            Ok(FilteringRule {
                filter_column: FilterColumn::Int("age", 18),
                filter_operator: FilterOperator::GreaterThan,
                conditional_operator: ConditionalOperator::And,
            }),
            Ok(FilteringRule {
                filter_column: FilterColumn::String("city", "'%New York%'".to_string()),
                filter_operator: FilterOperator::Like,
                conditional_operator: ConditionalOperator::Or,
            }),
            Ok(FilteringRule {
                filter_column: FilterColumn::String("name", "'Doe'".to_string()),
                filter_operator: FilterOperator::Equal,
                conditional_operator: ConditionalOperator::And,
            }),
        ],
        true,
    );
    assert_eq!(filtering.filters.len(), 4);
    assert_eq!(
        filtering.sql,
        " WHERE LOWER(name) = LOWER('John') AND age > 18 OR LOWER(city) LIKE LOWER('%New York%') AND LOWER(name) = LOWER('Doe')"
    );
}

#[test]
fn test_filtering_options_case_insensitive() {
    let filtering_options = FilteringOptions::new(vec![Ok(FilteringRule {
        filter_column: FilterColumn::String("name", "'John'".to_string()),
        filter_operator: FilterOperator::Equal,
        conditional_operator: ConditionalOperator::Or,
    })]);

    assert_eq!(filtering_options.filtering_rules.len(), 1);
    assert!(filtering_options.case_insensitive);
    assert!(filtering_options
        .filtering()
        .sql
        .contains("WHERE LOWER(name) = LOWER('John')"));
}

#[test]
fn test_filtering_options_case_sensitive() {
    let filtering_options = FilteringOptions::case_sensitive(vec![Ok(FilteringRule {
        filter_column: FilterColumn::String("name", "'John'".to_string()),
        filter_operator: FilterOperator::Equal,
        conditional_operator: ConditionalOperator::Or,
    })]);

    assert_eq!(filtering_options.filtering_rules.len(), 1);
    assert!(!filtering_options.case_insensitive);
    assert!(filtering_options
        .filtering()
        .sql
        .contains("WHERE name = 'John'"));
}
