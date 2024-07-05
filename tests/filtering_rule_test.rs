use pg_filters::filtering::{ColumnName, ConditionalOperator, FilterOperator, FilterValue, FilteringRule};

#[test]
fn test_filtering_rule() {
    let filtering_rule = FilteringRule::new("and".into(), ColumnName::String("name"), "=".into(), "John".into());

    assert_eq!(filtering_rule.column, ColumnName::String("name"),);
    assert_eq!(filtering_rule.filter_operator, FilterOperator::Equal);
    assert_eq!(
        filtering_rule.conditional_operator,
        ConditionalOperator::And
    );
    assert_eq!(filtering_rule.value, FilterValue::String("John".into()));
}

#[test]
fn test_filtering_rule_with_int_value() {
    let filtering_rule = FilteringRule::new("or".into(), ColumnName::Int("age"), ">".into(), "18".into());

    assert_eq!(filtering_rule.column, ColumnName::Int("age"));
    assert_eq!(filtering_rule.filter_operator, FilterOperator::GreaterThan);
    assert_eq!(filtering_rule.conditional_operator, ConditionalOperator::Or);
    assert_eq!(filtering_rule.value, FilterValue::Int(18));
}

#[test]
fn test_filtering_rule_with_float_value() {
    let filtering_rule =
        FilteringRule::new("and".into(), ColumnName::Float("height"), ">=".into(), "5.5".into());

    assert_eq!(filtering_rule.column, ColumnName::Float("height"));
    assert_eq!(
        filtering_rule.filter_operator,
        FilterOperator::GreaterThanOrEqual
    );
    assert_eq!(
        filtering_rule.conditional_operator,
        ConditionalOperator::And
    );
    assert_eq!(filtering_rule.value, FilterValue::Float(5.5));
}

#[test]
fn test_filtering_rule_with_bool_value() {
    let filtering_rule =
        FilteringRule::new("or".into(), ColumnName::Bool("is_active"), "=".into(), "true".into());

    assert_eq!(filtering_rule.column, ColumnName::Bool("is_active"));
    assert_eq!(filtering_rule.filter_operator, FilterOperator::Equal);
    assert_eq!(filtering_rule.conditional_operator, ConditionalOperator::Or);
    assert_eq!(filtering_rule.value, FilterValue::Bool(true));
}

#[test]
fn test_filtering_rule_with_null_value() {
    let filtering_rule =
        FilteringRule::new("and".into(), ColumnName::String("email"), "is null".into(), "".into());

    assert_eq!(filtering_rule.column, ColumnName::String("email"));
    assert_eq!(filtering_rule.filter_operator, FilterOperator::IsNull);
    assert_eq!(
        filtering_rule.conditional_operator,
        ConditionalOperator::And
    );
    assert_eq!(filtering_rule.value, FilterValue::String("".into()));
}

#[test]
fn test_filtering_rule_with_not_null_value() {
    let filtering_rule =
        FilteringRule::new("or".into(), ColumnName::String("email"), "is not null".into(), "".into());

    assert_eq!(filtering_rule.column, ColumnName::String("email"));
    assert_eq!(filtering_rule.filter_operator, FilterOperator::IsNotNull);
    assert_eq!(filtering_rule.conditional_operator, ConditionalOperator::Or);
    assert_eq!(filtering_rule.value, FilterValue::String("".into()));
}

#[test]
fn test_filtering_rule_with_like_value() {
    let filtering_rule =
        FilteringRule::new("and".into(), ColumnName::String("name"), "like".into(), "John".into());

    assert_eq!(filtering_rule.column, ColumnName::String("name"));
    assert_eq!(filtering_rule.filter_operator, FilterOperator::Like);
    assert_eq!(
        filtering_rule.conditional_operator,
        ConditionalOperator::And
    );
    assert_eq!(filtering_rule.value, FilterValue::String("John".into()));
}

