use pg_filters::filtering::{ConditionalOperator, FilterOperator, FilterValue, FilteringRule};

#[test]
fn test_filtering_rule() {
    let filtering_rule = FilteringRule::new("name".into(), "=".into(), "and".into(), "John".into());

    assert_eq!(filtering_rule.column, "name");
    assert_eq!(filtering_rule.filter_operator, FilterOperator::Equal);
    assert_eq!(
        filtering_rule.conditional_operator,
        ConditionalOperator::And
    );
    assert_eq!(filtering_rule.value, FilterValue::String("John".into()));
}

#[test]
fn test_filtering_rule_with_int_value() {
    let filtering_rule = FilteringRule::new("age".into(), ">".into(), "or".into(), "18".into());

    assert_eq!(filtering_rule.column, "age");
    assert_eq!(filtering_rule.filter_operator, FilterOperator::GreaterThan);
    assert_eq!(filtering_rule.conditional_operator, ConditionalOperator::Or);
    assert_eq!(filtering_rule.value, FilterValue::Int(18));
}

#[test]
fn test_filtering_rule_with_float_value() {
    let filtering_rule =
        FilteringRule::new("height".into(), ">=".into(), "and".into(), "5.5".into());

    assert_eq!(filtering_rule.column, "height");
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
        FilteringRule::new("is_active".into(), "=".into(), "or".into(), "true".into());

    assert_eq!(filtering_rule.column, "is_active");
    assert_eq!(filtering_rule.filter_operator, FilterOperator::Equal);
    assert_eq!(filtering_rule.conditional_operator, ConditionalOperator::Or);
    assert_eq!(filtering_rule.value, FilterValue::Bool(true));
}

#[test]
fn test_filtering_rule_with_null_value() {
    let filtering_rule =
        FilteringRule::new("email".into(), "is null".into(), "and".into(), "".into());

    assert_eq!(filtering_rule.column, "email");
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
        FilteringRule::new("email".into(), "is not null".into(), "or".into(), "".into());

    assert_eq!(filtering_rule.column, "email");
    assert_eq!(filtering_rule.filter_operator, FilterOperator::IsNotNull);
    assert_eq!(filtering_rule.conditional_operator, ConditionalOperator::Or);
    assert_eq!(filtering_rule.value, FilterValue::String("".into()));
}

#[test]
fn test_filtering_rule_with_like_value() {
    let filtering_rule =
        FilteringRule::new("name".into(), "like".into(), "and".into(), "John".into());

    assert_eq!(filtering_rule.column, "name");
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
        FilteringRule::new("name".into(), "not like".into(), "or".into(), "John".into());

    assert_eq!(filtering_rule.column, "name");
    assert_eq!(filtering_rule.filter_operator, FilterOperator::NotLike);
    assert_eq!(filtering_rule.conditional_operator, ConditionalOperator::Or);
    assert_eq!(filtering_rule.value, FilterValue::String("John".into()));
}

#[test]
fn test_filtering_rule_with_in_value() {
    let filtering_rule = FilteringRule::new(
        "name".into(),
        "in".into(),
        "and".into(),
        "('John', 'Jane')".into(),
    );

    assert_eq!(filtering_rule.column, "name");
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
        "name".into(),
        "not in".into(),
        "or".into(),
        "('John', 'Jane')".into(),
    );

    assert_eq!(filtering_rule.column, "name");
    assert_eq!(filtering_rule.filter_operator, FilterOperator::NotIn);
    assert_eq!(filtering_rule.conditional_operator, ConditionalOperator::Or);
    assert_eq!(
        filtering_rule.value,
        FilterValue::String("('John', 'Jane')".into())
    );
}

#[test]
fn test_filtering_rule_with_invalid_filter_operator() {
    let filtering_rule = FilteringRule::new("name".into(), "}".into(), "and".into(), "John".into());

    assert_eq!(filtering_rule.column, "name");
    assert_eq!(filtering_rule.filter_operator, FilterOperator::Equal);
    assert_eq!(
        filtering_rule.conditional_operator,
        ConditionalOperator::And
    );
    assert_eq!(filtering_rule.value, FilterValue::String("John".into()));
}

#[test]
fn test_filtering_rule_with_invalid_conditional_operator() {
    let filtering_rule = FilteringRule::new("name".into(), "=".into(), "}".into(), "John".into());

    assert_eq!(filtering_rule.column, "name");
    assert_eq!(filtering_rule.filter_operator, FilterOperator::Equal);
    assert_eq!(
        filtering_rule.conditional_operator,
        ConditionalOperator::And
    );
    assert_eq!(filtering_rule.value, FilterValue::String("John".into()));
}
