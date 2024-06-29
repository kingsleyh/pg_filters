use pg_filters::pagination::Paginate;

#[test]
fn test_pagination_with_negative_current_page() {
    let paginate = Paginate::new(-1, 10, 100, 1000);
    assert_eq!(paginate.pagination.current_page, 1);
    assert_eq!(paginate.pagination.previous_page, 1);
    assert_eq!(paginate.pagination.next_page, 2);
    assert_eq!(paginate.pagination.total_pages, 100);
    assert_eq!(paginate.pagination.per_page, 10);
    assert_eq!(paginate.pagination.total_records, 1000);
    assert_eq!(paginate.sql, "LIMIT 10 OFFSET 0");
}

#[test]
fn test_pagination_with_current_page_greater_than_total_pages() {
    let paginate = Paginate::new(101, 10, 100, 1000);
    assert_eq!(paginate.pagination.current_page, 100);
    assert_eq!(paginate.pagination.previous_page, 99);
    assert_eq!(paginate.pagination.next_page, 100);
    assert_eq!(paginate.pagination.total_pages, 100);
    assert_eq!(paginate.pagination.per_page, 10);
    assert_eq!(paginate.pagination.total_records, 1000);
    assert_eq!(paginate.sql, "LIMIT 10 OFFSET 990");
}

#[test]
fn test_pagination_with_per_page_greater_than_per_page_limit() {
    let paginate = Paginate::new(1, 100, 10, 1000);
    assert_eq!(paginate.pagination.current_page, 1);
    assert_eq!(paginate.pagination.previous_page, 1);
    assert_eq!(paginate.pagination.next_page, 2);
    assert_eq!(paginate.pagination.total_pages, 100);
    assert_eq!(paginate.pagination.per_page, 10);
    assert_eq!(paginate.pagination.total_records, 1000);
    assert_eq!(paginate.sql, "LIMIT 10 OFFSET 0");
}

#[test]
fn test_pagination_with_per_page_equal_to_per_page_limit() {
    let paginate = Paginate::new(1, 10, 10, 1000);
    assert_eq!(paginate.pagination.current_page, 1);
    assert_eq!(paginate.pagination.previous_page, 1);
    assert_eq!(paginate.pagination.next_page, 2);
    assert_eq!(paginate.pagination.total_pages, 100);
    assert_eq!(paginate.pagination.per_page, 10);
    assert_eq!(paginate.pagination.total_records, 1000);
    assert_eq!(paginate.sql, "LIMIT 10 OFFSET 0");
}

#[test]
fn test_pagination_with_per_page_less_than_per_page_limit() {
    let paginate = Paginate::new(1, 5, 10, 1000);
    assert_eq!(paginate.pagination.current_page, 1);
    assert_eq!(paginate.pagination.previous_page, 1);
    assert_eq!(paginate.pagination.next_page, 2);
    assert_eq!(paginate.pagination.total_pages, 200);
    assert_eq!(paginate.pagination.per_page, 5);
    assert_eq!(paginate.pagination.total_records, 1000);
    assert_eq!(paginate.sql, "LIMIT 5 OFFSET 0");
}

#[test]
fn test_pagination_with_per_page_equal_to_total_records() {
    let paginate = Paginate::new(1, 1000, 1000, 1000);
    assert_eq!(paginate.pagination.current_page, 1);
    assert_eq!(paginate.pagination.previous_page, 1);
    assert_eq!(paginate.pagination.next_page, 1);
    assert_eq!(paginate.pagination.total_pages, 1);
    assert_eq!(paginate.pagination.per_page, 1000);
    assert_eq!(paginate.pagination.total_records, 1000);
    assert_eq!(paginate.sql, "LIMIT 1000 OFFSET 0");
}

#[test]
fn test_pagination_with_per_page_greater_than_total_records() {
    let paginate = Paginate::new(1, 1001, 1000, 1000);
    assert_eq!(paginate.pagination.current_page, 1);
    assert_eq!(paginate.pagination.previous_page, 1);
    assert_eq!(paginate.pagination.next_page, 1);
    assert_eq!(paginate.pagination.total_pages, 1);
    assert_eq!(paginate.pagination.per_page, 1000);
    assert_eq!(paginate.pagination.total_records, 1000);
    assert_eq!(paginate.sql, "LIMIT 1000 OFFSET 0");
}

#[test]
fn test_pagination_with_per_page_less_than_total_records() {
    let paginate = Paginate::new(1, 500, 1000, 1000);
    assert_eq!(paginate.pagination.current_page, 1);
    assert_eq!(paginate.pagination.previous_page, 1);
    assert_eq!(paginate.pagination.next_page, 2);
    assert_eq!(paginate.pagination.total_pages, 2);
    assert_eq!(paginate.pagination.per_page, 500);
    assert_eq!(paginate.pagination.total_records, 1000);
    assert_eq!(paginate.sql, "LIMIT 500 OFFSET 0");
}
