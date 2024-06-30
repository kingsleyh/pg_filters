use pg_filters::sorting::{SortOrder, SortedColumn, Sorting};

#[test]
fn test_sorting() {
    let sorting = Sorting::new(vec![
        SortedColumn {
            column: "name".to_string(),
            order: SortOrder::Asc,
        },
        SortedColumn {
            column: "age".to_string(),
            order: SortOrder::Desc,
        },
    ]);
    assert_eq!(sorting.columns.len(), 2);
    assert_eq!(sorting.sql, " ORDER BY age DESC, name ASC");
}

#[test]
fn test_sorting_with_duplicate_columns() {
    let sorting = Sorting::new(vec![
        SortedColumn {
            column: "name".to_string(),
            order: SortOrder::Asc,
        },
        SortedColumn {
            column: "name".to_string(),
            order: SortOrder::Desc,
        },
    ]);
    assert_eq!(sorting.columns.len(), 1);
    assert_eq!(sorting.sql, " ORDER BY name ASC");
}

#[test]
fn test_sorting_with_single_column() {
    let sorting = Sorting::new(vec![SortedColumn {
        column: "name".to_string(),
        order: SortOrder::Asc,
    }]);
    assert_eq!(sorting.columns.len(), 1);
    assert_eq!(sorting.sql, " ORDER BY name ASC");
}

#[test]
fn test_sorting_with_empty_columns() {
    let sorting = Sorting::new(vec![]);
    assert_eq!(sorting.columns.len(), 0);
    assert_eq!(sorting.sql, "");
}

#[test]
fn test_sorting_with_multiple_columns() {
    let sorting = Sorting::new(vec![
        SortedColumn {
            column: "name".to_string(),
            order: SortOrder::Asc,
        },
        SortedColumn {
            column: "age".to_string(),
            order: SortOrder::Desc,
        },
        SortedColumn {
            column: "created_at".to_string(),
            order: SortOrder::Asc,
        },
    ]);
    assert_eq!(sorting.columns.len(), 3);
    assert_eq!(sorting.sql, " ORDER BY age DESC, created_at ASC, name ASC");
}

#[test]
fn test_sorting_with_single_column_desc() {
    let sorting = Sorting::new(vec![SortedColumn {
        column: "name".to_string(),
        order: SortOrder::Desc,
    }]);
    assert_eq!(sorting.columns.len(), 1);
    assert_eq!(sorting.sql, " ORDER BY name DESC");
}

#[test]
fn test_sorting_with_single_column_asc() {
    let sorting = Sorting::new(vec![SortedColumn {
        column: "name".to_string(),
        order: SortOrder::Asc,
    }]);
    assert_eq!(sorting.columns.len(), 1);
    assert_eq!(sorting.sql, " ORDER BY name ASC");
}

#[test]
fn test_sorting_with_multiple_columns_desc() {
    let sorting = Sorting::new(vec![
        SortedColumn {
            column: "name".to_string(),
            order: SortOrder::Desc,
        },
        SortedColumn {
            column: "age".to_string(),
            order: SortOrder::Desc,
        },
        SortedColumn {
            column: "created_at".to_string(),
            order: SortOrder::Desc,
        },
    ]);
    assert_eq!(sorting.columns.len(), 3);
    assert_eq!(
        sorting.sql,
        " ORDER BY age DESC, created_at DESC, name DESC"
    );
}

#[test]
fn test_sorting_with_multiple_columns_asc() {
    let sorting = Sorting::new(vec![
        SortedColumn {
            column: "name".to_string(),
            order: SortOrder::Asc,
        },
        SortedColumn {
            column: "age".to_string(),
            order: SortOrder::Asc,
        },
        SortedColumn {
            column: "created_at".to_string(),
            order: SortOrder::Asc,
        },
    ]);
    assert_eq!(sorting.columns.len(), 3);
    assert_eq!(sorting.sql, " ORDER BY age ASC, created_at ASC, name ASC");
}

#[test]
fn test_sorting_with_multiple_columns_desc_and_asc() {
    let sorting = Sorting::new(vec![
        SortedColumn {
            column: "name".to_string(),
            order: SortOrder::Desc,
        },
        SortedColumn {
            column: "age".to_string(),
            order: SortOrder::Asc,
        },
    ]);
    assert_eq!(sorting.columns.len(), 2);
    assert_eq!(sorting.sql, " ORDER BY age ASC, name DESC");
}

#[test]
fn test_sorted_column_new_asc() {
    let sorted_column = SortedColumn::new("name", "asc".to_string());
    assert_eq!(sorted_column.column, "name");
    assert_eq!(sorted_column.order, SortOrder::Asc);
}

#[test]
fn test_sorted_column_new_desc() {
    let sorted_column = SortedColumn::new("name", "desc".to_string());
    assert_eq!(sorted_column.column, "name");
    assert_eq!(sorted_column.order, SortOrder::Desc);
}

#[test]
fn test_sorted_column_new_invalid_order_defaults_to_asc() {
    let sorted_column = SortedColumn::new("name", "invalid".to_string());
    assert_eq!(sorted_column.column, "name");
    assert_eq!(sorted_column.order, SortOrder::Asc);
}
