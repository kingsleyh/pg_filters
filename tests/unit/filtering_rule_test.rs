use pg_filters::filtering::{
    ColumnName, ConditionalOperator, FilterColumn, FilterOperator, FilteringRule,
};

#[test]
fn test_filtering_rule() {
    let filtering_rule =
        FilteringRule::new("and", ColumnName::String("name"), "=", "John").unwrap();

    assert_eq!(
        filtering_rule.filter_column,
        FilterColumn::String("name", "'John'".to_string())
    );
    assert_eq!(filtering_rule.filter_operator, FilterOperator::Equal);
    assert_eq!(
        filtering_rule.conditional_operator,
        ConditionalOperator::And
    );
}

#[test]
fn test_filtering_rule_with_int_value() {
    let filtering_rule = FilteringRule::new("or", ColumnName::Int("age"), ">", "18").unwrap();

    assert_eq!(filtering_rule.filter_column, FilterColumn::Int("age", 18));
    assert_eq!(filtering_rule.filter_operator, FilterOperator::GreaterThan);
    assert_eq!(filtering_rule.conditional_operator, ConditionalOperator::Or);
}

#[test]
fn test_filtering_rule_with_float_value() {
    let filtering_rule =
        FilteringRule::new("and", ColumnName::Float("height"), ">=", "5.5").unwrap();

    assert_eq!(
        filtering_rule.filter_column,
        FilterColumn::Float("height", 5.5)
    );
    assert_eq!(
        filtering_rule.filter_operator,
        FilterOperator::GreaterThanOrEqual
    );
    assert_eq!(
        filtering_rule.conditional_operator,
        ConditionalOperator::And
    );
}

#[test]
fn test_filtering_rule_with_bool_value() {
    let filtering_rule =
        FilteringRule::new("or", ColumnName::Bool("is_active"), "=", "true").unwrap();

    assert_eq!(
        filtering_rule.filter_column,
        FilterColumn::Bool("is_active", true)
    );
    assert_eq!(filtering_rule.filter_operator, FilterOperator::Equal);
    assert_eq!(filtering_rule.conditional_operator, ConditionalOperator::Or);
}

#[test]
fn test_filtering_rule_with_null_value() {
    let filtering_rule =
        FilteringRule::new("and", ColumnName::String("email"), "is null", "").unwrap();

    assert_eq!(
        filtering_rule.filter_column,
        FilterColumn::String("email", "".into())
    );
    assert_eq!(filtering_rule.filter_operator, FilterOperator::IsNull);
    assert_eq!(
        filtering_rule.conditional_operator,
        ConditionalOperator::And
    );
}

#[test]
fn test_filtering_rule_with_not_null_value() {
    let filtering_rule =
        FilteringRule::new("or", ColumnName::String("email"), "is not null", "").unwrap();

    assert_eq!(
        filtering_rule.filter_column,
        FilterColumn::String("email", "".into())
    );
    assert_eq!(filtering_rule.filter_operator, FilterOperator::IsNotNull);
    assert_eq!(filtering_rule.conditional_operator, ConditionalOperator::Or);
}

#[test]
fn test_filtering_rule_with_like_value() {
    let filtering_rule =
        FilteringRule::new("and", ColumnName::String("name"), "like", "John").unwrap();

    assert_eq!(
        filtering_rule.filter_column,
        FilterColumn::String("name", "'%John%'".into())
    );
    assert_eq!(filtering_rule.filter_operator, FilterOperator::Like);
    assert_eq!(
        filtering_rule.conditional_operator,
        ConditionalOperator::And
    );
}

#[test]
fn test_filtering_rule_with_not_like_value() {
    let filtering_rule =
        FilteringRule::new("or", ColumnName::String("name"), "not like", "John").unwrap();

    assert_eq!(
        filtering_rule.filter_column,
        FilterColumn::String("name", "'%John%'".into())
    );
    assert_eq!(filtering_rule.filter_operator, FilterOperator::NotLike);
    assert_eq!(filtering_rule.conditional_operator, ConditionalOperator::Or);
}

#[test]
fn test_filtering_rule_with_string_in_value() {
    let filtering_rule =
        FilteringRule::new("and", ColumnName::String("name"), "in", "John,Jane").unwrap();

    assert_eq!(
        filtering_rule.filter_column,
        FilterColumn::StringList("name", vec!["'John'".into(), "'Jane'".into()])
    );
    assert_eq!(filtering_rule.filter_operator, FilterOperator::In);
    assert_eq!(
        filtering_rule.conditional_operator,
        ConditionalOperator::And
    );
}

#[test]
fn test_filtering_rule_with_int_in_value() {
    let filtering_rule = FilteringRule::new("and", ColumnName::Int("name"), "in", "1,2,3").unwrap();

    assert_eq!(
        filtering_rule.filter_column,
        FilterColumn::IntList("name", vec![1, 2, 3])
    );
    assert_eq!(filtering_rule.filter_operator, FilterOperator::In);
    assert_eq!(
        filtering_rule.conditional_operator,
        ConditionalOperator::And
    );
}

