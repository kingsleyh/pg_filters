use pg_filters::filtering::{
    ColumnName, ConditionalOperator, FilterColumn, FilterOperator, FilteringRule,
};

#[test]
fn test_filtering_rule() {
    let filtering_rule = FilteringRule::new(
        "and".into(),
        ColumnName::String("name"),
        "=".into(),
        "John".into(),
    )
    .unwrap();

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
    let filtering_rule =
        FilteringRule::new("or".into(), ColumnName::Int("age"), ">".into(), "18".into()).unwrap();

    assert_eq!(filtering_rule.filter_column, FilterColumn::Int("age", 18));
    assert_eq!(filtering_rule.filter_operator, FilterOperator::GreaterThan);
    assert_eq!(filtering_rule.conditional_operator, ConditionalOperator::Or);
}

#[test]
fn test_filtering_rule_with_float_value() {
    let filtering_rule = FilteringRule::new(
        "and".into(),
        ColumnName::Float("height"),
        ">=".into(),
        "5.5".into(),
    )
    .unwrap();

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
    let filtering_rule = FilteringRule::new(
        "or".into(),
        ColumnName::Bool("is_active"),
        "=".into(),
        "true".into(),
    )
    .unwrap();

    assert_eq!(
        filtering_rule.filter_column,
        FilterColumn::Bool("is_active", true)
    );
    assert_eq!(filtering_rule.filter_operator, FilterOperator::Equal);
    assert_eq!(filtering_rule.conditional_operator, ConditionalOperator::Or);
}

#[test]
fn test_filtering_rule_with_null_value() {
    let filtering_rule = FilteringRule::new(
        "and".into(),
        ColumnName::String("email"),
        "is null".into(),
        "".into(),
    )
    .unwrap();

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
    let filtering_rule = FilteringRule::new(
        "or".into(),
        ColumnName::String("email"),
        "is not null".into(),
        "".into(),
    )
    .unwrap();

    assert_eq!(
        filtering_rule.filter_column,
        FilterColumn::String("email", "".into())
    );
    assert_eq!(filtering_rule.filter_operator, FilterOperator::IsNotNull);
    assert_eq!(filtering_rule.conditional_operator, ConditionalOperator::Or);
}

#[test]
fn test_filtering_rule_with_like_value() {
    let filtering_rule = FilteringRule::new(
        "and".into(),
        ColumnName::String("name"),
        "like".into(),
        "John".into(),
    )
    .unwrap();

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
    let filtering_rule = FilteringRule::new(
        "or".into(),
        ColumnName::String("name"),
        "not like".into(),
        "John".into(),
    )
    .unwrap();

    assert_eq!(
        filtering_rule.filter_column,
        FilterColumn::String("name", "'%John%'".into())
    );
    assert_eq!(filtering_rule.filter_operator, FilterOperator::NotLike);
    assert_eq!(filtering_rule.conditional_operator, ConditionalOperator::Or);
}

#[test]
fn test_filtering_rule_with_string_in_value() {
    let filtering_rule = FilteringRule::new(
        "and".into(),
        ColumnName::String("name"),
        "in".into(),
        "John,Jane".into(),
    )
    .unwrap();

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
    let filtering_rule = FilteringRule::new(
        "and".into(),
        ColumnName::Int("name"),
        "in".into(),
        "1,2,3".into(),
    )
    .unwrap();

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
    let filtering_rule = FilteringRule::new(
        "and".into(),
        ColumnName::Float("name"),
        "in".into(),
        "1.1,2.2,3.0".into(),
    )
    .unwrap();

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
    let filtering_rule = FilteringRule::new(
        "and".into(),
        ColumnName::Bool("name"),
        "in".into(),
        "true,false".into(),
    )
    .unwrap();

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
    let filtering_rule = FilteringRule::new(
        "and".into(),
        ColumnName::String("name"),
        "not in".into(),
        "John,Jane".into(),
    )
    .unwrap();

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
    let filtering_rule = FilteringRule::new(
        "and".into(),
        ColumnName::Int("name"),
        "not in".into(),
        "1,2,3".into(),
    )
    .unwrap();

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
    let filtering_rule = FilteringRule::new(
        "and".into(),
        ColumnName::Float("name"),
        "not in".into(),
        "1.1,2.2,3.0".into(),
    )
    .unwrap();

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
    let filtering_rule = FilteringRule::new(
        "and".into(),
        ColumnName::Bool("name"),
        "not in".into(),
        "true,false".into(),
    )
    .unwrap();

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
    let filtering_rule = FilteringRule::new(
        "and".into(),
        ColumnName::String("name"),
        "}".into(),
        "John".into(),
    )
    .unwrap();

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
    let filtering_rule = FilteringRule::new(
        "}".into(),
        ColumnName::String("name"),
        "=".into(),
        "John".into(),
    )
    .unwrap();

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
    let filtering_rule = FilteringRule::new(
        "and".into(),
        ColumnName::String("name"),
        "starts with".into(),
        "John".into(),
    )
    .unwrap();

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
    let filtering_rule = FilteringRule::new(
        "or".into(),
        ColumnName::String("name"),
        "ends with".into(),
        "John".into(),
    )
    .unwrap();

    assert_eq!(
        filtering_rule.filter_column,
        FilterColumn::String("name", "'%John'".into())
    );
    assert_eq!(filtering_rule.filter_operator, FilterOperator::EndsWith);
    assert_eq!(filtering_rule.conditional_operator, ConditionalOperator::Or);
}