#[test]
fn test_filtering_rule_with_not_like_value() {
    let filtering_rule =
        FilteringRule::new("or".into(), ColumnName::String("name"), "not like".into(), "John".into());

    assert_eq!(filtering_rule.column, ColumnName::String("name"));
    assert_eq!(filtering_rule.filter_operator, FilterOperator::NotLike);
    assert_eq!(filtering_rule.conditional_operator, ConditionalOperator::Or);
    assert_eq!(filtering_rule.value, FilterValue::String("John".into()));
}

#[test]
fn test_filtering_rule_with_in_value() {
    let filtering_rule = FilteringRule::new(
        "and".into(),
        ColumnName::String("name"),
        "in".into(),
        "('John', 'Jane')".into(),
    );

    assert_eq!(filtering_rule.column, ColumnName::String("name"));
    assert_eq!(filtering_rule.filter_operator, FilterOperator::In);
    assert_eq!(
        filtering_rule.conditional_operator,
        ConditionalOperator::And
    );
    assert_eq!(
        filtering_rule.value,
        FilterValue::String("('John', 'Jane')".into())
    );
}

#[test]
fn test_filtering_rule_with_not_in_value() {
    let filtering_rule = FilteringRule::new(
        "or".into(),
        ColumnName::String("name"),
        "not in".into(),
        "('John', 'Jane')".into(),
    );

    assert_eq!(filtering_rule.column, ColumnName::String("name"));
    assert_eq!(filtering_rule.filter_operator, FilterOperator::NotIn);
    assert_eq!(filtering_rule.conditional_operator, ConditionalOperator::Or);
    assert_eq!(
        filtering_rule.value,
        FilterValue::String("('John', 'Jane')".into())
    );
}

#[test]
fn test_filtering_rule_with_invalid_filter_operator() {
    let filtering_rule = FilteringRule::new("and".into(), ColumnName::String("name"), "}".into(), "John".into());

    assert_eq!(filtering_rule.column, ColumnName::String("name"));
    assert_eq!(filtering_rule.filter_operator, FilterOperator::Equal);
    assert_eq!(
        filtering_rule.conditional_operator,
        ConditionalOperator::And
    );
    assert_eq!(filtering_rule.value, FilterValue::String("John".into()));
}

#[test]
fn test_filtering_rule_with_invalid_conditional_operator() {
    let filtering_rule = FilteringRule::new("}".into(), ColumnName::String("name"), "=".into(), "John".into());

    assert_eq!(filtering_rule.column, ColumnName::String("name"));
    assert_eq!(filtering_rule.filter_operator, FilterOperator::Equal);
    assert_eq!(
        filtering_rule.conditional_operator,
        ConditionalOperator::And
    );
    assert_eq!(filtering_rule.value, FilterValue::String("John".into()));
}

#[test]
fn test_filtering_rule_with_starts_with_value() {
    let filtering_rule = FilteringRule::new(
        "and".into(),
        ColumnName::String("name"),
        "starts with".into(),
        "John".into(),
    );

    assert_eq!(filtering_rule.column, ColumnName::String("name"));
    assert_eq!(filtering_rule.filter_operator, FilterOperator::StartsWith);
    assert_eq!(
        filtering_rule.conditional_operator,
        ConditionalOperator::And
    );
    assert_eq!(filtering_rule.value, FilterValue::String("John".into()));
}

#[test]
fn test_filtering_rule_with_ends_with_value() {
    let filtering_rule = FilteringRule::new(
        "or".into(),
        ColumnName::String("name"),
        "ends with".into(),
        "John".into(),
    );

    assert_eq!(filtering_rule.column,ColumnName::String("name"));
    assert_eq!(filtering_rule.filter_operator, FilterOperator::EndsWith);
    assert_eq!(filtering_rule.conditional_operator, ConditionalOperator::Or);
    assert_eq!(filtering_rule.value, FilterValue::String("John".into()));
}