#[test]
fn test_filtering_rule_with_float_in_value() {
    let filtering_rule =
        FilteringRule::new("and", ColumnName::Float("name"), "in", "1.1,2.2,3.0").unwrap();

    assert_eq!(
        filtering_rule.filter_column,
        FilterColumn::FloatList("name", vec![1.1, 2.2, 3.0])
    );
    assert_eq!(filtering_rule.filter_operator, FilterOperator::In);
    assert_eq!(
        filtering_rule.conditional_operator,
        ConditionalOperator::And
    );
}

#[test]
fn test_filtering_rule_with_bool_in_value() {
    let filtering_rule =
        FilteringRule::new("and", ColumnName::Bool("name"), "in", "true,false").unwrap();

    assert_eq!(
        filtering_rule.filter_column,
        FilterColumn::BoolList("name", vec![true, false])
    );
    assert_eq!(filtering_rule.filter_operator, FilterOperator::In);
    assert_eq!(
        filtering_rule.conditional_operator,
        ConditionalOperator::And
    );
}

#[test]
fn test_filtering_rule_with_string_not_in_value() {
    let filtering_rule =
        FilteringRule::new("and", ColumnName::String("name"), "not in", "John,Jane").unwrap();

    assert_eq!(
        filtering_rule.filter_column,
        FilterColumn::StringList("name", vec!["'John'".into(), "'Jane'".into()])
    );
    assert_eq!(filtering_rule.filter_operator, FilterOperator::NotIn);
    assert_eq!(
        filtering_rule.conditional_operator,
        ConditionalOperator::And
    );
}

#[test]
fn test_filtering_rule_with_int_not_in_value() {
    let filtering_rule =
        FilteringRule::new("and", ColumnName::Int("name"), "not in", "1,2,3").unwrap();

    assert_eq!(
        filtering_rule.filter_column,
        FilterColumn::IntList("name", vec![1, 2, 3])
    );
    assert_eq!(filtering_rule.filter_operator, FilterOperator::NotIn);
    assert_eq!(
        filtering_rule.conditional_operator,
        ConditionalOperator::And
    );
}

#[test]
fn test_filtering_rule_with_float_not_in_value() {
    let filtering_rule =
        FilteringRule::new("and", ColumnName::Float("name"), "not in", "1.1,2.2,3.0").unwrap();

    assert_eq!(
        filtering_rule.filter_column,
        FilterColumn::FloatList("name", vec![1.1, 2.2, 3.0])
    );
    assert_eq!(filtering_rule.filter_operator, FilterOperator::NotIn);
    assert_eq!(
        filtering_rule.conditional_operator,
        ConditionalOperator::And
    );
}

#[test]
fn test_filtering_rule_with_bool_not_in_value() {
    let filtering_rule =
        FilteringRule::new("and", ColumnName::Bool("name"), "not in", "true,false").unwrap();

    assert_eq!(
        filtering_rule.filter_column,
        FilterColumn::BoolList("name", vec![true, false])
    );
    assert_eq!(filtering_rule.filter_operator, FilterOperator::NotIn);
    assert_eq!(
        filtering_rule.conditional_operator,
        ConditionalOperator::And
    );
}

#[test]
fn test_filtering_rule_with_invalid_filter_operator() {
    let filtering_rule =
        FilteringRule::new("and", ColumnName::String("name"), "}", "John").unwrap();

    assert_eq!(
        filtering_rule.filter_column,
        FilterColumn::String("name", "'John'".into())
    );
    assert_eq!(filtering_rule.filter_operator, FilterOperator::Equal);
    assert_eq!(
        filtering_rule.conditional_operator,
        ConditionalOperator::And
    );
}

#[test]
fn test_filtering_rule_with_invalid_conditional_operator() {
    let filtering_rule = FilteringRule::new("}", ColumnName::String("name"), "=", "John").unwrap();

    assert_eq!(
        filtering_rule.filter_column,
        FilterColumn::String("name", "'John'".into())
    );
    assert_eq!(filtering_rule.filter_operator, FilterOperator::Equal);
    assert_eq!(
        filtering_rule.conditional_operator,
        ConditionalOperator::And
    );
}

#[test]
fn test_filtering_rule_with_starts_with_value() {
    let filtering_rule =
        FilteringRule::new("and", ColumnName::String("name"), "starts with", "John").unwrap();

    assert_eq!(
        filtering_rule.filter_column,
        FilterColumn::String("name", "'John%'".into())
    );
    assert_eq!(filtering_rule.filter_operator, FilterOperator::StartsWith);
    assert_eq!(
        filtering_rule.conditional_operator,
        ConditionalOperator::And
    );
}

#[test]
fn test_filtering_rule_with_ends_with_value() {
    let filtering_rule =
        FilteringRule::new("or", ColumnName::String("name"), "ends with", "John").unwrap();

    assert_eq!(
        filtering_rule.filter_column,
        FilterColumn::String("name", "'%John'".into())
    );
    assert_eq!(filtering_rule.filter_operator, FilterOperator::EndsWith);
    assert_eq!(filtering_rule.conditional_operator, ConditionalOperator::Or);
}
